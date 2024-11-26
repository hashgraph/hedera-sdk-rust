use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{
    Arc,
    Mutex,
};

use hedera::{
    AccountCreateTransaction,
    AccountId,
    AccountUpdateTransaction,
    Client,
    EvmAddress,
    Hbar,
    PrivateKey,
};
use jsonrpsee::core::async_trait;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::types::error::INTERNAL_ERROR_CODE;
use jsonrpsee::types::{
    ErrorObject,
    ErrorObjectOwned,
};
use once_cell::sync::Lazy;
use serde_json::Value;
use time::{
    Duration,
    OffsetDateTime,
};

use crate::errors::from_hedera_error;
use crate::helpers::{
    fill_common_transaction_params,
    generate_key_helper,
    get_hedera_key,
};
use crate::responses::{
    AccountCreateResponse,
    AccountUpdateResponse,
    GenerateKeyResponse,
};

static GLOBAL_SDK_CLIENT: Lazy<Arc<Mutex<Option<Client>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

#[rpc(server, client)]
pub trait Rpc {
    /*
    / Specification:
    / https://github.com/hiero-ledger/hiero-sdk-tck/blob/main/test-specifications/utility.md#generateKey
    */
    #[method(name = "generateKey")]
    fn generate_key(
        &self,
        _type: String,
        from_key: Option<String>,
        threshold: Option<i32>,
        keys: Option<Value>,
    ) -> Result<GenerateKeyResponse, ErrorObjectOwned>;

    /*
    / Specification:
    / https://github.com/hiero-ledger/hiero-sdk-tck/blob/main/test-specifications/utility.md#setup
    */
    #[method(name = "setup")]
    fn setup(
        &self,
        operator_account_id: Option<String>,
        operator_private_key: Option<String>,
        node_ip: Option<String>,
        node_account_id: Option<String>,
        mirror_network_ip: Option<String>,
    ) -> Result<String, ErrorObjectOwned>;

    /*
    / Specification:
    / https://github.com/hiero-ledger/hiero-sdk-tck/blob/main/test-specifications/utility.md#reset
    */
    #[method(name = "reset")]
    fn reset(&self) -> Result<HashMap<String, String>, ErrorObjectOwned>;

    /*
    / Specification:
    / https://github.com/hiero-ledger/hiero-sdk-tck/blob/main/test-specifications/crypto-service/accountCreateTransaction.md#createAccount
    */
    #[method(name = "createAccount")]
    async fn create_account(
        &self,
        key: Option<String>,
        initial_balance: Option<i64>,
        receiver_signature_required: Option<bool>,
        auto_renew_period: Option<i64>,
        memo: Option<String>,
        max_auto_token_associations: Option<i64>,
        staked_account_id: Option<String>,
        staked_node_id: Option<i64>,
        decline_staking_reward: Option<bool>,
        alias: Option<String>,
        common_transaction_params: Option<HashMap<String, Value>>,
    ) -> Result<AccountCreateResponse, ErrorObjectOwned>;

    /*
    / Specification:
    / https://github.com/hiero-ledger/hiero-sdk-tck/blob/main/test-specifications/crypto-service/accountUpdateTransaction.md#updateAccount
    */
    #[method(name = "updateAccount")]
    async fn update_account(
        &self,
        account_id: Option<String>,
        key: Option<String>,
        auto_renew_period: Option<i64>,
        expiration_time: Option<i64>,
        receiver_signature_required: Option<bool>,
        memo: Option<String>,
        max_auto_token_associations: Option<i64>,
        staked_account_id: Option<String>,
        staked_node_id: Option<i64>,
        decline_staking_reward: Option<bool>,
        common_transaction_params: Option<HashMap<String, Value>>,
    ) -> Result<AccountUpdateResponse, ErrorObjectOwned>;
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
                let account_id = AccountId::from_str(node_account_id.as_str()).map_err(|e| {
                    ErrorObject::owned(INTERNAL_ERROR_CODE, e.to_string(), None::<()>)
                })?;
                network.insert(node_ip, account_id);

