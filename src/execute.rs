/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use std::ops::ControlFlow;

use backoff::ExponentialBackoff;
use futures_core::future::BoxFuture;
use futures_util::StreamExt;
use prost::Message;
use rand::seq::SliceRandom;
use rand::thread_rng;
use time::OffsetDateTime;
use tonic::metadata::AsciiMetadataValue;
use tonic::transport::Channel;

use crate::{
    retry,
    AccountId,
    BoxGrpcFuture,
    Client,
    Error,
    Status,
    TransactionId,
    ValidateChecksums,
};

pub(crate) trait Execute: ValidateChecksums {
    type GrpcRequest: Clone + Message;

    type GrpcResponse: Message;

    /// Additional context returned from each call to `make_request`. Upon
    /// a successful request, the associated response context is passed to
    /// `make_response`.
    type Context: Send;

    type Response;

    /// Get the _explicit_ nodes that this request will be submitted to.
    fn node_account_ids(&self) -> Option<&[AccountId]>;

    /// Get the _explicit_ transaction ID that this request will use.
    fn transaction_id(&self) -> Option<TransactionId>;

    /// Get whether to generate transaction IDs for request creation.
    fn requires_transaction_id(&self) -> bool;

    /// Check whether to retry an pre-check status.
    fn should_retry_pre_check(&self, _status: Status) -> bool {
        false
    }

    /// Check whether we should retry an otherwise successful response.
    #[allow(unused_variables)]
    fn should_retry(&self, response: &Self::GrpcResponse) -> bool {
        false
    }

    /// Create a new request for execution.
    ///
    /// A created request is cached per node until any request returns
    /// `TransactionExpired`; in which case, the request cache is cleared.
    fn make_request(
        &self,
        transaction_id: Option<&TransactionId>,
        node_account_id: AccountId,
    ) -> crate::Result<(Self::GrpcRequest, Self::Context)>;

    /// Execute the created GRPC request against the provided GRPC channel.
    fn execute(
        &self,
        channel: Channel,
        request: Self::GrpcRequest,
    ) -> BoxGrpcFuture<Self::GrpcResponse>;

    /// Create a response from the GRPC response and the saved transaction
    /// and node account ID from the successful request.
    fn make_response(
        &self,
        response: Self::GrpcResponse,
        context: Self::Context,
        node_account_id: AccountId,
        transaction_id: Option<&TransactionId>,
    ) -> crate::Result<Self::Response>;

    /// Create an error from the given pre-check status.
    fn make_error_pre_check(
        &self,
        status: Status,
        transaction_id: Option<&TransactionId>,
    ) -> crate::Error;

    /// Extract the pre-check status from the GRPC response.
    fn response_pre_check_status(response: &Self::GrpcResponse) -> crate::Result<i32>;
}

pub(crate) async fn execute<E>(
    client: &Client,
    executable: &E,
    timeout: Option<std::time::Duration>,
) -> crate::Result<E::Response>
where
    E: Execute + Sync,
{
    fn recurse_ping(client: &Client, node_index: usize) -> BoxFuture<'_, bool> {
        Box::pin(async move { client.ping(client.network().node_ids()[node_index]).await.is_ok() })
    }

    let timeout = timeout.or_else(|| client.request_timeout()).unwrap_or_else(|| {
        std::time::Duration::from_millis(backoff::default::MAX_ELAPSED_TIME_MILLIS)
    });

    // the overall timeout for the backoff starts measuring from here
    let backoff =
        ExponentialBackoff { max_elapsed_time: Some(timeout), ..ExponentialBackoff::default() };

    if client.auto_validate_checksums() {
        let ledger_id = client.ledger_id_internal();
        let ledger_id = ledger_id
            .as_ref()
            .expect("Client had auto_validate_checksums enabled but no ledger ID");

        executable.validate_checksums(ledger_id)?;
    }

    // TODO: cache requests to avoid signing a new request for every node in a delayed back-off

    // if we need to generate a transaction ID for this request (and one was not provided),
    // generate one now
    let explicit_transaction_id = executable.transaction_id();
    let mut transaction_id = executable
        .requires_transaction_id()
        .then(|| explicit_transaction_id.or_else(|| client.generate_transaction_id()))
        .flatten();

    // if we were explicitly given a list of nodes to use, we iterate through each
    // of the given nodes (in a random order)
    let explicit_node_indexes = executable
        .node_account_ids()
        .map(|ids| client.network().node_indexes_for_ids(ids))
        .transpose()?;

    let explicit_node_indexes = explicit_node_indexes.as_deref();

    let layer = move || async move {
        loop {
            let mut last_error: Option<Error> = None;

            let random_node_indexes = random_node_indexes(client, explicit_node_indexes)
                .ok_or(retry::Error::EmptyTransient)?;

            let random_node_indexes = {
                let random_node_indexes = &random_node_indexes;
                let client = client;
                let now = OffsetDateTime::now_utc();
                futures_util::stream::iter(random_node_indexes.iter().copied()).filter(
                    move |&node_index| async move {
                        // NOTE: For pings we're relying on the fact that they have an explict node index.
                        explicit_node_indexes.is_some()
                            || client.network().node_recently_pinged(node_index, now)
                            || recurse_ping(client, node_index).await
                    },
                )
            };

            let mut random_node_indexes = std::pin::pin!(random_node_indexes);

            while let Some(node_index) = random_node_indexes.next().await {
                let tmp = execute_single(
                    client,
                    executable,
                    node_index,
                    explicit_transaction_id.is_some(),
                    &mut transaction_id,
                )
                .await;

                client.network().mark_node_used(node_index, OffsetDateTime::now_utc());

                match tmp? {
                    ControlFlow::Continue(err) => last_error = Some(err),
                    ControlFlow::Break(res) => return Ok(res),
                }
            }

            match last_error {
                Some(it) => return Err(retry::Error::Transient(it)),
                // this can only happen if we skipped every node due to pinging it coming up `false` (unhealthy)... The node will be marked as unhealthy, soo
                None => continue,
            }
        }
    };

    // the outer loop continues until we timeout or reach the maximum number of "attempts"
    // an attempt is counted when we have a successful response from a node that must either
    // be retried immediately (on a new node) or retried after a backoff.
    crate::retry(backoff, layer).await
}

