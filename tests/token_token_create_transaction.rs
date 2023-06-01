mod common;
use common::setup_global;
use hedera::{ Client, TokenCreateTransaction, TokenType, TokenSupplyType, TokenId, AccountId, PublicKey, TokenInfo, TokenInfoQuery, Key, KeyList, PrivateKey, Hbar, FractionalFee, FractionalFeeData, RoyaltyFee, RoyaltyFeeData, AnyCustomFee, FeeAssessmentMethod };
use crate::common::TestEnvironment;
use time::Duration;

enum PublicKeyType {
    PublicKey(PublicKey),
    KeyList(KeyList),
}

struct UsedData {
    name: String,
    token_type: TokenType,
    symbol: String,
    decimals: u32,
    initial_supply: u64,
    account_id: AccountId,
    public_key: PublicKeyType,
    private_keys: Option<Vec<PrivateKey>>,
    custom_fees: Vec<AnyCustomFee>,
    max_supply: Option<u64>,
    token_supply_type: TokenSupplyType,
    default_freeze_status: bool,
    auto_renew_period: Duration,
    memo: String,
}

#[tokio::test]
async fn test_create_fugible_token() {
    let TestEnvironment { config, client } = setup_global();

    let Some(operator) = &config.operator else {
        log::debug!("skipping test due to lack of operator");
        panic!("skipping test due to lack of operator");
    };

    if !config.run_nonfree_tests {
        log::debug!("skipping non-free test");
        panic!("skipping non-free test");
    }

    let fractional_fee = AnyCustomFee::from(FractionalFee {
        fee: FractionalFeeData{
            numerator: 1,
            denominator: 5,
            minimum_amount: 5,
            maximum_amount: 10,
            assessment_method: FeeAssessmentMethod::Inclusive,
        },
        fee_collector_account_id: Some(operator.account_id.clone()),
        all_collectors_are_exempt: false,
    });

    let used_data: UsedData = UsedData {
        name: String::from("sdk::rust::e2e::TokenCreateTransaction::1"),
        token_type: TokenType::FungibleCommon,
        symbol: String::from("e2e::ft"),
        decimals: 2,
        initial_supply: 10_000,
        account_id: operator.account_id.clone(),
        public_key: PublicKeyType::PublicKey(operator.private_key.clone().public_key()),
        private_keys: None,
        custom_fees: vec![fractional_fee],
        max_supply: None,
        token_supply_type: TokenSupplyType::Infinite,
        default_freeze_status: false,
        auto_renew_period: Duration::days(80),
        memo: String::from("sdk::rust::e2e::TokenCreateTransaction::1"),
    };
    
    let token_id = create_new_token(&client, &used_data).await;
    let token_info = get_token_info(&client, token_id).await;

    check_token_info(token_info, &used_data);
}

#[tokio::test]
async fn test_create_non_fungible_token() {
    let TestEnvironment { config, client } = setup_global();

    let Some(operator) = &config.operator else {
        log::debug!("skipping test due to lack of operator");
        panic!("skipping test due to lack of operator");
    };

    if !config.run_nonfree_tests {
        log::debug!("skipping non-free test");
        panic!("skipping non-free test");
    }

    let (private_keys, public_keys) = generate_key_list(Some(2));

    let royalty_fee = AnyCustomFee::from( RoyaltyFee {
        fee: RoyaltyFeeData {
            numerator: 1,
            denominator: 5,
            fallback_fee: None,
        },
        fee_collector_account_id: Some(operator.account_id.clone()),
        all_collectors_are_exempt: false,
    });

    let used_data: UsedData = UsedData {
        name: String::from("sdk::rust::e2e::TokenCreateTransaction::2"),
        token_type: TokenType::NonFungibleUnique,
        symbol: String::from("e2e::nft"),
        decimals: 0,
        initial_supply: 0,
        account_id: operator.account_id.clone(),
        public_key: PublicKeyType::KeyList(public_keys),
        private_keys: Some(private_keys),
        custom_fees: vec![royalty_fee],
        max_supply: Some(100),
        token_supply_type: TokenSupplyType::Finite,
        default_freeze_status: true,
        auto_renew_period: Duration::days(90),
        memo: String::from("sdk::rust::e2e::TokenCreateTransaction::2"),
    };
    
    let token_id = create_new_token(&client, &used_data).await;
    let token_info = get_token_info(&client, token_id).await;

    check_token_info(token_info, &used_data);

}

