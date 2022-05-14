use std::fmt::Debug;

use async_trait::async_trait;
use hedera_proto::services;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tonic::transport::Channel;

use crate::execute::Execute;
use crate::query::{AnyQueryData, ToQueryProtobuf};
use crate::{AccountId, Client, Error, FromProtobuf, Query, TransactionId};

/// Describes a specific query that can be executed on the Hedera network.
#[async_trait]
pub trait QueryExecute:
    Sync + Send + Into<AnyQueryData> + Clone + Debug + Serialize + DeserializeOwned + ToQueryProtobuf
{
    type Response: FromProtobuf<Protobuf = services::response::Response>;

    /// Returns `true` if this query requires a payment to be submitted.
    fn is_payment_required(&self) -> bool {
        true
    }

    /// Execute the prepared query request against the provided GRPC channel.
    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status>;
}

#[async_trait]
impl<D> Execute for Query<D>
where
    D: QueryExecute,
{
    type GrpcRequest = services::Query;

    type GrpcResponse = services::Response;

    type Response = D::Response;

    type Context = ();

    fn node_account_ids(&self) -> Option<&[AccountId]> {
        self.payment.node_account_ids()
    }

    fn transaction_id(&self) -> Option<TransactionId> {
        self.payment.transaction_id()
    }

    fn requires_transaction_id(&self) -> bool {
        self.data.is_payment_required()
    }

    async fn make_request(
        &self,
        client: &Client,
        transaction_id: &Option<TransactionId>,
        node_account_id: AccountId,
    ) -> crate::Result<(Self::GrpcRequest, Self::Context)> {
        let payment = if self.data.is_payment_required() {
            Some(self.payment.make_request(client, transaction_id, node_account_id).await?.0)
        } else {
            None
        };

        let header = services::QueryHeader { response_type: 0, payment };

        Ok((self.data.to_query_protobuf(header), ()))
    }

    async fn execute(
        &self,
        channel: Channel,
        request: Self::GrpcRequest,
    ) -> Result<tonic::Response<Self::GrpcResponse>, tonic::Status> {
        self.data.execute(channel, request).await
    }

    fn make_response(
        response: Self::GrpcResponse,
        _context: Self::Context,
        _node_account_id: AccountId,
        _transaction_id: Option<TransactionId>,
    ) -> crate::Result<Self::Response> {
        let response = pb_getf!(response, response)?;

        <D::Response as FromProtobuf>::from_protobuf(response)
    }

    fn response_pre_check_status(response: &Self::GrpcResponse) -> crate::Result<i32> {
        Ok(response_header(&response.response)?.node_transaction_precheck_code)
    }
}

pub(super) fn response_header(
    response: &Option<services::response::Response>,
) -> crate::Result<&services::ResponseHeader> {
    use services::response::Response::*;

    let header = match response {
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