fn map_tonic_error(status: tonic::Status, client: &Client, node_index: usize) -> retry::Error {
    const MIME_HTML: &[u8] = b"text/html";

    match status.code() {
        // if the node says it isn't available, then we should just try again with a different node.
        tonic::Code::Unavailable | tonic::Code::ResourceExhausted => {
            // NOTE: this is an "unhealthy" node
            client.network().mark_node_unhealthy(node_index);

            // try the next node in our allowed list, immediately
            retry::Error::Transient(status.into())
        }

        // todo: find a way to make this less fragile
        // if this happens:
        // the node is completely borked (we're probably seeing the load balancer's response),
        // and we have no clue if the effect went through
        tonic::Code::Internal
            if status.metadata().get("content-type").map(AsciiMetadataValue::as_bytes)
                == Some(MIME_HTML) =>
        {
            client.network().mark_node_unhealthy(node_index);

            retry::Error::Permanent(status.into())
        }

        // fail immediately
        _ => retry::Error::Permanent(status.into()),
    }
}

async fn execute_single<E: Execute + Sync>(
    client: &Client,
    executable: &E,
    node_index: usize,
    has_explicit_transaction_id: bool,
    transaction_id: &mut Option<TransactionId>,
) -> retry::Result<ControlFlow<E::Response, Error>> {
    let (node_account_id, channel) = client.network().channel(node_index);

    let (request, context) = executable
        .make_request(transaction_id.as_ref(), node_account_id)
        .map_err(crate::retry::Error::Permanent)?;

    let response = executable
        .execute(channel, request)
        .await
        .map(tonic::Response::into_inner)
        .map_err(|status| map_tonic_error(status, client, node_index));

    let response = match response {
        Ok(response) => response,
        Err(retry::Error::Transient(err)) => {
            return Ok(ControlFlow::Continue(err));
        }

        Err(e) => return Err(e),
    };

    let status = E::response_pre_check_status(&response)
        .and_then(|status| {
            // not sure how to proceed, fail immediately
            Status::from_i32(status).ok_or_else(|| Error::ResponseStatusUnrecognized(status))
        })
        .map_err(retry::Error::Permanent)?;

    match status {
        Status::Ok if executable.should_retry(&response) => Err(retry::Error::Transient(
            executable.make_error_pre_check(status, transaction_id.as_ref()),
        )),

        Status::Ok => executable
            .make_response(response, context, node_account_id, transaction_id.as_ref())
            .map(ControlFlow::Break)
            .map_err(retry::Error::Permanent),

        Status::Busy | Status::PlatformNotActive => {
            // NOTE: this is a "busy" node
            // try the next node in our allowed list, immediately
            Ok(ControlFlow::Continue(
                executable.make_error_pre_check(status, transaction_id.as_ref()),
            ))
        }

        Status::TransactionExpired if !has_explicit_transaction_id => {
            // the transaction that was generated has since expired
            // re-generate the transaction ID and try again, immediately
            let _ = transaction_id.insert(client.generate_transaction_id().unwrap());

            Ok(ControlFlow::Continue(
                executable.make_error_pre_check(status, transaction_id.as_ref()),
            ))
        }

        _ if executable.should_retry_pre_check(status) => {
            // conditional retry on pre-check should back-off and try again
            Err(retry::Error::Transient(
                executable.make_error_pre_check(status, transaction_id.as_ref()),
            ))
        }

        _ => {
            // any other pre-check is an error that the user needs to fix, fail immediately
            Err(retry::Error::Permanent(
                executable.make_error_pre_check(status, transaction_id.as_ref()),
            ))
        }
    }
}

// todo: return an iterator.
fn random_node_indexes(
    client: &Client,
    explicit_node_indexes: Option<&[usize]>,
) -> Option<Vec<usize>> {
    // cache the rng impl and "now" because `thread_rng` is TLS (a thread local),
    // and because using the same reference time avoids situations where a node that wasn't available becomes available.
    let mut rng = thread_rng();
    let now = OffsetDateTime::now_utc();

    if let Some(indexes) = explicit_node_indexes {
        let tmp: Vec<_> = indexes
            .iter()
            .copied()
            .filter(|index| client.network().is_node_healthy(*index, now))
            .collect();

        let mut indexes = if tmp.is_empty() { indexes.to_vec() } else { tmp };

        assert!(!indexes.is_empty(), "empty explicitly set nodes");

        indexes.shuffle(&mut rng);

        return Some(indexes);
    }

    {
        let mut indexes: Vec<_> = client.network().healthy_node_indexes(now).collect();

        if indexes.is_empty() {
            return None;
        }

        // would put this inline, but borrowck wouldn't allow that.
        let amount = (indexes.len() + 2) / 3;

        let (shuffled, _) = indexes.partial_shuffle(&mut rng, amount);

        Some(shuffled.to_vec())
    }
}
