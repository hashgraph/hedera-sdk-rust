use async_trait::async_trait;
use backoff::backoff::Backoff;
use backoff::ExponentialBackoff;
use hedera_proto::services;
use hedera_proto::services::ResponseCodeEnum;
use tokio::time::sleep;

use crate::client::NetworkChannel;
use crate::{AccountId, Client, Error, FromProtobuf, ToProtobuf};

#[async_trait]
pub trait QueryExecute {
    type Response: FromProtobuf<Protobuf = services::response::Response>;

    async fn execute(
        &self,
        channel: NetworkChannel,
    ) -> Result<tonic::Response<services::Response>, tonic::Status>;
}

#[derive(Debug, Default)]
pub struct Query<D> {
    pub(crate) data: D,
    pub(crate) node_account_ids: Option<Vec<AccountId>>,
    // TODO: payment_transaction: Option<TransferTransaction>,
    payment_amount: Option<u64>,
    payment_amount_max: Option<Option<u64>>,
}

impl<D> Query<D>
where
    D: Default,
{
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<D> Query<D> {
    /// Set the account IDs of the nodes that this query may be submitted to.
    ///
    /// Defaults to the full list of nodes configured on the client; or, the node account IDs
    /// configured on the query payment transaction (if explicitly provided).
    ///
    pub fn node_account_ids(&mut self, ids: impl IntoIterator<Item = AccountId>) -> &mut Self {
        self.node_account_ids = Some(ids.into_iter().collect());
        self
    }

    /// Set an explicit payment amount for this query.
    ///
    /// The client will submit exactly this amount for the payment of this query. Hedera
    /// will not return any remainder (over the actual cost for this query).
    ///
    // TODO: Use Hbar
    pub fn payment_amount(&mut self, amount: impl Into<Option<u64>>) -> &mut Self {
        self.payment_amount = amount.into();
        self
    }

    /// Set the maximum payment allowable for this query.
    ///
    /// When a query is executed without an explicit payment amount set,
    /// the client will first request the cost of the given query from the node it will be
    /// submitted to and attach a payment for that amount from the operator account on the client.
    ///
    /// If the returned value is greater than this value, a [`MaxQueryPaymentExceeded`] error
    /// will be returned.
    ///
    /// Defaults to the maximum payment amount configured on the client.
    ///
    /// Set to `None` to disable automatic query payments for this query.
    ///
    pub fn max_payment_amount(&mut self, max: impl Into<Option<u64>>) -> &mut Self {
        self.payment_amount_max = Some(max.into());
        self
    }
}

impl<D> Query<D>
where
    Self: QueryExecute,
    D: ToProtobuf<Protobuf = services::Query>,
{
    /// Execute this query against the provided client of the Hedera network.
    pub async fn execute(
        &self,
        client: &Client,
    ) -> crate::Result<<Self as QueryExecute>::Response> {
        let mut backoff = ExponentialBackoff::default();
        let mut last_error: Option<Error> = None;

        loop {
            // FIXME: do we really want to check EVERY node before we start backing off?
            //  we can sample 2/3 or even 1/3 safely I think

            let num_nodes = self
                .node_account_ids
                .as_ref()
                .map(|ids| ids.len())
                .unwrap_or_else(|| client.network.num_nodes());

            let node_indexes =
                rand::seq::index::sample(&mut rand::thread_rng(), num_nodes, num_nodes);

            for node_index in node_indexes.iter() {
                let channel = match &self.node_account_ids {
                    Some(ids) => client.network.channel(ids[node_index]),
                    None => client.network.channel_nth(node_index),
                };

                let response = match QueryExecute::execute(self, channel).await {
                    Ok(response) => response,
                    Err(status) => {
                        match status.code() {
                            tonic::Code::Unavailable | tonic::Code::ResourceExhausted => {
                                // NOTE: this is an "unhealthy" node
                                // try the next node in our allowed list, immediately
                                last_error = Some(status.into());
                                continue;
                            }

                            // FIXME: handle stream has been reset failures (?)
                            _ => {
                                // fail immediately
                                return Err(status.into());
                            }
                        }
                    }
                };

                let response = response.into_inner();

                let mut response = pb_getf!(response, response, "Response")?;

                let header = response_take_header(&mut response)?;
                let status =
                    ResponseCodeEnum::from_i32(header.node_transaction_precheck_code).unwrap();

                match status {
                    ResponseCodeEnum::Ok => {
                        return <<Self as QueryExecute>::Response as FromProtobuf>::from_protobuf(
                            response,
                        );
                    }

                    ResponseCodeEnum::Busy | ResponseCodeEnum::PlatformNotActive => {
                        // NOTE: this is a "busy" node
                        // try the next node in our allowed list, immediately
                        last_error = Some(Error::query_pre_check(status));
                        continue;
                    }

                    _ => {
                        // fail immediately
                        return Err(Error::query_pre_check(status));
                    }
                }
            }

            // we tried ~every~ node in our defined network, suspend execution until the next
            // backoff interval
            if let Some(duration) = backoff.next_backoff() {
                sleep(duration).await;
            } else {
                // maximum time allowed has elapsed
                // return a timeout failure with the last observed error
                return Err(Error::Timeout(last_error.unwrap().into()));
            }
        }
    }
}

fn response_take_header(
    response: &mut services::response::Response,
) -> crate::Result<services::ResponseHeader> {
    use services::response::Response::*;

    let header = match response {
        CryptogetAccountBalance(response) => response.header.take(),
        GetByKey(response) => response.header.take(),
        GetBySolidityId(response) => response.header.take(),
        ContractCallLocal(response) => response.header.take(),
        ContractGetBytecodeResponse(response) => response.header.take(),
        ContractGetInfo(response) => response.header.take(),
        ContractGetRecordsResponse(response) => response.header.take(),
        CryptoGetAccountRecords(response) => response.header.take(),
        CryptoGetInfo(response) => response.header.take(),
        CryptoGetLiveHash(response) => response.header.take(),
        CryptoGetProxyStakers(response) => response.header.take(),
        FileGetContents(response) => response.header.take(),
        FileGetInfo(response) => response.header.take(),
        TransactionGetReceipt(response) => response.header.take(),
        TransactionGetRecord(response) => response.header.take(),
        TransactionGetFastRecord(response) => response.header.take(),
        ConsensusGetTopicInfo(response) => response.header.take(),
        NetworkGetVersionInfo(response) => response.header.take(),
        TokenGetInfo(response) => response.header.take(),
        ScheduleGetInfo(response) => response.header.take(),
        TokenGetAccountNftInfos(response) => response.header.take(),
        TokenGetNftInfo(response) => response.header.take(),
        TokenGetNftInfos(response) => response.header.take(),
        NetworkGetExecutionTime(response) => response.header.take(),
        AccountDetails(response) => response.header.take(),
    };

    header.ok_or_else(|| Error::from_protobuf("unexpected missing `header` in `Response`"))
}
