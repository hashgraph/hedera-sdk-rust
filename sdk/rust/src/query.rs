use async_trait::async_trait;
use backoff::backoff::Backoff;
use backoff::ExponentialBackoff;
use hedera_proto::services;
use hedera_proto::services::ResponseCodeEnum;
use rand::thread_rng;
use tokio::time::sleep;
use tonic::transport::Channel;
use tonic::{Response, Status};

use crate::execute::{execute, Execute};
use crate::{AccountId, Client, Error, FromProtobuf, ToProtobuf, TransactionId};

#[async_trait]
pub trait QueryExecute {
    type Response: FromProtobuf<Protobuf = services::response::Response>;

    async fn execute(
        channel: Channel,
        request: services::Query,
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

#[async_trait]
impl<D> Execute for Query<D>
where
    Self: QueryExecute,
    D: ToProtobuf<Protobuf = services::Query>,
{
    type GrpcRequest = services::Query;

    type GrpcResponse = services::Response;

    type Response = <Self as QueryExecute>::Response;

    type RequestContext = ();

    type ResponseContext = ();

    fn node_account_ids(&self) -> Option<&[AccountId]> {
        self.node_account_ids.as_deref()
    }

    fn transaction_id(&self) -> Option<TransactionId> {
        // TODO: paid queries
        None
    }

    fn requires_transaction_id() -> bool {
        // TODO: paid queries
        false
    }

    async fn make_request(
        &self,
        client: &Client,
        _transaction_id: Option<TransactionId>,
        _node_account_id: AccountId,
        _context: &Self::RequestContext,
    ) -> crate::Result<(Self::GrpcRequest, Self::ResponseContext)> {
        // TODO: paid queries
        Ok((self.data.to_protobuf(), ()))
    }

    async fn execute(
        channel: Channel,
        request: Self::GrpcRequest,
    ) -> Result<tonic::Response<Self::GrpcResponse>, tonic::Status> {
        <Self as QueryExecute>::execute(channel, request).await
    }

    fn make_response(
        response: Self::GrpcResponse,
        _context: Self::ResponseContext,
        _node_account_id: AccountId,
        _transaction_id: Option<TransactionId>,
    ) -> crate::Result<Self::Response> {
        let response = pb_getf!(response, response)?;

        <<Self as QueryExecute>::Response as FromProtobuf>::from_protobuf(response)
    }

    fn response_pre_check_status(response: &Self::GrpcResponse) -> crate::Result<i32> {
        Ok(response_header(response)?.node_transaction_precheck_code)
    }
}

impl<D> Query<D>
where
    Self: QueryExecute,
    D: ToProtobuf<Protobuf = services::Query>,
{
    /// Execute this query against the provided client of the Hedera network.
    #[inline]
    pub async fn execute(
        &self,
        client: &Client,
    ) -> crate::Result<<Self as QueryExecute>::Response> {
        execute(client, self, ()).await
    }
}

fn response_header(response: &services::Response) -> crate::Result<&services::ResponseHeader> {
    use services::response::Response::*;

    let header = match &response.response {
        Some(CryptogetAccountBalance(response)) => &response.header,
        Some(GetByKey(response)) => &response.header,
        Some(GetBySolidityId(response)) => &response.header,
        Some(ContractCallLocal(response)) => &response.header,
        Some(ContractGetBytecodeResponse(response)) => &response.header,
        Some(ContractGetInfo(response)) => &response.header,
        Some(ContractGetRecordsResponse(response)) => &response.header,
        Some(CryptoGetAccountRecords(response)) => &response.header,
        Some(CryptoGetInfo(response)) => &response.header,
        Some(CryptoGetLiveHash(response)) => &response.header,
        Some(CryptoGetProxyStakers(response)) => &response.header,
        Some(FileGetContents(response)) => &response.header,
        Some(FileGetInfo(response)) => &response.header,
        Some(TransactionGetReceipt(response)) => &response.header,
        Some(TransactionGetRecord(response)) => &response.header,
        Some(TransactionGetFastRecord(response)) => &response.header,
        Some(ConsensusGetTopicInfo(response)) => &response.header,
        Some(NetworkGetVersionInfo(response)) => &response.header,
        Some(TokenGetInfo(response)) => &response.header,
        Some(ScheduleGetInfo(response)) => &response.header,
        Some(TokenGetAccountNftInfos(response)) => &response.header,
        Some(TokenGetNftInfo(response)) => &response.header,
        Some(TokenGetNftInfos(response)) => &response.header,
        Some(NetworkGetExecutionTime(response)) => &response.header,
        Some(AccountDetails(response)) => &response.header,
        None => &None,
    };

    header.as_ref().ok_or_else(|| Error::from_protobuf("unexpected missing `header` in `Response`"))
}
