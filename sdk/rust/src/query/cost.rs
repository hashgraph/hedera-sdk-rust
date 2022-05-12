use async_trait::async_trait;
use hedera_proto::services;
use tonic::transport::Channel;

use crate::execute::{execute, Execute};
use crate::query::execute::response_header;
use crate::query::{QueryData, QueryExecute};
use crate::{AccountId, Client, Query, TransactionId};

pub(super) struct QueryCost<'a, D>(&'a Query<D>)
where
    D: QueryData;

impl<'a, D> QueryCost<'a, D>
where
    D: QueryData,
{
    pub(super) fn new(query: &'a Query<D>) -> Self {
        Self(query)
    }
}

#[async_trait]
impl<D> Execute for QueryCost<'_, D>
where
    Query<D>: QueryExecute,
    Query<D>: Execute,
    D: QueryData,
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

    fn requires_transaction_id() -> bool {
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
        channel: Channel,
        request: Self::GrpcRequest,
    ) -> Result<tonic::Response<Self::GrpcResponse>, tonic::Status> {
        <Query<D> as QueryExecute>::execute(channel, request).await
    }

    fn make_response(
        response: Self::GrpcResponse,
        _context: Self::Context,
        _node_account_id: AccountId,
        _transaction_id: Option<TransactionId>,
    ) -> crate::Result<Self::Response> {
        Ok(response_header(&response.response)?.cost)
    }

    fn response_pre_check_status(response: &Self::GrpcResponse) -> crate::Result<i32> {
        Ok(response_header(&response.response)?.node_transaction_precheck_code)
    }
}

impl<D> QueryCost<'_, D>
where
    Query<D>: QueryExecute + Send + Sync,
    D: QueryData,
{
    /// Execute this query against the provided client of the Hedera network.
    pub async fn execute(&mut self, client: &Client) -> crate::Result<u64> {
        execute(client, self).await
    }
}