// Create new token according to predefined used_data
async fn create_new_token(client: &Client, used_data: &UsedData) -> TokenId {
    let mut token_create_tx = TokenCreateTransaction::new();

    token_create_tx
        .name(&used_data.name)
        .token_type(used_data.token_type)
        .symbol(&used_data.symbol)
        .decimals(used_data.decimals)
        .initial_supply(used_data.initial_supply)
        .treasury_account_id(used_data.account_id)
        .custom_fees(used_data.custom_fees.clone())
        .freeze_default(used_data.default_freeze_status)
        .auto_renew_account_id(used_data.account_id)
        .auto_renew_period(used_data.auto_renew_period)
        .token_memo(&used_data.memo)
        .token_supply_type(used_data.token_supply_type);

    // Add either one publickey (same as operator) or multiple keys with keylist
    // Sing transaction with the required number of keys if keylist was used
    match &used_data.public_key {
        PublicKeyType::PublicKey(key) => {
            token_create_tx
                .admin_key(key.clone())
                .kyc_key(key.clone())
                .freeze_key(key.clone())
                .wipe_key(key.clone())
                .supply_key(key.clone())
                .fee_schedule_key(key.clone())
                .pause_key(key.clone());
            ();
        },
        PublicKeyType::KeyList(keylist) => {
            token_create_tx
                .admin_key(keylist.clone())
                .kyc_key(keylist.clone())
                .freeze_key(keylist.clone())
                .wipe_key(keylist.clone())
                .supply_key(keylist.clone())
                .fee_schedule_key(keylist.clone())
                .pause_key(keylist.clone());

            // Signing of the transaction
            match &used_data.private_keys {
                    Some(private_keys) => {
                        match keylist.threshold {
                            // Sign only with the required number of Privatekey according to keylist threshold
                            Some(threshold) => {
                                for i in 0..threshold {
                                    token_create_tx.sign(private_keys[i as usize].clone());
                                }
                                ();
                            }
                            // Sign with all Privatekeys
                            None => {
                                for private_key in private_keys {
                                    token_create_tx.sign(private_key.clone());
                                }
                                ();
                            }
                        }
                    }
                    None => panic!("Keylist with public keys but no list with private keys to sign the transaction provided."),
                }
            ();
        }
    }
    
    // If token supply is finite a maximum supply must be set
    if used_data.token_supply_type == TokenSupplyType::Finite {
        match used_data.max_supply {
            Some(max_supply) => {
                token_create_tx.max_supply(max_supply);
                ();
            }
            None => panic!("Token Supply Type is finite but no max supply was provided."),
        }
    }

    // Execute transaction with max fee of 100 HBar to be sure it gets executed
    let token_create_response = match token_create_tx.max_transaction_fee(Hbar::new(100)).execute(client).await {
        Ok(tx_response) => tx_response,
        Err(e) => panic!("Token Create Transaction failed with error: {}", e)
    };

    // Get transaction receipt
    let receipt = match token_create_response.get_receipt(client).await {
        Ok(receipt) => receipt,
        Err(e) => panic!("Transaction Receipt failed with error: {}", e)
    };

    // Ectract and return only token id
    match receipt.token_id {
        Some(token_id) => return token_id,
        None => panic!("Token Id retrieval failed")
    };
}

// Fetch token data on chain with TokenInfo query
async fn get_token_info(client: &Client, token_id: TokenId) -> TokenInfo{
    let _ = match TokenInfoQuery::new()
        .token_id(token_id)
        .execute(client)
        .await {
            Ok(token_info) => return token_info,
            Err(e) => panic!("Token Info Query failed with error: {}", e)
        };
}

