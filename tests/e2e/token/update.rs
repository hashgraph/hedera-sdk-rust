use assert_matches::assert_matches;
use hedera::{
    Hbar,
    Key,
    Status,
    TokenInfoQuery,
    TokenUpdateTransaction,
};

use super::FungibleToken;
use crate::account::Account;
use crate::common::{
    setup_nonfree,
    TestEnvironment,
};
use crate::token::{
    CreateFungibleToken,
    TokenKeys,
};

#[tokio::test]
async fn basic() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let account = Account::create(Hbar::new(0), &client).await?;
    let token = FungibleToken::create(
        &client,
        &account,
        CreateFungibleToken { initial_supply: 0, keys: TokenKeys::ALL_OWNER },
    )
    .await?;

    TokenUpdateTransaction::new()
        .token_id(token.id)
        .token_name("aaaa")
        .token_symbol("A")
        .sign(account.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let info = TokenInfoQuery::new().token_id(token.id).execute(&client).await?;

    assert_eq!(info.token_id, token.id);
    assert_eq!(info.name, "aaaa");
    assert_eq!(info.symbol, "A");
    assert_eq!(info.decimals, 3);
    assert_eq!(info.treasury_account_id, account.id);
    assert_eq!(info.admin_key, Some(Key::Single(account.key.public_key())));
    assert_eq!(info.freeze_key, Some(Key::Single(account.key.public_key())));
    assert_eq!(info.wipe_key, Some(Key::Single(account.key.public_key())));
    assert_eq!(info.kyc_key, Some(Key::Single(account.key.public_key())));
    assert_eq!(info.supply_key, Some(Key::Single(account.key.public_key())));
    assert_eq!(info.pause_key, Some(Key::Single(account.key.public_key())));
    assert_eq!(info.fee_schedule_key, Some(Key::Single(account.key.public_key())));
    assert_eq!(info.default_freeze_status, Some(false));
    assert_eq!(info.default_kyc_status, Some(false));

    token.delete(&client).await?;
    account.delete(&client).await?;

    Ok(())
}

#[tokio::test]

async fn immutable_token_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let account = Account::create(Hbar::new(0), &client).await?;

    let token = FungibleToken::create(
        &client,
        &account,
        CreateFungibleToken { initial_supply: 0, keys: TokenKeys::NONE },
    )
    .await?;

    let res = TokenUpdateTransaction::new()
        .token_id(token.id)
        .token_name("aaaa")
        .token_symbol("A")
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus { status: Status::TokenIsImmutable, transaction_id: _ })
    );

    // can't delete the account because the token still exists, can't delete the token because there's no admin key.

    Ok(())
}
