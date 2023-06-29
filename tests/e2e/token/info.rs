use assert_matches::assert_matches;
use hedera::{
    Hbar,
    Key,
    PrivateKey,
    Status,
    TokenCreateTransaction,
    TokenInfoQuery,
    TokenSupplyType,
    TokenType,
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
use crate::token::{
    FungibleToken,
    Nft,
};

#[tokio::test]

async fn query_all_different_keys() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let account = Account::create(Hbar::new(0), &client).await?;

    let key2 = PrivateKey::generate_ed25519();
    let key3 = PrivateKey::generate_ed25519();
    let key4 = PrivateKey::generate_ed25519();
    let key5 = PrivateKey::generate_ed25519();

    let token_id = TokenCreateTransaction::new()
        .name("ffff")
        .symbol("F")
        .decimals(3)
        .initial_supply(0)
        .treasury_account_id(account.id)
        .admin_key(account.key.public_key())
        .freeze_key(key2.public_key())
        .wipe_key(key3.public_key())
        .kyc_key(key4.public_key())
        .supply_key(key5.public_key())
        .freeze_default(false)
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .sign(account.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token = FungibleToken { id: token_id, owner: account.clone() };

    let info = TokenInfoQuery::new().token_id(token.id).execute(&client).await?;

    assert_eq!(info.token_id, token.id);
    assert_eq!(info.name, "ffff");
    assert_eq!(info.symbol, "F");
    assert_eq!(info.decimals, 3);
    assert_eq!(info.treasury_account_id, account.id);
    assert_eq!(info.admin_key, Some(Key::Single(account.key.public_key())));
    assert_eq!(info.freeze_key, Some(Key::Single(key2.public_key())));
    assert_eq!(info.wipe_key, Some(Key::Single(key3.public_key())));
    assert_eq!(info.kyc_key, Some(Key::Single(key4.public_key())));
    assert_eq!(info.supply_key, Some(Key::Single(key5.public_key())));
    assert_eq!(info.default_freeze_status, Some(false));
    assert_eq!(info.default_kyc_status, Some(false));
    assert_eq!(info.token_type, TokenType::FungibleCommon);
    assert_eq!(info.supply_type, TokenSupplyType::Infinite);

    token.delete(&client).await?;
    account.delete(&client).await?;

    Ok(())
}

#[tokio::test]

async fn query_minimal() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let account = Account::create(Hbar::new(0), &client).await?;

    let token_id = TokenCreateTransaction::new()
        .name("ffff")
        .symbol("F")
        .treasury_account_id(account.id)
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .sign(account.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(info.token_id, token_id);
    assert_eq!(info.name, "ffff");
    assert_eq!(info.symbol, "F");
    assert_eq!(info.decimals, 0);
    assert_eq!(info.treasury_account_id, account.id);
    assert_eq!(info.admin_key, None);
    assert_eq!(info.freeze_key, None);
    assert_eq!(info.wipe_key, None);
    assert_eq!(info.kyc_key, None);
    assert_eq!(info.supply_key, None);
    assert_eq!(info.default_freeze_status, None);
    assert_eq!(info.default_kyc_status, None);
    assert_eq!(info.token_type, TokenType::FungibleCommon);
    assert_eq!(info.supply_type, TokenSupplyType::Infinite);

    // we have to leave this account, for it is a treasury.
    // account.delete(&client).await?;

    Ok(())
}

#[tokio::test]

async fn query_nft() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let account = Account::create(Hbar::new(0), &client).await?;

    let token_id = TokenCreateTransaction::new()
        .name("ffff")
        .symbol("F")
        .treasury_account_id(account.id)
        .admin_key(account.key.public_key())
        .supply_key(account.key.public_key())
        .token_type(TokenType::NonFungibleUnique)
        .token_supply_type(TokenSupplyType::Finite)
        .max_supply(5000)
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .sign(account.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token = Nft { id: token_id, owner: account.clone() };

    let serials = token.mint_incremental(&client, 10).await?;

    assert_eq!(serials.len(), 10);

    let info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(info.token_id, token_id);
    assert_eq!(info.name, "ffff");
    assert_eq!(info.symbol, "F");
    assert_eq!(info.decimals, 0);
    assert_eq!(info.total_supply, 10);
    assert_eq!(info.treasury_account_id, account.id);
    assert_eq!(info.admin_key, Some(Key::Single(account.key.public_key())));
    assert_eq!(info.supply_key, Some(Key::Single(account.key.public_key())));
    assert_eq!(info.default_freeze_status, None);
    assert_eq!(info.default_kyc_status, None);
    assert_eq!(info.token_type, TokenType::NonFungibleUnique);
    assert_eq!(info.supply_type, TokenSupplyType::Finite);
    assert_eq!(info.max_supply, 5000);

    token.burn(&client, serials).await?;
    token.delete(&client).await?;
    account.delete(&client).await?;

    Ok(())
}

#[tokio::test]

async fn query_cost() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let account = Account::create(Hbar::new(0), &client).await?;
    let token = super::FungibleToken::create(&client, &account, 0).await?;

    let mut query = TokenInfoQuery::new();

    query.token_id(token.id);

    let cost = query.get_cost(&client).await?;

    query.payment_amount(cost).execute(&client).await?;

    token.delete(&client).await?;
    account.delete(&client).await?;

    Ok(())
}

#[tokio::test]

async fn query_cost_big_max() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let account = Account::create(Hbar::new(0), &client).await?;
    let token = super::FungibleToken::create(&client, &account, 0).await?;

    let mut query = TokenInfoQuery::new();

    query.token_id(token.id).max_payment_amount(Hbar::new(1000));

    let cost = query.get_cost(&client).await?;

    query.payment_amount(cost).execute(&client).await?;

    token.delete(&client).await?;
    account.delete(&client).await?;

    Ok(())
}

#[tokio::test]

async fn query_cost_small_max_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let account = Account::create(Hbar::new(0), &client).await?;
    let token = super::FungibleToken::create(&client, &account, 0).await?;

    let mut query = TokenInfoQuery::new();

    query.token_id(token.id).max_payment_amount(Hbar::from_tinybars(1));

    let cost = query.get_cost(&client).await?;

    let res = query.execute(&client).await;

    let (max_query_payment, query_cost) = assert_matches!(
        res,
        Err(hedera::Error::MaxQueryPaymentExceeded {
            max_query_payment,
            query_cost
        }) => (max_query_payment, query_cost)
    );

    assert_eq!(max_query_payment, Hbar::from_tinybars(1));
    // note: there's a very small chance this fails if the cost of a FileContentsQuery changes right when we execute it.
    assert_eq!(query_cost, cost);

    token.delete(&client).await?;
    account.delete(&client).await?;

    Ok(())
}

#[tokio::test]

async fn query_cost_insufficient_tx_fee_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let account = Account::create(Hbar::new(0), &client).await?;
    let token = super::FungibleToken::create(&client, &account, 0).await?;

    let res = TokenInfoQuery::new()
        .token_id(token.id)
        .max_payment_amount(Hbar::from_tinybars(10000))
        .payment_amount(Hbar::from_tinybars(1))
        .execute(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::QueryPaymentPreCheckStatus {
            status: Status::InsufficientTxFee,
            transaction_id: _
        })
    );

    token.delete(&client).await?;
    account.delete(&client).await?;

    Ok(())
}
