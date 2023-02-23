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

use backoff::ExponentialBackoff;
use prost::Message;
use rand::seq::SliceRandom;
use rand::thread_rng;
use time::OffsetDateTime;
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
        transaction_id: &Option<TransactionId>,
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
        transaction_id: Option<TransactionId>,
    ) -> crate::Result<Self::Response>;

    /// Create an error from the given pre-check status.
    fn make_error_pre_check(
        &self,
        status: Status,
        transaction_id: Option<TransactionId>,
    ) -> crate::Error;

    /// Extract the pre-check status from the GRPC response.
    fn response_pre_check_status(response: &Self::GrpcResponse) -> crate::Result<i32>;
}

pub(crate) async fn execute<E>(
    client: &Client,
    executable: &E,
    timeout: impl Into<Option<std::time::Duration>> + Send,
) -> crate::Result<E::Response>
where
    E: Execute + Sync,
{
    let timeout: Option<std::time::Duration> = timeout.into();

    let timeout = timeout.or_else(|| client.request_timeout()).unwrap_or_else(|| {
        std::time::Duration::from_millis(backoff::default::MAX_ELAPSED_TIME_MILLIS)
    });

    // the overall timeout for the backoff starts measuring from here
    let backoff =
        ExponentialBackoff { max_elapsed_time: Some(timeout), ..ExponentialBackoff::default() };

    if client.auto_validate_checksums() {
        if let Some(ledger_id) = &*client.ledger_id_internal() {
            executable.validate_checksums(ledger_id)?;
        } else {
            return Err(Error::CannotPerformTaskWithoutLedgerId { task: "validate checksums" });
        }
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

    // if all nodes are unhealthy then we must try at least one of them...
    let mut include_unhealthy = false;

    let layer = move |include_unhealthy: bool| async move {
        let mut last_error: Option<Error> = None;

        let random_node_indexes =
            random_node_indexes(client, include_unhealthy, explicit_node_indexes)
                .ok_or(retry::Error::EmptyTransient)?;

        for node_index in random_node_indexes {
            let (node_account_id, channel) = client.network().channel(node_index);

            let (request, context) = executable
                .make_request(&transaction_id, node_account_id)
                .map_err(crate::retry::Error::Permanent)?;

            let response =
                executable.execute(channel, request).await.map(|it| it.into_inner()).map_err(
                    |status| match status.code() {
                        tonic::Code::Unavailable | tonic::Code::ResourceExhausted => {
                            // NOTE: this is an "unhealthy" node
                            client.network().mark_node_unhealthy(node_index);

                            // try the next node in our allowed list, immediately
                            retry::Error::Transient(status.into())
                        }

                        _ => {
                            // fail immediately
                            retry::Error::Permanent(status.into())
                        }
                    },
                );

            let response = match response {
                Ok(response) => response,
                Err(retry::Error::Transient(err)) => {
                    last_error = Some(err);
                    continue;
                }

                Err(e) => return Err(e),
            };

            let status = E::response_pre_check_status(&response)
                .map_err(retry::Error::Permanent)
                .and_then(|status| {
                // not sure how to proceed, fail immediately
                Status::from_i32(status).ok_or_else(|| {
                    retry::Error::Permanent(Error::ResponseStatusUnrecognized(status))
                })
            })?;

            let (new_last_error, new_transaction_id) = match status {
                Status::Ok if executable.should_retry(&response) => {
                    return Err(retry::Error::Transient(
                        executable.make_error_pre_check(status, transaction_id),
                    ))
                }

                Status::Ok => {
                    return executable
                        .make_response(response, context, node_account_id, transaction_id)
                        .map_err(retry::Error::Permanent);
                }

                Status::Busy | Status::PlatformNotActive => {
                    // NOTE: this is a "busy" node
                    // try the next node in our allowed list, immediately
                    (executable.make_error_pre_check(status, transaction_id), None)
                }

                Status::TransactionExpired if explicit_transaction_id.is_none() => {
                    // the transaction that was generated has since expired
                    // re-generate the transaction ID and try again, immediately
                    (
                        executable.make_error_pre_check(status, transaction_id),
                        client.generate_transaction_id(),
                    )
                }

                _ if executable.should_retry_pre_check(status) => {
                    // conditional retry on pre-check should back-off and try again
                    return Err(retry::Error::Transient(
                        executable.make_error_pre_check(status, transaction_id),
                    ));
                }

                _ => {
                    // any other pre-check is an error that the user needs to fix, fail immediately
                    return Err(retry::Error::Permanent(
                        executable.make_error_pre_check(status, transaction_id),
                    ));
                }
            };

            last_error = Some(new_last_error);
            transaction_id = new_transaction_id.or(transaction_id);
            continue;
        }

        // unreachable panic:
        // in order for `last_error` to be `None` that'd mean that
        return Err(retry::Error::Transient(last_error.unwrap()));
    };

    // is this a layered fn? Yep. Is this required? Yes, unless we want more nesting.
    let layer = move || async move {
        let include_unhealthy = &mut include_unhealthy;
        match layer(*include_unhealthy).await {
            // on a transient error we want to enable `include_unhealthy` (if it isn't already enabled)
            result @ Err(retry::Error::Transient(_)) => {
                // only ever include unhealthy nodes if we have explicit nodes.
                *include_unhealthy = explicit_node_indexes.is_some();

                result
            }

            result => result,
        }
    };

    // the outer loop continues until we timeout or reach the maximum number of "attempts"
    // an attempt is counted when we have a successful response from a node that must either
    // be retried immediately (on a new node) or retried after a backoff.
    crate::retry(backoff, layer).await
}

// todo: return an iterator.
fn random_node_indexes(
    client: &Client,
    include_unhealthy: bool,
    explicit_node_indexes: Option<&[usize]>,
) -> Option<Vec<usize>> {
    // cache the rng impl and "now" because `thread_rng` is TLS (a thread local),
    // and because using the same reference time avoids situations where a node that wasn't available becomes available.
    let mut rng = thread_rng();
    let now = OffsetDateTime::now_utc();

    if let Some(indexes) = explicit_node_indexes {
        let mut indexes = match include_unhealthy {
            true => indexes.to_vec(),
            false => {
                let tmp: Vec<_> = indexes
                    .iter()
                    .copied()
                    .filter(|index| client.network().is_node_healthy(*index, now))
                    .collect();

                if tmp.is_empty() {
                    indexes.to_vec()
                } else {
                    tmp
                }
            }
        };

        if indexes.is_empty() {
            panic!("empty explicitly set nodes")
        }

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

        return Some(shuffled.to_vec());
    }
}
