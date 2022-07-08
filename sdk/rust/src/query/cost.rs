use async_trait::async_trait;
use hedera_proto::services;
use tonic::transport::Channel;

use crate::execute::{
    execute,
    Execute,
};
use crate::query::execute::response_header;
use crate::query::QueryExecute;
use crate::{
    AccountId,
    Client,
    Query,
    TransactionId,
};

pub(super) struct QueryCost<'a, D>(&'a Query<D>)
where
    D: QueryExecute;

impl<'a, D> QueryCost<'a, D>
where
    D: QueryExecute,
{
    pub(super) fn new(query: &'a Query<D>) -> Self {
        Self(query)
    }
}

#[async_trait]
impl<D> Execute for QueryCost<'_, D>
where
    Query<D>: Execute,
    D: QueryExecute,
{
    type GrpcRequest = services::Query;

    type GrpcResponse = services::Response;

    type Response = u64;

    type Context = ();

    fn node_account_ids(&self) -> Option<&[AccountId]> {
        Execute::node_account_ids(self.0)
    }

    fn transaction_id(&self) -> Option<TransactionId> {
        None
    }

    fn requires_transaction_id(&self) -> bool {
        false
    }

    async fn make_request(
        &self,
        _client: &Client,
        _transaction_id: &Option<TransactionId>,
        _node_account_id: AccountId,
    ) -> crate::Result<(Self::GrpcRequest, Self::Context)> {
        let header = services::QueryHeader {
            response_type: services::ResponseType::CostAnswer as i32,
            payment: None,
        };

        Ok((self.0.data.to_query_protobuf(header), ()))
    }

    async fn execute(
        &self,
        channel: Channel,
        request: Self::GrpcRequest,
    ) -> Result<tonic::Response<Self::GrpcResponse>, tonic::Status> {
        <D as QueryExecute>::execute(&self.0.data, channel, request).await
    }

    fn make_response(
        &self,
        response: Self::GrpcResponse,
        _context: Self::Context,
        _node_account_id: AccountId,
        _transaction_id: Option<TransactionId>,
    ) -> crate::Result<Self::Response> {
        Ok(response_header(&response.response)?.cost)
    }

    fn make_error_pre_check(
        &self,
        status: crate::Status,
        transaction_id: Option<TransactionId>,
    ) -> crate::Error {
        if let Some(transaction_id) = self.0.data.transaction_id() {
            crate::Error::QueryPreCheckStatus { status, transaction_id }
        } else if let Some(transaction_id) = transaction_id {
            crate::Error::QueryPaymentPreCheckStatus { status, transaction_id }
        } else {
            crate::Error::QueryNoPaymentPreCheckStatus { status }
        }
    }

    fn response_pre_check_status(response: &Self::GrpcResponse) -> crate::Result<i32> {
        Ok(response_header(&response.response)?.node_transaction_precheck_code)
    }
}

impl<D> QueryCost<'_, D>
where
    D: QueryExecute,
{
    /// Execute this query against the provided client of the Hedera network.
    pub async fn execute(&mut self, client: &Client) -> crate::Result<u64> {
        execute(client, self).await
    }
}
