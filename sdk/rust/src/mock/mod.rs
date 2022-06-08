use std::collections::HashMap;
use std::future::Future;
use tonic::{transport::Server, Request, Response, Status};
use hedera_proto::services::{self, crypto_service_server::CryptoServiceServer};
use std::sync::{Arc};
use futures_core::future::BoxFuture;
use tokio::sync::Mutex;
use tokio::task::JoinError;
use crate::{AccountId, Client};

macro_rules! response {
    ($self: ident, $request: ident) => {{
        use crate::mock::{MockRequest};

        let inner = $self.0.lock().await;
        let request = MockRequest::from($request.into_inner());
        let response = inner.responses[inner.index].into_value(request)
            .await
            .map_err(|err| tonic::Status::aborted("failed to create response"))?
            .into();
        Ok(Response::new(response))
    }}
}

mod crypto;

use crypto::MockCryptoService;


pub enum MockResponseValue {
    TransactionResponse(services::TransactionResponse),
    QueryResponse(services::Response),
}

impl Into<services::TransactionResponse> for MockResponseValue {
    fn into(self) -> services::TransactionResponse {
        match self {
            Self::TransactionResponse(value) => value,
            _ => panic!("expected transaction response, found query response"),
        }
    }
}

impl Into<services::Response> for MockResponseValue {
    fn into(self) -> services::Response {
        match self {
            Self::QueryResponse(value) => value,
            _ => panic!("expected query response, found transaction response"),
        }
    }
}

pub enum MockRequest {
    Query(services::Query),
    Transaction(services::Transaction),
}

impl From<services::Query> for MockRequest {
    fn from(value: services::Query) -> Self {
        Self::Query(value)
    }
}

impl From<services::Transaction> for MockRequest {
    fn from(value: services::Transaction) -> Self {
        Self::Transaction(value)
    }
}

#[tonic::async_trait]
pub trait AnyMockResponseInput: Send + Sync {
    async fn into_value(&self, request: MockRequest) -> anyhow::Result<MockResponseValue>;
}

#[tonic::async_trait]
impl AnyMockResponseInput for services::TransactionResponse {
    async fn into_value(&self, _: MockRequest) -> anyhow::Result<MockResponseValue> {
        Ok(MockResponseValue::TransactionResponse(self.clone()))
    }
}

#[tonic::async_trait]
impl AnyMockResponseInput for services::Response {
    async fn into_value(&self, _: MockRequest) -> anyhow::Result<MockResponseValue> {
        Ok(MockResponseValue::QueryResponse(self.clone()))
    }
}

#[tonic::async_trait]
impl AnyMockResponseInput for Box<fn(services::Query) -> BoxFuture<'static, anyhow::Result<MockResponseValue>>> {
    async fn into_value(&self, request: MockRequest) -> anyhow::Result<MockResponseValue> {
        match request {
            MockRequest::Query(value) => self(value).await,
            _ => panic!("expected query found transaction"),
        }
    }
}

#[tonic::async_trait]
impl AnyMockResponseInput for Box<fn(services::Transaction) -> BoxFuture<'static, anyhow::Result<MockResponseValue>>> {
    async fn into_value(&self, request: MockRequest) -> anyhow::Result<MockResponseValue> {
        match request {
            MockRequest::Transaction(value) => self(value).await,
            _ => panic!("expected query found transaction"),
        }
    }
}

pub(crate) struct MockService {
    index: usize,
    responses: Vec<Box<dyn AnyMockResponseInput>>,
}

impl MockService {
    fn new(responses: Vec<Box<dyn AnyMockResponseInput>>) -> Self {
        Self { index: 0, responses }
    }
}

impl From<services::Response> for Box<dyn AnyMockResponseInput> {
    fn from(value: services::Response) -> Self {
        Box::new(value)
    }
}

impl From<services::TransactionResponse> for Box<dyn AnyMockResponseInput> {
    fn from(value: services::TransactionResponse) -> Self {
        Box::new(value)
    }
}

// Since this only contains a client, should we even have a `Mocker` type?
pub(crate) struct Mocker {
    pub(crate) client: Client,
}

impl Mocker {
    pub async fn new(responses: Vec<Box<dyn AnyMockResponseInput>>) -> anyhow::Result<Mocker> {
        // FIXME: We should likely not be hardcoding the address here since
        // there can be multiple mocking tests running at the same time and hence
        // would cause this to fail. Go supports creating the server, and then getting
        // the port used from the server instead of declaring what the port should be up
        // front. This would be very useful here, but not sure if tonic supports it.
        let addr = "127.0.0.1:50211".parse()?;

        let service = Arc::new(Mutex::new(MockService::new(responses)));
        let crypto_service = CryptoServiceServer::new(MockCryptoService(service.clone()));

        // TODO: other services

        tokio::task::spawn(
            Server::builder()
            .add_service(crypto_service)
            .serve(addr)
        );

        let network = HashMap::from([
            (AccountId::from(3), Vec::from(["127.0.0.1".to_owned()])),
        ]);

        Ok(Mocker {
            client: Client::for_network(network),
        })
    }
}
