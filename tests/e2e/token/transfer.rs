use assert_matches::assert_matches;
use hedera::{
    FixedFee,
    FixedFeeData,
    Hbar,
    Status,
    TokenAssociateTransaction,
    TokenCreateTransaction,
    TokenGrantKycTransaction,
    TokenId,
    TokenWipeTransaction,
    TransferTransaction,
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
use crate::token::FungibleToken;

#[tokio::test]
async fn basic() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let (alice, bob) = tokio::try_join!(
        Account::create(Hbar::new(0), &client),
        Account::create(Hbar::new(0), &client)
    )?;

    let token = super::FungibleToken::create(&client, &alice, 10).await?;

    TokenAssociateTransaction::new()
        .account_id(bob.id)
        .token_ids([token.id])
        .freeze_with(&client)?
        .sign(bob.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    TokenGrantKycTransaction::new()
        .account_id(bob.id)
        .token_id(token.id)
        .sign(alice.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    TransferTransaction::new()
        .token_transfer(token.id, alice.id, -10)
        .token_transfer(token.id, bob.id, 10)
        .sign(alice.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    TransferTransaction::new()
        .token_transfer(token.id, bob.id, -10)
        .token_transfer(token.id, alice.id, 10)
        .sign(bob.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    token.burn(&client, 10).await?;

    token.delete(&client).await?;

    tokio::try_join!(alice.delete(&client), bob.delete(&client))?;

    Ok(())
}

#[tokio::test]
async fn insufficient_balance_for_fee_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let (alice, bob, cherry) = tokio::try_join!(
        Account::create(Hbar::new(0), &client),
        Account::create(Hbar::new(0), &client),
        Account::create(Hbar::new(0), &client),
    )?;

    let fee = FixedFee {
        all_collectors_are_exempt: true,
        fee_collector_account_id: Some(alice.id),
        fee: FixedFeeData {
            denominating_token_id: Some(TokenId::new(0, 0, 0)),
            amount: 5000_000_000,
        },
    };

    let token_id = TokenCreateTransaction::new()
        .name("ffff")
        .symbol("F")
        .initial_supply(1)
        .custom_fees([fee.into()])
        .treasury_account_id(alice.id)
        .freeze_default(false)
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .admin_key(alice.key.public_key())
        .wipe_key(alice.key.public_key())
        .fee_schedule_key(alice.key.public_key())
        .sign(alice.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token = FungibleToken { id: token_id, owner: alice.clone() };

    TokenAssociateTransaction::new()
        .account_id(bob.id)
        .token_ids([token.id])
        .freeze_with(&client)?
        .sign(bob.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    TokenAssociateTransaction::new()
        .account_id(cherry.id)
        .token_ids([token.id])
        .freeze_with(&client)?
        .sign(cherry.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    TransferTransaction::new()
        .token_transfer(token.id, alice.id, -1)
        .token_transfer(token.id, bob.id, 1)
        .sign(alice.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let res = TransferTransaction::new()
        .token_transfer(token.id, bob.id, -1)
        .token_transfer(token.id, cherry.id, 1)
        .sign(bob.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus {
            status: Status::InsufficientSenderAccountBalanceForCustomFee,
            transaction_id: _
        })
    );

    TokenWipeTransaction::new()
        .account_id(bob.id)
        .token_id(token.id)
        .amount(1_u64)
        .sign(alice.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    token.delete(&client).await?;

    tokio::try_join!(alice.delete(&client), bob.delete(&client), cherry.delete(&client))?;

    Ok(())
}

#[tokio::test]
async fn unowned_token_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let (alice, bob) = tokio::try_join!(
        Account::create(Hbar::new(0), &client),
        Account::create(Hbar::new(0), &client)
    )?;

    let token = super::FungibleToken::create(&client, &alice, 10).await?;

    TokenAssociateTransaction::new()
        .account_id(bob.id)
        .token_ids([token.id])
        .freeze_with(&client)?
        .sign(bob.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    TokenGrantKycTransaction::new()
        .account_id(bob.id)
        .token_id(token.id)
        .sign(alice.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    // notice the swapped direction
    let res = TransferTransaction::new()
        .token_transfer(token.id, bob.id, -10)
        .token_transfer(token.id, alice.id, 10)
        .sign(bob.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus {
            status: Status::InsufficientTokenBalance,
            transaction_id: _
        })
    );

    token.burn(&client, 10).await?;

    token.delete(&client).await?;

    tokio::try_join!(alice.delete(&client), bob.delete(&client))?;

    Ok(())
}
