use assert_matches::assert_matches;
use hedera::{
    Hbar,
    Status,
    TokenAssociateTransaction,
    TokenBurnTransaction,
    TokenCreateTransaction,
    TokenDeleteTransaction,
    TokenMintTransaction,
    TokenType,
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

#[tokio::test]
async fn basic() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
            return Ok(())
        };

    let account = Account::create(Hbar::new(0), &client).await?;

    let token_id = TokenCreateTransaction::new()
        .name("ffff")
        .symbol("F")
        .decimals(3)
        .initial_supply(10)
        .treasury_account_id(account.id)
        .admin_key(account.key.public_key())
        .supply_key(account.key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .freeze_default(false)
        .sign(account.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let receipt = TokenBurnTransaction::new()
        .amount(10_u64)
        .token_id(token_id)
        .sign(account.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    assert_eq!(receipt.total_supply, 0);

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
async fn missing_token_id_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
            return Ok(())
        };

    let res = TokenBurnTransaction::new().amount(10_u64).execute(&client).await;

    assert_matches!(
        res,
        Err(hedera::Error::TransactionPreCheckStatus {
            status: Status::InvalidTokenId,
            transaction_id: _
        })
    );

    Ok(())
}

#[tokio::test]
async fn burn_zero() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
            return Ok(())
        };

    let account = Account::create(Hbar::new(0), &client).await?;

    let token_id = TokenCreateTransaction::new()
        .name("ffff")
        .symbol("F")
        .decimals(3)
        .initial_supply(0)
        .treasury_account_id(account.id)
        .admin_key(account.key.public_key())
        .supply_key(account.key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .freeze_default(false)
        .sign(account.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let receipt = TokenBurnTransaction::new()
        .token_id(token_id)
        .sign(account.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    assert_eq!(receipt.total_supply, 0);

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
async fn missing_supply_key_sig_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
            return Ok(())
        };

    let account = Account::create(Hbar::new(0), &client).await?;

    let token_id = TokenCreateTransaction::new()
        .name("ffff")
        .symbol("F")
        .decimals(3)
        .initial_supply(0)
        .treasury_account_id(account.id)
        .admin_key(account.key.public_key())
        .supply_key(account.key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .freeze_default(false)
        .sign(account.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let res = TokenBurnTransaction::new()
        .token_id(token_id)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

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
async fn burn_nfts() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let account = Account::create(Hbar::new(0), &client).await?;

    let token_id = TokenCreateTransaction::new()
        .name("ffff")
        .symbol("F")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(account.id)
        .admin_key(account.key.public_key())
        .supply_key(account.key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .freeze_default(false)
        .sign(account.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let mint_receipt = TokenMintTransaction::new()
        .token_id(token_id)
        .metadata((0..10).map(|it| [it]))
        .sign(account.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    TokenBurnTransaction::new()
        .serials(mint_receipt.serials)
        .token_id(token_id)
        .sign(account.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

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
async fn unowned_nft_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let (alice, bob) = tokio::try_join!(
        Account::create(Hbar::new(0), &client),
        Account::create(Hbar::new(0), &client)
    )?;

    let token_id = TokenCreateTransaction::new()
        .name("ffff")
        .symbol("F")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(alice.id)
        .admin_key(alice.key.public_key())
        .supply_key(alice.key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .freeze_default(false)
        .sign(alice.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let serials = TokenMintTransaction::new()
        .token_id(token_id)
        .metadata((0..1).map(|it| [it]))
        .sign(alice.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .serials;

    let nft = token_id.nft(serials[0] as u64);

    TokenAssociateTransaction::new()
        .account_id(bob.id)
        .token_ids([token_id])
        .sign(bob.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    TransferTransaction::new()
        .nft_transfer(nft, alice.id, bob.id)
        .sign(alice.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let res = TokenBurnTransaction::new()
        .serials([nft.serial as i64])
        .token_id(token_id)
        .sign(alice.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus {
            status: Status::TreasuryMustOwnBurnedNft,
            transaction_id: _
        })
    );

    TransferTransaction::new()
        .nft_transfer(nft, bob.id, alice.id)
        .sign(bob.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    TokenBurnTransaction::new()
        .serials([nft.serial as i64])
        .token_id(token_id)
        .sign(alice.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    TokenDeleteTransaction::new()
        .token_id(token_id)
        .sign(alice.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    tokio::try_join!(alice.delete(&client), bob.delete(&client))?;

    Ok(())
}
