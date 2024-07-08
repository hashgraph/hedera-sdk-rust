use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{
    Arc,
    Mutex,
};

use hedera::{
    AccountId,
    Client,
    PrivateKey,
};
use jsonrpsee::core::async_trait;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::types::{
    ErrorObject,
    ErrorObjectOwned,
};
use once_cell::sync::Lazy;

static GLOBAL_SDK_CLIENT: Lazy<Arc<Mutex<Option<Client>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

#[rpc(server, client)]
pub trait Rpc {
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

    #[method(name = "reset")]
    fn reset(&self) -> Result<HashMap<String, String>, ErrorObjectOwned>;
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

        // Client setup, if the network is not set, it will be created using testnet.
        // If the network is manually set, the network will be configured using the
        // provided ips and account id.
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
            return Err(ErrorObject::borrowed(-32603, "Missing operator account id", None));
        };

        let operator_key = if let Some(operator_private_key) = operator_private_key {
            PrivateKey::from_str(operator_private_key.as_str())
                .map_err(|e| ErrorObject::owned(-32603, e.to_string(), None::<()>))?
        } else {
            return Err(ErrorObject::borrowed(-32603, "Missing operator private key", None));
        };

        client.set_operator(operator_id, operator_key);

        let mut global_client = GLOBAL_SDK_CLIENT.lock().unwrap();
        *global_client = Some(client);

        Ok("SUCCESS".to_owned())
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

    fn reset(&self) -> Result<HashMap<String, String>, ErrorObjectOwned> {
        let mut global_client = GLOBAL_SDK_CLIENT.lock().unwrap();
        *global_client = None;
        Ok(HashMap::from([("status".to_string(), "SUCCESS".to_string())].to_owned()))
    }
}
