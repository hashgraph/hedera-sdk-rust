use assert_matches::assert_matches;
use hedera::{
    Hbar,
    Status,
    TokenDeleteTransaction,
};

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
async fn all_keys() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let account = Account::create(Hbar::new(0), &client).await?;
    let token = super::FungibleToken::create(
        &client,
        &account,
        CreateFungibleToken { initial_supply: 0, keys: TokenKeys::ALL_OWNER },
    )
    .await?;

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
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let account = Account::create(Hbar::new(0), &client).await?;

    let token = super::FungibleToken::create(&client, &account, Default::default()).await?;

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
async fn missing_admin_key_signature_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let account = Account::create(Hbar::new(0), &client).await?;
    let token = super::FungibleToken::create(&client, &account, Default::default()).await?;

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
async fn missing_admin_key_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };
    let account = Account::create(Hbar::new(0), &client).await?;

    let token = super::FungibleToken::create(
        &client,
        &account,
        CreateFungibleToken { initial_supply: 0, keys: TokenKeys::NONE },
    )
    .await?;

    let res = TokenDeleteTransaction::new()
        .token_id(token.id)
        .sign(token.owner.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus { status: Status::TokenIsImmutable, transaction_id: _ })
    );

    Ok(())
}

#[tokio::test]
async fn missing_token_id_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

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
