use tonic::{transport::Server, Request, Response, Status};
use hedera_proto::services::{self, crypto_service_server::CryptoServiceServer};
use std::sync::{Arc, Mutex};

macro_rules! response {
    ($self: ident) => {{
        let inner = $self.0.clone();
        let inner = inner.lock().unwrap();
        Ok(Response::new(inner.responses[inner.index].clone().into()))
    }}
}

mod crypto;

use crypto::MockCryptoService;


#[derive(Debug, Clone)]
pub(crate) enum MockResponse {
    TransactionResponse(services::TransactionResponse),
    QueryResponse(services::Response),
}

pub(crate) struct MockService {
    index: usize,
    responses: Vec<MockResponse>,
}

impl MockService {
    fn new(responses: Vec<MockResponse>) -> Self {
        Self { index: 0, responses }
    }
}

pub(crate) struct Mocker<'a> {
    // client: Client,
    server: futures::future::BoxFuture<'a, Result<(), tonic::transport::Error>>,
}

impl<'a> Mocker<'a> {
    pub async fn new(responses: Vec<MockResponse>) -> anyhow::Result<Mocker<'a>> {
        let addr = "[::1]:50051".parse()?;
        let service = Arc::new(Mutex::new(MockService::new(responses)));
        let crypto_service = CryptoServiceServer::new(MockCryptoService(service.clone()));
        // TODO: other services


        let server = Server::builder()
            .add_service(crypto_service)
            .serve(addr);

        Ok(Mocker {
            server: Box::pin(server),
        })
    }
}
