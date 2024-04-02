use assert_matches::assert_matches;
use hedera::{
    Hbar,
    Status,
    TokenAssociateTransaction,
    TokenDissociateTransaction,
};

use crate::account::Account;
use crate::common::{
    setup_nonfree,
    TestEnvironment,
};

#[tokio::test]
async fn basic() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let (alice, bob) = tokio::try_join!(
        Account::create(Hbar::new(5), &client),
        Account::create(Hbar::new(1), &client)
    )?;

    let token = super::FungibleToken::create(&client, &alice, Default::default()).await?;

    TokenAssociateTransaction::new()
        .account_id(bob.id)
        .token_ids([token.id])
        .sign(bob.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    TokenDissociateTransaction::new()
        .account_id(bob.id)
        .token_ids([token.id])
        .sign(bob.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    token.delete(&client).await?;

    tokio::try_join!(alice.delete(&client), bob.delete(&client))?;

    Ok(())
}

#[tokio::test]
async fn missing_token_id() -> anyhow::Result<()> {
    let Some(TestEnvironment { config, client }) = setup_nonfree() else {
        return Ok(());
    };

    let Some(op) = &config.operator else {
        log::debug!("skipping test due to lack of operator");
        return Ok(());
    };

    let res = TokenDissociateTransaction::new()
        .account_id(op.account_id)
        .freeze_with(&client)?
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus { status: Status::FailInvalid, transaction_id: _ })
    );

    Ok(())
}

#[tokio::test]
async fn missing_account_id_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let res = TokenDissociateTransaction::new().execute(&client).await?;

    let res = res.get_receipt(&client).await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidAccountId, transaction_id: _ })
    );

    Ok(())
}

#[tokio::test]
async fn missing_signature_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let account = Account::create(Hbar::new(0), &client).await?;

    let res = TokenDissociateTransaction::new()
        .account_id(account.id)
        .freeze_with(&client)?
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    account.delete(&client).await?;

    Ok(())
}

#[tokio::test]
async fn unassociated_token_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let (alice, bob) = tokio::try_join!(
        Account::create(Hbar::new(5), &client),
        Account::create(Hbar::new(1), &client)
    )?;

    let token = super::FungibleToken::create(&client, &alice, Default::default()).await?;

    let res = TokenDissociateTransaction::new()
        .account_id(bob.id)
        .token_ids([token.id])
        .sign(bob.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus {
            status: Status::TokenNotAssociatedToAccount,
            transaction_id: _
        })
    );

    token.delete(&client).await?;

    tokio::try_join!(alice.delete(&client), bob.delete(&client))?;

    Ok(())
}
