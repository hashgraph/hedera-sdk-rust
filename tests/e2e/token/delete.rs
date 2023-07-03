use assert_matches::assert_matches;
use hedera::{
    Hbar,
    Status,
    TokenCreateTransaction,
    TokenDeleteTransaction,
};
use time::{
    Duration,
    OffsetDateTime,
};

use crate::account::Account;
use crate::common::{
    setup_nonfree,
    TestEnvironment,
};

#[tokio::test]
async fn basic() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else { return Ok(()) };

    let account = Account::create(Hbar::new(0), &client).await?;
    let token = super::FungibleToken::create(&client, &account, 0).await?;

    TokenDeleteTransaction::new()
        .token_id(token.id)
        .sign(account.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    account.delete(&client).await?;

    Ok(())
}

#[tokio::test]
async fn only_admin_key() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else { return Ok(()) };

    let account = Account::create(Hbar::new(0), &client).await?;

    // n.b. `FungibleToken::create` sets all keys for convenience sake.
    let token_id = TokenCreateTransaction::new()
        .name("ffff")
        .symbol("F")
        .treasury_account_id(account.id)
        .admin_key(account.key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .sign(account.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    TokenDeleteTransaction::new()
        .token_id(token_id)
        .sign(account.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    account.delete(&client).await?;

    Ok(())
}

#[tokio::test]
async fn missing_admin_key_signature_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else { return Ok(()) };

    let account = Account::create(Hbar::new(0), &client).await?;
    let token = super::FungibleToken::create(&client, &account, 0).await?;

    let res = TokenDeleteTransaction::new()
        .token_id(token.id)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    token.delete(&client).await?;
    account.delete(&client).await?;

    Ok(())
}

#[tokio::test]
async fn missing_token_id_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else { return Ok(()) };

    let res = TokenDeleteTransaction::new().execute(&client).await;

    assert_matches!(
        res,
        Err(hedera::Error::TransactionPreCheckStatus {
            status: Status::InvalidTokenId,
            transaction_id: _
        })
    );

    Ok(())
}
