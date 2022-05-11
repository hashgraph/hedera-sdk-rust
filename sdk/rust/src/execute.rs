use async_trait::async_trait;
use backoff::backoff::Backoff;
use backoff::ExponentialBackoff;
use hedera_proto::services::ResponseCodeEnum;
use prost::Message;
use rand::seq::SliceRandom;
use rand::thread_rng;
use tokio::time::sleep;
use tonic::transport::Channel;

use crate::{AccountId, Client, Error, TransactionId};

#[async_trait]
pub(crate) trait Execute {
    type GrpcRequest: Clone + Message;

    type GrpcResponse: Message;

    /// Additional context given at the start of the execution, passed by
    /// reference into each call to `make_request`.
    type RequestContext;

    /// Additional context returned from each call to `make_request`. Upon
    /// a successful request, the associated response context is passed to
    /// `make_response`.
    type ResponseContext;

    type Response;

    /// Get the _explicit_ nodes that this request will be submitted to.
    fn node_account_ids(&self) -> Option<&[AccountId]>;

    /// Get the _explicit_ transaction ID that this request will use.
    fn transaction_id(&self) -> Option<TransactionId>;

    /// Get whether to generate transaction IDs for request creation.
    fn requires_transaction_id() -> bool;

    /// Create a new request for execution.
    ///
    /// A created request is cached per node until any request returns
    /// `TransactionExpired`; in which case, the request cache is cleared.
    ///
    async fn make_request(
        &self,
        client: &Client,
        transaction_id: Option<TransactionId>,
        node_account_id: AccountId,
        context: &Self::RequestContext,
    ) -> crate::Result<(Self::GrpcRequest, Self::ResponseContext)>;

    /// Execute the created GRPC request against the provided GRPC channel.
    async fn execute(
        channel: Channel,
        request: Self::GrpcRequest,
    ) -> Result<tonic::Response<Self::GrpcResponse>, tonic::Status>;

    /// Create a response from the GRPC response and the saved transaction
    /// and node account ID from the successful request.
    fn make_response(
        response: Self::GrpcResponse,
        context: Self::ResponseContext,
        node_account_id: AccountId,
        transaction_id: Option<TransactionId>,
    ) -> crate::Result<Self::Response>;

    /// Extract the pre-check status from the GRPC response.
    fn response_pre_check_status(response: &Self::GrpcResponse) -> crate::Result<i32>;
}

pub(crate) async fn execute<E>(
    client: &Client,
    executable: &E,
    context: E::RequestContext,
) -> crate::Result<E::Response>
where
    E: Execute,
{
    // the overall timeout for the backoff starts measuring from here

    let mut rng = thread_rng();
    let mut backoff = ExponentialBackoff::default();
    let mut last_error: Option<Error> = None;
    let mut last_request: Option<(AccountId, E::ResponseContext)> = None;
    let max_attempts = 10; // FIXME: from client

    // TODO: cache requests to avoid signing a new request for every node in a delayed back-off

    // if we need to generate a transaction ID for this request (and one was not provided),
    // generate one now
    let explicit_transaction_id = executable.transaction_id();
    let mut transaction_id = E::requires_transaction_id()
        .then(|| explicit_transaction_id.or_else(|| client.generate_transaction_id()))
        .flatten();

    // if we were explicitly given a list of nodes to use, we iterate through each
    // of the given nodes (in a random order)

    let explicit_node_indexes = executable
        .node_account_ids()
        .map(|ids| client.network.node_indexes_for_ids(ids))
        .transpose()?;

    // the outer loop continues until we timeout or reach the maximum number of "attempts"
    // an attempt is counted when we have a successful response from a node that must either
    // be retried immediately (on a new node) or retried after a backoff.

    for _ in 0..max_attempts {
        // if no explicit set of node account IDs, we randomly sample 1/3 of all
        // healthy nodes on the client. this set of healthy nodes can change on
        // each iteration

        let healthy_node_indexes =
            explicit_node_indexes.is_none().then(|| client.network.healthy_node_indexes());

        let node_indexes =
            explicit_node_indexes.as_deref().or(healthy_node_indexes.as_deref()).unwrap();

        let node_sample_amount = if explicit_node_indexes.is_none() {
            (node_indexes.len() + 2) / 3
        } else {
            node_indexes.len()
        };

        for &node_index in node_indexes.choose_multiple(&mut rng, node_sample_amount) {
            let (node_account_id, channel) = client.network.channel(node_index);

            let (request, context) =
                executable.make_request(client, transaction_id, node_account_id, &context).await?;

            last_request = Some((node_account_id, context));

            let response = match E::execute(channel, request).await {
                Ok(response) => response.into_inner(),
                Err(status) => {
                    match status.code() {
                        tonic::Code::Unavailable | tonic::Code::ResourceExhausted => {
                            // NOTE: this is an "unhealthy" node
                            client.network.mark_node_unhealthy(node_index);

                            // try the next node in our allowed list, immediately
                            last_error = Some(status.into());
                            continue;
                        }

                        _ => {
                            // fail immediately
                            return Err(status.into());
                        }
                    }
                }
            };

            let pre_check_status = E::response_pre_check_status(&response)?;

            match ResponseCodeEnum::from_i32(pre_check_status) {
                Some(status) => match status {
                    ResponseCodeEnum::Ok => {
                        // TODO: another function in the Execute trait to see if we need to
                        //  retry yet again

                        let (node_account_id, context) = last_request.unwrap();

                        return E::make_response(
                            response,
                            context,
                            node_account_id,
                            transaction_id,
                        );
                    }

                    ResponseCodeEnum::Busy | ResponseCodeEnum::PlatformNotActive => {
                        // NOTE: this is a "busy" node
                        // try the next node in our allowed list, immediately
                        last_error = Some(Error::pre_check(status));
                        continue;
                    }

                    ResponseCodeEnum::TransactionExpired if explicit_transaction_id.is_none() => {
                        // the transaction that was generated has since expired
                        // re-generate the transaction ID and try again, immediately
                        transaction_id = client.generate_transaction_id();
                        last_error = Some(Error::pre_check(status));
                        continue;
                    }

                    _ => {
                        // any other pre-check is an error that the user needs to fix, fail immediately
                        return Err(Error::pre_check(status));
                    }
                },

                None => {
                    // not sure how to proceed, fail immediately
                    return Err(Error::ResponseStatusUnrecognized(pre_check_status));
                }
            }
        }

        // we tried each node, suspend execution until the next backoff interval
        if let Some(duration) = backoff.next_backoff() {
            sleep(duration).await;
        } else {
            // maximum time allowed has elapsed
            // NOTE: it should be impossible to reach here without capturing at least one error
            return Err(Error::TimedOut(last_error.unwrap().into()));
        }
    }

    // NOTE: it should be impossible to reach here without capturing at least one error
    Err(Error::MaxAttemptsExceededException(Box::new(last_error.unwrap())))
}
