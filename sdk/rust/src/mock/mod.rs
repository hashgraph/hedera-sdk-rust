use std::collections::HashMap;
use tonic::{transport::Server, Request, Response, Status};
use hedera_proto::services::{self, crypto_service_server::CryptoServiceServer};
use std::sync::{Arc, Mutex};
use tokio::task::JoinError;
use crate::{AccountId, Client};

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

pub(crate) struct Mocker {
    pub(crate) client: Client,
}

impl Mocker {
    pub async fn new(responses: Vec<MockResponse>) -> anyhow::Result<Mocker> {
        // FIXME: We should likely not be hardcoding the address here since
        // there can be multiple mocking tests running at the same time and hence
        // would cause this to fail
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
