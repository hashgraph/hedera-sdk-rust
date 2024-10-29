use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{
    Arc,
    Mutex,
};

use hedera::{
    AccountCreateTransaction,
    AccountId,
    Client,
    EntityId,
    EvmAddress,
    Hbar,
    PrivateKey,
    Transaction,
    TransactionId,
};
use jsonrpsee::core::async_trait;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::types::error::PARSE_ERROR_CODE;
use jsonrpsee::types::{
    ErrorObject,
    ErrorObjectOwned,
};
use once_cell::sync::Lazy;
use time::Duration;

use crate::helpers::TransactionParamValue;

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

    #[method(name = "createAccount")]
    fn create_account(
        &self,
        key: Option<String>,
        initial_balance: Option<i64>,
        receiver_signature_required: Option<bool>,
        auto_renew_period: Option<i64>,
        memo: Option<String>,
        max_automatic_token_associations: Option<i64>,
        staked_account_id: Option<String>,
        alias: Option<String>,
        staked_node_id: Option<i64>,
        declining_staking_reward: Option<bool>,
        common_transaction_params: Option<HashMap<String, TransactionParamValue>>,
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

    fn create_account(
        &self,
        key: Option<String>,
        initial_balance: Option<i64>,
        receiver_signature_required: Option<bool>,
        auto_renew_period: Option<i64>,
        memo: Option<String>,
        max_automatic_token_associations: Option<i64>,
        staked_account_id: Option<String>,
        alias: Option<String>,
        staked_node_id: Option<i64>,
        decline_staking_reward: Option<bool>,
        common_transaction_params: Option<HashMap<String, TransactionParamValue>>,
    ) -> Result<String, ErrorObjectOwned> {
        let client = GLOBAL_SDK_CLIENT.lock().unwrap();
        let client = client.as_ref().ok_or_else(|| {
            ErrorObject::owned(-32603, "Client not initialized".to_string(), None::<()>)
        })?;

        let mut account_create_tx = AccountCreateTransaction::new();

        if let Some(key) = key {
            let private_key = PrivateKey::from_str(&key)
                .map_err(|e| ErrorObject::owned(-32603, e.to_string(), None::<()>))?;
            let public_key = private_key.public_key();
            account_create_tx.key(public_key);
        }

        if let Some(initial_balance) = initial_balance {
            account_create_tx.initial_balance(Hbar::new(initial_balance));
        }

        if let Some(receiver_signature_required) = receiver_signature_required {
            account_create_tx.receiver_signature_required(receiver_signature_required);
        }

        if let Some(auto_renew_period) = auto_renew_period {
            account_create_tx.auto_renew_period(Duration::seconds(auto_renew_period));
        }
        if let Some(memo) = memo {
            account_create_tx.account_memo(memo);
        }

        if let Some(max_automatic_token_associations) = max_automatic_token_associations {
            account_create_tx
                .max_automatic_token_associations(max_automatic_token_associations as i32);
        }

        if let Some(staked_account_id) = staked_account_id {
            account_create_tx.staked_account_id(AccountId::from_str(&staked_account_id).unwrap());
        }

        if let Some(alias) = alias {
            account_create_tx
                .alias(EvmAddress::from_str(&alias).map_err(|e| {
                    ErrorObject::owned(PARSE_ERROR_CODE, e.to_string(), None::<()>)
                })?);
        }

        if let Some(staked_node_id) = staked_node_id {
            account_create_tx.staked_node_id(staked_node_id as u64);
        }

        if let Some(decline_staking_reward) = decline_staking_reward {
            account_create_tx.decline_staking_reward(decline_staking_reward);
        }

        if let Some(common_transaction_params) = common_transaction_params {
            fill_common_transaction_params(
                client,
                &mut account_create_tx,
                &common_transaction_params,
            );

            account_create_tx.freeze_with(client).unwrap();

            if let Some(signers) = common_transaction_params.get("signers") {
                match signers {
                    TransactionParamValue::Array(signers) => {
                        for signer in signers {
                            account_create_tx.sign(PrivateKey::from_str_der(signer).unwrap());
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok("SUCCESS".to_owned())
    }
}

fn fill_common_transaction_params<D>(
    client: &Client,
    transaction: &mut Transaction<D>,
    common_transaction_params: &HashMap<String, TransactionParamValue>,
) -> Result<String, ErrorObjectOwned> {
    if let Some(transaction_id) = common_transaction_params.get("transactionId") {
        match transaction_id {
            TransactionParamValue::String(transaction_id) => {
                transaction
                    .transaction_id(TransactionId::from_str(transaction_id.as_str()).unwrap());
            }
            _ => {}
        }
    }

    if let Some(node_id) = common_transaction_params.get("nodeId") {
        match node_id {
            TransactionParamValue::String(node_id) => {
                transaction.node_account_ids([AccountId::from_str(&node_id.as_str()).unwrap()]);
            }
            _ => {}
        }
    }

    if let Some(max_fee) = common_transaction_params.get("maxTransactionFee") {
        match max_fee {
            TransactionParamValue::String(max_fee) => {
                transaction.max_transaction_fee(Hbar::from_tinybars(
                    max_fee.as_str().parse::<i64>().unwrap(),
                ));
            }
            _ => {}
        }
    }

    if let Some(transaction_valid_duration) =
        common_transaction_params.get("transactionValidDuration")
    {
        match transaction_valid_duration {
            TransactionParamValue::String(transaction_valid_duration) => {
                transaction.transaction_valid_duration(Duration::seconds(
                    transaction_valid_duration.as_str().parse::<i64>().unwrap(),
                ));
            }
            _ => {}
        }
    }

    if let Some(memo) = common_transaction_params.get("memo") {
        match memo {
            TransactionParamValue::String(memo) => {
                transaction.transaction_memo(memo.as_str());
            }
            _ => {}
        }
    }
}