// Compare all data from the token onchain (token_info) with the previously used data (used_data)
fn check_token_info(token_info: TokenInfo, used_data: &UsedData) {
    assert_eq!(token_info.name, used_data.name, "On chain name does not match name from used_Data.");
    assert_eq!(token_info.token_type, used_data.token_type, "On chain token_type does not match token_type from used_Data.");
    assert_eq!(token_info.symbol, used_data.symbol, "On chain symbol does not match symbol from used_Data.");
    assert_eq!(token_info.decimals, used_data.decimals, "On chain decimals do not match decimals from used_Data.");
    assert_eq!(token_info.total_supply, used_data.initial_supply, "On chain total_supply does not match initial_supply from used_Data.");
    assert_eq!(token_info.treasury_account_id, used_data.account_id, "On chain treasury_account_id does not match account_id from used_Data.");
    assert_eq!(token_info.default_freeze_status.unwrap(), used_data.default_freeze_status, "On chain default_freeze_status does not match default_freeze_status from used_Data.");
    assert_eq!(token_info.auto_renew_account.unwrap(), used_data.account_id, "On chain auto_renew_account does not match account_id from used_Data.");
    assert_eq!(token_info.auto_renew_period.unwrap(), used_data.auto_renew_period, "On chain auto_renew_period does not match auto_renew_period from used_Data.");
    assert_eq!(token_info.token_memo, used_data.memo, "On chain token_memo does not match memo from used_Data.");
    assert_eq!(token_info.custom_fees, used_data.custom_fees, "On chain custom_fees do not match custom_fees from used_Data.");
    
    match &used_data.public_key {
        PublicKeyType::PublicKey(key) => {
            let key = Key::from(key.clone());
            assert_eq!(token_info.admin_key.unwrap(), key, "On chain admin_key does not match public_key from used_Data.");
            assert_eq!(token_info.kyc_key.unwrap(), key, "On chain kyc_key does not match public_key from used_Data.");
            assert_eq!(token_info.freeze_key.unwrap(), key, "On chain freeze_key does not match public_key from used_Data.");
            assert_eq!(token_info.wipe_key.unwrap(), key, "On chain wipe_key does not match public_key from used_Data.");
            assert_eq!(token_info.supply_key.unwrap(), key, "On chain supply_key does not match public_key from used_Data.");
            assert_eq!(token_info.fee_schedule_key.unwrap(), key, "On chain fee_schedule_key does not match public_key from used_Data.");
            assert_eq!(token_info.pause_key.unwrap(), key, "On chain pause_key does not match public_key from used_Data.");
            ();
        },
        PublicKeyType::KeyList(keylist) => {
            let keylist = Key::from(keylist.clone());
            assert_eq!(token_info.admin_key.unwrap(), keylist, "On chain admin_key does not match public_key from used_Data.");
            assert_eq!(token_info.kyc_key.unwrap(), keylist, "On chain kyc_key does not match public_key from used_Data.");
            assert_eq!(token_info.freeze_key.unwrap(), keylist, "On chain freeze_key does not match public_key from used_Data.");
            assert_eq!(token_info.wipe_key.unwrap(), keylist, "On chain wipe_key does not match public_key from used_Data.");
            assert_eq!(token_info.supply_key.unwrap(), keylist, "On chain supply_key does not match public_key from used_Data.");
            assert_eq!(token_info.fee_schedule_key.unwrap(), keylist, "On chain fee_schedule_key does not match public_key from used_Data.");
            assert_eq!(token_info.pause_key.unwrap(), keylist, "On chain pause_key does not match public_key from used_Data.");
            ();
        }
    }
    
    assert_eq!(token_info.supply_type, used_data.token_supply_type, "On chain token_supply_type does not match token_supply_type from used_Data.");
    if used_data.token_supply_type == TokenSupplyType::Finite  {
        match used_data.max_supply {
            Some(max_supply) => {
                assert_eq!(token_info.max_supply, max_supply, "On chain max_supply does not match max_supply from used_Data.");
                ();
            }
            None => panic!("Token Supply Type is finite but no max supply was provided."),
        }
    }
}

// Create new keylist and list with the according privatekeys
fn generate_key_list(key_threshold: Option<u32>) -> (Vec<PrivateKey>, KeyList) {
    let mut private_keys: Vec<PrivateKey> = vec![];
    let mut public_keys: Vec<Key> = vec![];
    let number_of_keys = match key_threshold {
        Some(threshold) => threshold + 1,
        None => 3,
    };
    for _ in 0..number_of_keys {
        let private = PrivateKey::generate_ed25519();
        private_keys.push(PrivateKey::from(private.clone()));
        public_keys.push(Key::from(private.public_key()));
    }
    let keylist = KeyList {
        keys: public_keys,
        threshold: key_threshold,
    };
    return (private_keys, keylist)
}
