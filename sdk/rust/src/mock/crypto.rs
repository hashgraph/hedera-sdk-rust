use tonic::{transport::Server, Request, Response, Status};
use hedera_proto::services;
use std::sync::Arc;
use tokio::sync::Mutex;
use super::MockService;

pub(crate) struct MockCryptoService(pub Arc<Mutex<MockService>>);

#[tonic::async_trait]
impl services::crypto_service_server::CryptoService for MockCryptoService {
    async fn create_account(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self, request)
    }

    async fn update_account(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self, request)
    }

    async fn crypto_transfer(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self, request)
    }

    async fn crypto_delete(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self, request)
    }

    async fn approve_allowances(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self, request)
    }

    async fn delete_allowances(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self, request)
    }

    async fn add_live_hash(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self, request)
    }

    async fn delete_live_hash(
        &self,
        request: Request<services::Transaction>,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        response!(self, request)
    }

    async fn get_live_hash(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self, request)
    }

    async fn get_account_records(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self, request)
    }

    async fn crypto_get_balance(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self, request)
    }

    async fn get_account_info(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self, request)
    }

    async fn get_transaction_receipts(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self, request)
    }

    async fn get_fast_transaction_record(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self, request)
    }

    async fn get_tx_record_by_tx_id(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self, request)
    }

    async fn get_stakers_by_account_id(
        &self,
        request: Request<services::Query>,
    ) -> Result<Response<services::Response>, Status> {
        response!(self, request)
    }
}

