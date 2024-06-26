use std::net::SocketAddr;
use std::time::Duration;

use hyper::body::Bytes;
use jsonrpsee::core::{async_trait, client::ClientT};
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;
use jsonrpsee::server::{RpcModule, Server};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::types::ErrorObjectOwned;
use jsonrpsee::ConnectionDetails;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing_subscriber::util::SubscriberInitExt;
use hedera::PrivateKey;

#[rpc(server, client)]
pub trait Rpc {
    /// Raw method with connection ID.
    #[method(name = "connectionIdMethod", raw_method)]
    async fn raw_method(&self, first_param: usize, second_param: u16) -> Result<usize, ErrorObjectOwned>;

    #[method(name = "createAccount")]
    fn create_account(
        &self,
        publicKey: String,
        initialBalance: Option<i64>,
        receiverSignatureRequired: Option<bool>,
        maxAutomaticTokenAssociations: Option<u32>,
        stakedAccountId: Option<String>,
        stakedNodeId: Option<u64>,
        declineStakingReward: Option<bool>,
        accountMemo: Option<String>,
        // privateKey: Option<String>,
        // autoRenewPeriod: Option<String>
    ) -> Result<usize, ErrorObjectOwned>;

    #[method(name = "generatePublicKey")]
    fn generate_public_key(&self, privateKey: String) -> Result<usize, ErrorObjectOwned>;

    #[method(name = "generatePrivateKey")]
    fn generate_private_key(&self) -> Result<String, ErrorObjectOwned>;

    // generatePublicKey: ({privateKey}) => {
    // return PrivateKey.fromString(privateKey).publicKey.toString();
    // },
    // generatePrivateKey: () => {
    // return PrivateKey.generateED25519().toString();
    // }
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
    async fn raw_method(
        &self,
        connection_details: ConnectionDetails,
        _first_param: usize,
        _second_param: u16,
    ) -> Result<usize, ErrorObjectOwned> {
        // Return the connection ID from which this method was called.
        Ok(connection_details.id())
    }

    fn create_account(
        &self,
        _publicKey: String,
        _initialBalance: Option<i64>,
        _receiverSignatureRequired: Option<bool>,
        _maxAutomaticTokenAssociations: Option<u32>,
        _stakedAccountId: Option<String>,
        _stakedNodeId: Option<u64>,
        _declineStakingReward: Option<bool>,
        _accountMemo: Option<String>,
        // _privateKey: Option<String>,
        // _autoRenewPeriod: Option<String>
    ) -> Result<usize, ErrorObjectOwned> {
        // The normal method does not have access to the connection ID.
        println!("hello");
        println!("{}", _publicKey);

        Ok(usize::MAX)
    }

    fn generate_public_key(&self, privateKey: String) -> Result<usize, ErrorObjectOwned> {
        println!("{}", privateKey);

        Ok(usize::MAX)
    }

    fn generate_private_key(&self) -> Result<String, ErrorObjectOwned> {
        println!("begin privateKey generation");

        Ok("hello".parse().unwrap())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive("jsonrpsee[method_call{name = \"createAccount\"}]=trace".parse()?);
    tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;

    let server_addr = run_server().await?;
    let url = format!("http://{}", server_addr);

    let middleware = tower::ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .on_request(
                    |request: &hyper::Request<hyper::Body>, _span: &tracing::Span| tracing::info!(request = ?request, "on_request"),
                )
                .on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
                    tracing::info!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk")
                })
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(DefaultOnResponse::new().include_headers(true).latency_unit(LatencyUnit::Micros)),
        );

    // let client = HttpClientBuilder::default().set_http_middleware(middleware).build(url)?;
    // let params = rpc_params![1_u64, 2, 3];
    // let response: Result<String, _> = client.request("createAccount", params).await;
    // tracing::info!("r: {:?}", response);

    println!("Server is running at {}", url);

    loop {}

    Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
    let server = Server::builder().build("127.0.0.1:8085").await?;

    let addr = server.local_addr()?;
    let handle = server.start(RpcServerImpl.into_rpc());

    // In this example we don't care about doing shutdown so let's it run forever.
    // You may use the `ServerHandle` to shut it down or manage it yourself.
    tokio::spawn(handle.stopped());

    Ok(addr)
}