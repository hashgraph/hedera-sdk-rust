mod common;
use common::setup_global;
use hedera::{
    Client,
    TokenCreateTransaction,
    TokenType,
    TokenSupplyType,
    TokenId,
    AccountId,
    PublicKey,
    TokenInfo,
    TokenInfoQuery,
    Key,
};
use crate::common::TestEnvironment;
use time::Duration;

struct UsedData {
    name: String,
    token_type: TokenType,
    symbol: String,
    decimals: u32,
    initial_supply: u64,
    account_id: AccountId,
    public_key: PublicKey,
    custom_fees: u8,
    max_supply: u64,
    token_supply_type: TokenSupplyType,
    default_freeze_status: bool,
    auto_renew_period: Duration,
    memo: String,
}

#[tokio::test]
async fn create_token() {
    let TestEnvironment { config, client } = setup_global();

    let Some(operator) = &config.operator else {
        log::debug!("skipping test due to lack of operator");
        panic!("skipping test due to lack of operator");
    };

    if !config.run_nonfree_tests {
        log::debug!("skipping non-free test");
        panic!("skipping non-free test");
    }

    let used_data = UsedData {
        name: String::from("sdk::rust::e2e::TokenCreateTransaction"),
        token_type: TokenType::FungibleCommon,
        symbol: String::from("e2e"),
        decimals: 2,
        initial_supply: 10_000,
        account_id: operator.account_id.clone(),
        public_key: operator.private_key.clone().public_key(),
        custom_fees: 20,
        max_supply: 100_000,
        token_supply_type: TokenSupplyType::Finite,
        default_freeze_status: false,
        auto_renew_period: Duration::days(80),
        memo: String::from("testmemo"),
    };
    
    let token_id = create_new_token(&client, &used_data).await;

    let token_info = get_token_info(&client, token_id).await;

    assert_eq!(token_info.name, used_data.name, "On chain name does not match name from used_Data.");
    assert_eq!(token_info.token_type, used_data.token_type, "On chain token_type does not match token_type from used_Data.");
    assert_eq!(token_info.symbol, used_data.symbol, "On chain symbol does not match symbol from used_Data.");
    assert_eq!(token_info.decimals, used_data.decimals, "On chain decimals do not match decimals from used_Data.");
    assert_eq!(token_info.total_supply, used_data.initial_supply, "On chain total_supply does not match initial_supply from used_Data.");
    assert_eq!(token_info.treasury_account_id, used_data.account_id, "On chain treasury_account_id does not match account_id from used_Data.");
    assert_eq!(token_info.admin_key.unwrap(), Key::from(used_data.public_key), "On chain admin_key does not match public_key from used_Data.");
    assert_eq!(token_info.kyc_key.unwrap(), Key::from(used_data.public_key), "On chain kyc_key does not match public_key from used_Data.");
    assert_eq!(token_info.freeze_key.unwrap(), Key::from(used_data.public_key), "On chain freeze_key does not match public_key from used_Data.");
    assert_eq!(token_info.wipe_key.unwrap(), Key::from(used_data.public_key), "On chain wipe_key does not match public_key from used_Data.");
    assert_eq!(token_info.supply_key.unwrap(), Key::from(used_data.public_key), "On chain supply_key does not match public_key from used_Data.");
    assert_eq!(token_info.fee_schedule_key.unwrap(), Key::from(used_data.public_key), "On chain fee_schedule_key does not match public_key from used_Data.");
    assert_eq!(token_info.pause_key.unwrap(), Key::from(used_data.public_key), "On chain pause_key does not match public_key from used_Data.");
    //assert_eq!(token_info.custom_fees, used_data.custom_fees, "On chain custom_fees do not match custom_fees from used_Data.");
    assert_eq!(token_info.max_supply, used_data.max_supply, "On chain max_supply does not match max_supply from used_Data.");
    assert_eq!(token_info.supply_type, used_data.token_supply_type, "On chain token_supply_type does not match token_supply_type from used_Data.");
    assert_eq!(token_info.default_freeze_status.unwrap(), used_data.default_freeze_status, "On chain default_freeze_status does not match default_freeze_status from used_Data.");
    assert_eq!(token_info.auto_renew_account.unwrap(), used_data.account_id, "On chain auto_renew_account does not match account_id from used_Data.");
    assert_eq!(token_info.auto_renew_period.unwrap(), used_data.auto_renew_period, "On chain auto_renew_period does not match auto_renew_period from used_Data.");
    assert_eq!(token_info.token_memo, used_data.memo, "On chain token_memo does not match memo from used_Data.");

}


async fn create_new_token(client: &Client, used_data: &UsedData) -> TokenId {
    let token_create_tx = match TokenCreateTransaction::new()
        .name(&used_data.name)
        .token_type(used_data.token_type)
        .symbol(&used_data.symbol)
        .decimals(used_data.decimals)
        .initial_supply(used_data.initial_supply)
        .treasury_account_id(used_data.account_id)
        .admin_key(used_data.public_key.clone())
        .kyc_key(used_data.public_key.clone())
        .freeze_key(used_data.public_key.clone())
        .wipe_key(used_data.public_key.clone())
        .supply_key(used_data.public_key.clone())
        .fee_schedule_key(used_data.public_key.clone())
        .pause_key(used_data.public_key.clone())
        //TODO: Implement custom fees
        //.custom_fees(used_data.custom_fees)
        .max_supply(used_data.max_supply)
        .token_supply_type(used_data.token_supply_type)
        .freeze_default(used_data.default_freeze_status)
        .auto_renew_account_id(used_data.account_id)
        .auto_renew_period(used_data.auto_renew_period)
        .token_memo(&used_data.memo)
        .execute(client)
        .await {
            Ok(tx_response) => tx_response,
            Err(e) => panic!("Token Create Transaction failed with error: {}", e)
        };

        let receipt = match token_create_tx.get_receipt(client).await {
            Ok(receipt) => receipt,
            Err(e) => panic!("Transaction Receipt failed with error: {}", e)
        };

        let _ = match receipt.token_id {
            Some(token_id) => return token_id,
            None => panic!("Token Id retrieval failed")
        };
}

async fn get_token_info(client: &Client, token_id: TokenId) -> TokenInfo{
    let _ = match TokenInfoQuery::new()
        .token_id(token_id)
        .execute(client)
        .await {
            Ok(token_info) => return token_info,
            Err(e) => panic!("Token Info Query failed with error: {}", e)
        };
}