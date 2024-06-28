use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::atomic::{
    AtomicUsize,
    Ordering,
};
use std::sync::{
    Arc,
    Mutex,
};
use std::time::Duration;

use futures_util::future::BoxFuture;
use hedera::{
    AccountId,
    Client,
    PrivateKey,
};
use hyper::body::Bytes;
use jsonrpsee::core::async_trait;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::middleware::rpc::{
    RpcService,
    RpcServiceT,
};
use jsonrpsee::server::{
    RpcServiceBuilder,
    Server,
};
use jsonrpsee::types::{
    ErrorObject,
    ErrorObjectOwned,
    Request,
};
use jsonrpsee::MethodResponse;
use tower_http::trace::{
    DefaultMakeSpan,
    DefaultOnResponse,
    TraceLayer,
};
use tower_http::LatencyUnit;
use tracing_subscriber::util::SubscriberInitExt;

pub(crate) mod errors;

use once_cell::sync::Lazy;

static GLOBAL_SDK_CLIENT: Lazy<Arc<Mutex<Option<Client>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

#[rpc(server, client)]
pub trait Rpc {
    #[method(name = "createAccount")]
    fn create_account(
        &self,
        public_key: Option<String>,
        initial_balance: Option<i64>,
        receiver_signature_required: Option<bool>,
        max_automatic_token_associations: Option<u32>,
        staked_account_id: Option<String>,
        staked_node_id: Option<u64>,
        decline_staking_reward: Option<bool>,
        account_memo: Option<String>,
        // privateKey: Option<String>,
        // autoRenewPeriod: Option<String>
    ) -> Result<usize, ErrorObjectOwned>;

    #[method(name = "generatePublicKey")]
    fn generate_public_key(&self, private_key: String) -> Result<String, ErrorObjectOwned>;

    #[method(name = "generatePrivateKey")]
    fn generate_private_key(&self) -> Result<String, ErrorObjectOwned>;

    #[method(name = "setup")]
    fn setup(
        &self,
        operator_account_id: Option<String>,
        operator_private_key: Option<String>,
        node_ip: Option<String>,
        node_account_id: Option<String>,
        mirror_network_ip: Option<String>,
    ) -> Result<String, ErrorObjectOwned>;
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
    fn setup(
        &self,
        operator_account_id: Option<String>,
        operator_private_key: Option<String>,
        node_ip: Option<String>,
        node_account_id: Option<String>,
        mirror_network_ip: Option<String>,
    ) -> Result<String, ErrorObjectOwned> {
        let mut network: HashMap<String, AccountId> = HashMap::new();

        let client = match (node_ip, node_account_id, mirror_network_ip) {
            (Some(node_ip), Some(node_account_id), Some(mirror_network_ip)) => {
                let account_id = AccountId::from_str(node_account_id.as_str())
                    .map_err(|e| ErrorObject::owned(-32603, e.to_string(), None::<()>))?;
                network.insert(node_ip, account_id);

                let client = Client::for_network(network)
                    .map_err(|e| ErrorObject::owned(-32603, e.to_string(), None::<()>))?;
                client.set_mirror_network([mirror_network_ip]);
                client
            }
            (None, None, None) => Client::for_testnet(),
            _ => return Err(ErrorObject::borrowed(-32603, "Failed to setup client", None)),
        };

        let operator_id = if let Some(operator_account_id) = operator_account_id {
            AccountId::from_str(operator_account_id.as_str())
                .map_err(|e| ErrorObject::owned(-32603, e.to_string(), None::<()>))?
        } else {
            return Err(ErrorObject::borrowed(-32603, "Invalid operator account id", None));
        };

        let operator_key = if let Some(operator_private_key) = operator_private_key {
            PrivateKey::from_str(operator_private_key.as_str())
                .map_err(|e| ErrorObject::owned(-32603, e.to_string(), None::<()>))?
        } else {
            return Err(ErrorObject::borrowed(-32603, "Invalid operator private key", None));
        };

        client.set_operator(operator_id, operator_key);

        let mut global_client = GLOBAL_SDK_CLIENT.lock().unwrap();
        *global_client = Some(client);

        Ok("Success".to_string())
    }

    fn create_account(
        &self,
        _public_key: Option<String>,
        _initial_balance: Option<i64>,
        _receiver_signature_required: Option<bool>,
        _max_automatic_token_associations: Option<u32>,
        _staked_account_id: Option<String>,
        _staked_node_id: Option<u64>,
        _decline_stakin_reward: Option<bool>,
        _account_memo: Option<String>,
        // _privateKey: Option<String>,
        // _autoRenewPeriod: Option<String>
    ) -> Result<usize, ErrorObjectOwned> {
        // The normal method does not have access to the connection ID.
        let mut client_guard = GLOBAL_SDK_CLIENT.lock().unwrap();

        Ok(usize::MAX)
    }

    fn generate_public_key(&self, private_key: String) -> Result<String, ErrorObjectOwned> {
        let private_key = private_key.trim_end();
        let key_type = PrivateKey::from_str(&private_key)
            .map_err(|e| ErrorObject::owned(-1, e.to_string(), None::<()>))?;

        let public_key = if key_type.is_ed25519() {
            PrivateKey::from_str_ed25519(&private_key)
                .map_err(|e| ErrorObject::owned(-1, e.to_string(), None::<()>))?
                .public_key()
                .to_string()
        } else if key_type.is_ecdsa() {
            PrivateKey::from_str_ecdsa(&private_key)
                .map_err(|e| ErrorObject::owned(-1, e.to_string(), None::<()>))?
                .public_key()
                .to_string()
        } else {
            return Err(ErrorObject::owned(
                -1,
                "Unsupported key type".to_string(),
                Some(private_key),
            ));
        };

        Ok(public_key)
    }

    fn generate_private_key(&self) -> Result<String, ErrorObjectOwned> {
        let private_key = PrivateKey::generate_ed25519().to_string();

        Ok(private_key)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive("jsonrpsee[method_call{name = \"createAccount\"}]=trace".parse()?);
    tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;

    let server_addr = run_server().await?;
    let url = format!("http://{}", server_addr);

    println!("Server is running at {}", url);

    loop {}

    Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
    let m = RpcServiceBuilder::new().layer_fn(move |service: RpcService| MyMiddleware {
        service,
        count: Arc::new(AtomicUsize::new(0)),
    });

    let server = Server::builder().set_rpc_middleware(m).build("127.0.0.1:80").await?;

    let addr = server.local_addr()?;
    let handle = server.start(RpcServerImpl.into_rpc());

    tokio::spawn(handle.stopped());

    Ok(addr)
}

#[derive(Clone)]
struct MyMiddleware<S> {
    service: S,
    count: Arc<AtomicUsize>,
}

impl<'a, S> RpcServiceT<'a> for MyMiddleware<S>
where
    S: RpcServiceT<'a> + Send + Sync + Clone + 'static,
{
    type Future = BoxFuture<'a, MethodResponse>;
    fn call(&self, req: Request<'a>) -> Self::Future {
        tracing::info!("MyMiddleware processed call {}", req.method);
        let count = self.count.clone();
        let service = self.service.clone();
        Box::pin(async move {
            let rp = service.call(req).await;
            // Modify the state.
            count.fetch_add(1, Ordering::Relaxed);
            rp
        })
    }
}