                let client = Client::for_network(network).map_err(|e| {
                    ErrorObject::owned(INTERNAL_ERROR_CODE, e.to_string(), None::<()>)
                })?;
                client.set_mirror_network([mirror_network_ip]);
                client
            }
            (None, None, None) => Client::for_testnet(),
            _ => {
                return Err(ErrorObject::borrowed(
                    INTERNAL_ERROR_CODE,
                    "Failed to setup client",
                    None,
                ))
            }
        };

        let operator_id = if let Some(operator_account_id) = operator_account_id {
            AccountId::from_str(operator_account_id.as_str())
                .map_err(|e| ErrorObject::owned(INTERNAL_ERROR_CODE, e.to_string(), None::<()>))?
        } else {
            return Err(ErrorObject::borrowed(
                INTERNAL_ERROR_CODE,
                "Missing operator account id",
                None,
            ));
        };

        let operator_key = if let Some(operator_private_key) = operator_private_key {
            PrivateKey::from_str(operator_private_key.as_str())
                .map_err(|e| ErrorObject::owned(INTERNAL_ERROR_CODE, e.to_string(), None::<()>))?
        } else {
            return Err(ErrorObject::borrowed(
                INTERNAL_ERROR_CODE,
                "Missing operator private key",
                None,
            ));
        };

        client.set_operator(operator_id, operator_key);

        let mut global_client = GLOBAL_SDK_CLIENT.lock().unwrap();
        *global_client = Some(client);

        Ok("SUCCESS".to_owned())
    }

    fn reset(&self) -> Result<HashMap<String, String>, ErrorObjectOwned> {
        let mut global_client = GLOBAL_SDK_CLIENT.lock().unwrap();
        *global_client = None;
        Ok(HashMap::from([("status".to_string(), "SUCCESS".to_string())].to_owned()))
    }

    fn generate_key(
        &self,
        _type: String,
        from_key: Option<String>,
        threshold: Option<i32>,
        keys: Option<Value>,
    ) -> Result<GenerateKeyResponse, ErrorObjectOwned> {
        let mut private_keys: Vec<Value> = Vec::new();

        let key = generate_key_helper(_type, from_key, threshold, keys, &mut private_keys, false)?;

        Ok(GenerateKeyResponse { key: key, private_keys: private_keys })
    }

    async fn create_account(
        &self,
        key: Option<String>,
        initial_balance: Option<i64>,
        receiver_signature_required: Option<bool>,
        auto_renew_period: Option<i64>,
        memo: Option<String>,
        max_auto_token_associations: Option<i64>,
        staked_account_id: Option<String>,
        staked_node_id: Option<i64>,
        decline_staking_reward: Option<bool>,
        alias: Option<String>,
        common_transaction_params: Option<HashMap<String, Value>>,
    ) -> Result<AccountCreateResponse, ErrorObjectOwned> {
        let client = {
            let guard = GLOBAL_SDK_CLIENT.lock().unwrap();
            guard
                .as_ref()
                .ok_or_else(|| {
                    ErrorObject::owned(
                        INTERNAL_ERROR_CODE,
                        "Client not initialized".to_string(),
                        None::<()>,
                    )
                })?
                .clone()
        };

        let mut account_create_tx = AccountCreateTransaction::new();

        if let Some(key) = key {
            let key = get_hedera_key(&key)?;

            account_create_tx.key(key);
        }

        if let Some(initial_balance) = initial_balance {
            account_create_tx.initial_balance(Hbar::from_tinybars(initial_balance));
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

        if let Some(max_auto_token_associations) = max_auto_token_associations {
            account_create_tx.max_automatic_token_associations(max_auto_token_associations as i32);
        }

        if let Some(staked_account_id) = staked_account_id {
            account_create_tx.staked_account_id(
                AccountId::from_str(&staked_account_id).map_err(|e| {
                    ErrorObject::owned(INTERNAL_ERROR_CODE, e.to_string(), None::<()>)
                })?,
            );
        }

        if let Some(alias) = alias {
            account_create_tx.alias(
                EvmAddress::from_str(&alias).map_err(|e| {
                    ErrorObject::owned(INTERNAL_ERROR_CODE, e.to_string(), None::<()>)
                })?,
            );
        }

        if let Some(staked_node_id) = staked_node_id {
            account_create_tx.staked_node_id(staked_node_id as u64);
        }

        if let Some(decline_staking_reward) = decline_staking_reward {
            account_create_tx.decline_staking_reward(decline_staking_reward);
        }

        if let Some(common_transaction_params) = common_transaction_params {
            let _ =
                fill_common_transaction_params(&mut account_create_tx, &common_transaction_params);

            account_create_tx.freeze_with(&client).unwrap();

            if let Some(signers) = common_transaction_params.get("signers") {
                if let Value::Array(signers) = signers {
                    for signer in signers {
                        if let Value::String(signer_str) = signer {
                            account_create_tx.sign(PrivateKey::from_str_der(signer_str).unwrap());
                        }
                    }
                }
            }
        }

        let tx_response =
            account_create_tx.execute(&client).await.map_err(|e| from_hedera_error(e))?;

        let tx_receipt =
            tx_response.get_receipt(&client).await.map_err(|e| from_hedera_error(e))?;

        Ok(AccountCreateResponse {
            account_id: tx_receipt.account_id.unwrap().to_string(),
            status: tx_receipt.status.as_str_name().to_string(),
        })
    }

    async fn update_account(
        &self,
        account_id: Option<String>,
        key: Option<String>,
        auto_renew_period: Option<i64>,
        expiration_time: Option<i64>,
        receiver_signature_required: Option<bool>,
        memo: Option<String>,
        max_auto_token_associations: Option<i64>,
        staked_account_id: Option<String>,
        staked_node_id: Option<i64>,
        decline_staking_reward: Option<bool>,
        common_transaction_params: Option<HashMap<String, Value>>,
    ) -> Result<AccountUpdateResponse, ErrorObjectOwned> {
        let client = {
            let guard = GLOBAL_SDK_CLIENT.lock().unwrap();
            guard
                .as_ref()
                .ok_or_else(|| {
                    ErrorObject::owned(
                        INTERNAL_ERROR_CODE,
                        "Client not initialized".to_string(),
                        None::<()>,
                    )
                })?
                .clone()
        };

        let mut account_update_tx = AccountUpdateTransaction::new();

        if let Some(account_id) = account_id {
            account_update_tx.account_id(account_id.parse().unwrap());
        }

        if let Some(key) = key {
            let key = get_hedera_key(&key)?;

            account_update_tx.key(key);
        }

        if let Some(receiver_signature_required) = receiver_signature_required {
            account_update_tx.receiver_signature_required(receiver_signature_required);
        }

        if let Some(auto_renew_period) = auto_renew_period {
            account_update_tx.auto_renew_period(Duration::seconds(auto_renew_period));
        }

        if let Some(expiration_time) = expiration_time {
            account_update_tx.expiration_time(
                OffsetDateTime::from_unix_timestamp(expiration_time).map_err(|e| {
                    ErrorObject::owned(INTERNAL_ERROR_CODE, e.to_string(), None::<()>)
                })?,
            );
        }

        if let Some(memo) = memo {
            account_update_tx.account_memo(memo);
        }

        if let Some(max_auto_token_associations) = max_auto_token_associations {
            account_update_tx.max_automatic_token_associations(max_auto_token_associations as i32);
        }

        if let Some(staked_account_id) = staked_account_id {
            account_update_tx.staked_account_id(
                AccountId::from_str(&staked_account_id).map_err(|e| {
                    ErrorObject::owned(INTERNAL_ERROR_CODE, e.to_string(), None::<()>)
                })?,
            );
        }

        if let Some(staked_node_id) = staked_node_id {
            account_update_tx.staked_node_id(staked_node_id as u64);
        }

        if let Some(decline_staking_reward) = decline_staking_reward {
            account_update_tx.decline_staking_reward(decline_staking_reward);
        }

        if let Some(common_transaction_params) = common_transaction_params {
            let _ =
                fill_common_transaction_params(&mut account_update_tx, &common_transaction_params);

            account_update_tx.freeze_with(&client).unwrap();

            if let Some(signers) = common_transaction_params.get("signers") {
                if let Value::Array(signers) = signers {
                    for signer in signers {
                        if let Value::String(signer_str) = signer {
                            account_update_tx.sign(PrivateKey::from_str_der(signer_str).unwrap());
                        }
                    }
                }
            }
        }

        let tx_response =
            account_update_tx.execute(&client).await.map_err(|e| from_hedera_error(e))?;

        let tx_receipt =
            tx_response.get_receipt(&client).await.map_err(|e| from_hedera_error(e))?;

        Ok(AccountUpdateResponse { status: tx_receipt.status.as_str_name().to_string() })
    }
}
