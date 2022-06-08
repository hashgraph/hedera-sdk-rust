use tonic::{transport::Server, Request, Response, Status};
use hedera_proto::services;
use std::sync::{Arc, Mutex};
use super::{MockService, MockResponse};

pub(crate) struct MockCryptoService(pub Arc<Mutex<MockService>>);

impl From<services::Response> for MockResponse {
    fn from(value: services::Response) -> Self {
        Self::QueryResponse(value)
    }
}

impl From<services::TransactionResponse> for MockResponse {
    fn from(value: services::TransactionResponse) -> Self {
        Self::TransactionResponse(value)
    }
}

impl Into<services::Response> for MockResponse {
    fn into(self) -> services::Response {
        match self {
            Self::QueryResponse(response) => response,
            _ => todo!(),
        }
    }
}

impl Into<services::TransactionResponse> for MockResponse {
    fn into(self) -> services::TransactionResponse {
        match self {
            Self::TransactionResponse(response) => response,
            _ => todo!(),
        }
    }
}

#[tonic::async_trait]
impl services::crypto_service_server::CryptoService for MockCryptoService {
    async fn create_account(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self)
    }

    async fn update_account(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self)
    }

    async fn crypto_transfer(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self)
    }

    async fn crypto_delete(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self)
    }

    async fn approve_allowances(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self)
    }

    async fn delete_allowances(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self)
    }

    async fn add_live_hash(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self)
    }

    async fn delete_live_hash(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self)
    }

    async fn get_live_hash(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self)
    }

    async fn get_account_records(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self)
    }

    async fn crypto_get_balance(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self)
    }

    async fn get_account_info(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self)
    }

    async fn get_transaction_receipts(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self)
    }

    async fn get_fast_transaction_record(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self)
    }

    async fn get_tx_record_by_tx_id(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self)
    }

    async fn get_stakers_by_account_id(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self)
    }
}

