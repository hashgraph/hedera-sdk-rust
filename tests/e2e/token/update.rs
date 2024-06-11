use assert_matches::assert_matches;
use hedera::{
    Hbar,
    Key,
    KeyList,
    PrivateKey,
    PublicKey,
    Status,
    TokenCreateTransaction,
    TokenInfoQuery,
    TokenKeyValidation,
    TokenType,
    TokenUpdateTransaction,
};
use time::{
    Duration,
    OffsetDateTime,
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

#[tokio::test]
async fn update_immutable_token_metadata() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };
    let initial_metadata = vec![1];
    let updated_metadata = vec![1, 2];
    let metadata_key = PrivateKey::generate_ed25519();

    // Create the Fungible Token with metadata key.
    let token_id = TokenCreateTransaction::new()
        .name("ffff")
        .symbol("F")
        .token_type(TokenType::FungibleCommon)
        .decimals(3)
        .initial_supply(100000)
        .metadata(initial_metadata.clone())
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .admin_key(client.get_operator_public_key().unwrap())
        .metadata_key(metadata_key.public_key())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(&token_info.metadata, &initial_metadata);
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    // Update token with metadata key.
    _ = TokenUpdateTransaction::new()
        .token_id(token_id)
        .metadata(updated_metadata.clone())
        .freeze_with(&client)?
        .sign(metadata_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.metadata, updated_metadata);

    Ok(())
}

// HIP-540 (https://hips.hedera.com/hip/hip-540)
// Make a token immutable when updating keys to an empty KeyList, signing with an Admin Key,
// and setting the key verification mode to NO_VALIDATION
#[tokio::test]
async fn update_immutable_token_keys_with_admin_sig() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    // Admin, Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let admin_key = PrivateKey::generate_ed25519();
    let freeze_key = PrivateKey::generate_ed25519();
    let wipe_key = PrivateKey::generate_ed25519();
    let kyc_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();
    let pause_key = PrivateKey::generate_ed25519();
    let fee_schedule_key = PrivateKey::generate_ed25519();
    let metadata_key = PrivateKey::generate_ed25519();

    // Create the NFT with all keys.
    let token_id = TokenCreateTransaction::new()
        .name("Test NFT")
        .symbol("TNFT")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .admin_key(admin_key.public_key())
        .freeze_key(freeze_key.public_key())
        .wipe_key(wipe_key.public_key())
        .kyc_key(kyc_key.public_key())
        .supply_key(supply_key.public_key())
        .pause_key(pause_key.public_key())
        .fee_schedule_key(fee_schedule_key.public_key())
        .metadata_key(metadata_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .freeze_with(&client)?
        .sign(admin_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.admin_key, Some(Key::Single(admin_key.public_key())));
    assert_eq!(token_info.freeze_key, Some(Key::Single(freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    let empty_keylist = KeyList::new();

    // Update all lower-privilege keys for token with empty key list,
    // signing with admin key, and verifying with no validation.
    _ = TokenUpdateTransaction::new()
        .token_id(token_id)
        .admin_key(empty_keylist.clone())
        .wipe_key(empty_keylist.clone())
        .freeze_key(empty_keylist.clone())
        .kyc_key(empty_keylist.clone())
        .supply_key(empty_keylist.clone())
        .pause_key(empty_keylist.clone())
        .fee_schedule_key(empty_keylist.clone())
        .metadata_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .freeze_with(&client)?
        .sign(admin_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.admin_key, None);
    assert_eq!(token_info.freeze_key, None);
    assert_eq!(token_info.wipe_key, None);
    assert_eq!(token_info.kyc_key, None);
    assert_eq!(token_info.supply_key, None);
    assert_eq!(token_info.pause_key, None);
    assert_eq!(token_info.fee_schedule_key, None);
    assert_eq!(token_info.metadata_key, None);

    Ok(())
}

// HIP-540 (https://hips.hedera.com/hip/hip-540)
// Can remove all of token’s lower-privilege keys when updating keys to an empty KeyList,
// signing with an Admin Key, and setting the key verification mode to FULL_VALIDATION
#[tokio::test]
async fn remove_token_keys_with_admin_sig() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    // Admin, Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let admin_key = PrivateKey::generate_ed25519();
    let freeze_key = PrivateKey::generate_ed25519();
    let wipe_key = PrivateKey::generate_ed25519();
    let kyc_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();
    let pause_key = PrivateKey::generate_ed25519();
    let fee_schedule_key = PrivateKey::generate_ed25519();
    let metadata_key = PrivateKey::generate_ed25519();

    // Create the NFT with all keys.
    let token_id = TokenCreateTransaction::new()
        .name("Test NFT")
        .symbol("TNFT")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .admin_key(admin_key.public_key())
        .freeze_key(freeze_key.public_key())
        .wipe_key(wipe_key.public_key())
        .kyc_key(kyc_key.public_key())
        .supply_key(supply_key.public_key())
        .pause_key(pause_key.public_key())
        .fee_schedule_key(fee_schedule_key.public_key())
        .metadata_key(metadata_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .freeze_with(&client)?
        .sign(admin_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.admin_key, Some(Key::Single(admin_key.public_key())));
    assert_eq!(token_info.freeze_key, Some(Key::Single(freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    let empty_keylist = KeyList::new();

    // Update all lower-privilege keys for token with empty key list,
    // signing with admin key, and verifying with full validation.
    _ = TokenUpdateTransaction::new()
        .token_id(token_id)
        .admin_key(empty_keylist.clone())
        .wipe_key(empty_keylist.clone())
        .freeze_key(empty_keylist.clone())
        .kyc_key(empty_keylist.clone())
        .supply_key(empty_keylist.clone())
        .pause_key(empty_keylist.clone())
        .fee_schedule_key(empty_keylist.clone())
        .metadata_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(admin_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.admin_key, None);
    assert_eq!(token_info.freeze_key, None);
    assert_eq!(token_info.wipe_key, None);
    assert_eq!(token_info.kyc_key, None);
    assert_eq!(token_info.supply_key, None);
    assert_eq!(token_info.pause_key, None);
    assert_eq!(token_info.fee_schedule_key, None);
    assert_eq!(token_info.metadata_key, None);

    Ok(())
}

// HIP-540 (https://hips.hedera.com/hip/hip-540)
// Can update all of token’s lower-privilege keys to an unusable key (i.e. all-zeros key),
// when signing with an Admin Key, and setting the key verification mode to FULL_VALIDATION, and then revert previous keys
#[tokio::test]
async fn revert_token_keys_with_admin_sig() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    // Admin, Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let admin_key = PrivateKey::generate_ed25519();
    let freeze_key = PrivateKey::generate_ed25519();
    let wipe_key = PrivateKey::generate_ed25519();
    let kyc_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();
    let pause_key = PrivateKey::generate_ed25519();
    let fee_schedule_key = PrivateKey::generate_ed25519();
    let metadata_key = PrivateKey::generate_ed25519();

    // Create the NFT with all keys.
    let token_id = TokenCreateTransaction::new()
        .name("Test NFT")
        .symbol("TNFT")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .admin_key(admin_key.public_key())
        .freeze_key(freeze_key.public_key())
        .wipe_key(wipe_key.public_key())
        .kyc_key(kyc_key.public_key())
        .supply_key(supply_key.public_key())
        .pause_key(pause_key.public_key())
        .fee_schedule_key(fee_schedule_key.public_key())
        .metadata_key(metadata_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .freeze_with(&client)?
        .sign(admin_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.admin_key, Some(Key::Single(admin_key.public_key())));
    assert_eq!(token_info.freeze_key, Some(Key::Single(freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    // Generate an unusable key to update the supply key.
    let unusable_key = PublicKey::from_str_ed25519(
        "0x0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();

    // Update all lower-privilege keys for token with invalid zeros key,
    // signing with admin key, and verifying with full validation.
    _ = TokenUpdateTransaction::new()
        .token_id(token_id)
        .wipe_key(unusable_key)
        .freeze_key(unusable_key)
        .kyc_key(unusable_key)
        .supply_key(unusable_key)
        .pause_key(unusable_key)
        .fee_schedule_key(unusable_key)
        .metadata_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(admin_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.freeze_key, Some(Key::Single(unusable_key)));
    assert_eq!(token_info.wipe_key, Some(Key::Single(unusable_key)));
    assert_eq!(token_info.kyc_key, Some(Key::Single(unusable_key)));
    assert_eq!(token_info.supply_key, Some(Key::Single(unusable_key)));
    assert_eq!(token_info.pause_key, Some(Key::Single(unusable_key)));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(unusable_key)));
    assert_eq!(token_info.metadata_key, Some(Key::Single(unusable_key)));

    // Set all lower-privilege keys back to their original values,
    // signing with admin key, and verifying with no validation.
    _ = TokenUpdateTransaction::new()
        .token_id(token_id)
        .wipe_key(wipe_key.public_key())
        .freeze_key(freeze_key.public_key())
        .kyc_key(kyc_key.public_key())
        .supply_key(supply_key.public_key())
        .pause_key(pause_key.public_key())
        .fee_schedule_key(fee_schedule_key.public_key())
        .metadata_key(metadata_key.public_key())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .freeze_with(&client)?
        .sign(admin_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.admin_key, Some(Key::Single(admin_key.public_key())));
    assert_eq!(token_info.freeze_key, Some(Key::Single(freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    Ok(())
}

// HIP-540 (https://hips.hedera.com/hip/hip-540)
// Can update all of token’s lower-privilege keys when signing with an Admin Key
// and new respective lower-privilege key, and setting key verification mode to FULL_VALIDATION
#[tokio::test]
async fn update_token_new_keys_with_admin_sig() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    // Admin, Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let admin_key = PrivateKey::generate_ed25519();
    let freeze_key = PrivateKey::generate_ed25519();
    let wipe_key = PrivateKey::generate_ed25519();
    let kyc_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();
    let pause_key = PrivateKey::generate_ed25519();
    let fee_schedule_key = PrivateKey::generate_ed25519();
    let metadata_key = PrivateKey::generate_ed25519();

    // New Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let new_freeze_key = PrivateKey::generate_ed25519();
    let new_wipe_key = PrivateKey::generate_ed25519();
    let new_kyc_key = PrivateKey::generate_ed25519();
    let new_supply_key = PrivateKey::generate_ed25519();
    let new_pause_key = PrivateKey::generate_ed25519();
    let new_fee_schedule_key = PrivateKey::generate_ed25519();
    let new_metadata_key = PrivateKey::generate_ed25519();

    // Create the NFT with all keys.
    let token_id = TokenCreateTransaction::new()
        .name("Test NFT")
        .symbol("TNFT")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .admin_key(admin_key.public_key())
        .freeze_key(freeze_key.public_key())
        .wipe_key(wipe_key.public_key())
        .kyc_key(kyc_key.public_key())
        .supply_key(supply_key.public_key())
        .pause_key(pause_key.public_key())
        .fee_schedule_key(fee_schedule_key.public_key())
        .metadata_key(metadata_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .freeze_with(&client)?
        .sign(admin_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.admin_key, Some(Key::Single(admin_key.public_key())));
    assert_eq!(token_info.freeze_key, Some(Key::Single(freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    // Update all lower-privilege keys for token with new lower-privilege keys,
    // signing with admin key and new lower-privilege keys, and verifying with full validation.
    _ = TokenUpdateTransaction::new()
        .token_id(token_id)
        .wipe_key(new_wipe_key.public_key())
        .freeze_key(new_freeze_key.public_key())
        .kyc_key(new_kyc_key.public_key())
        .supply_key(new_supply_key.public_key())
        .pause_key(new_pause_key.public_key())
        .fee_schedule_key(new_fee_schedule_key.public_key())
        .metadata_key(new_metadata_key.public_key())
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(admin_key.clone())
        .sign(new_wipe_key.clone())
        .sign(new_freeze_key.clone())
        .sign(new_kyc_key.clone())
        .sign(new_supply_key.clone())
        .sign(new_pause_key.clone())
        .sign(new_fee_schedule_key.clone())
        .sign(new_metadata_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.freeze_key, Some(Key::Single(new_freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(new_wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(new_kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(new_supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(new_pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(new_fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(new_metadata_key.public_key())));

    Ok(())
}

// HIP-540 (https://hips.hedera.com/hip/hip-540)
// Cannot make a token immutable when updating keys to an empty KeyList,
// signing with a key that is different from an Admin Key, and setting the key verification mode to NO_VALIDATION
#[tokio::test]
async fn update_keys_empty_keylist_without_admin_sig_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    // Admin, Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let admin_key = PrivateKey::generate_ed25519();
    let freeze_key = PrivateKey::generate_ed25519();
    let wipe_key = PrivateKey::generate_ed25519();
    let kyc_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();
    let pause_key = PrivateKey::generate_ed25519();
    let fee_schedule_key = PrivateKey::generate_ed25519();
    let metadata_key = PrivateKey::generate_ed25519();

    // Create the NFT with all keys.
    let token_id = TokenCreateTransaction::new()
        .name("Test NFT")
        .symbol("TNFT")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .admin_key(admin_key.public_key())
        .freeze_key(freeze_key.public_key())
        .wipe_key(wipe_key.public_key())
        .kyc_key(kyc_key.public_key())
        .supply_key(supply_key.public_key())
        .pause_key(pause_key.public_key())
        .fee_schedule_key(fee_schedule_key.public_key())
        .metadata_key(metadata_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .freeze_with(&client)?
        .sign(admin_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.admin_key, Some(Key::Single(admin_key.public_key())));
    assert_eq!(token_info.freeze_key, Some(Key::Single(freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    let empty_keylist = KeyList::new();

    // Fails to update the immutable token keys to empty keylist without admin signature (sign implicitly with operator key).
    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .wipe_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .kyc_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .freeze_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .pause_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .supply_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .fee_schedule_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .metadata_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .admin_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    Ok(())
}

// HIP-540 (https://hips.hedera.com/hip/hip-540)
// Cannot make a token immutable when updating keys to an unusable key (i.e. all-zeros key),
// signing with a key that is different from an Admin Key, and setting the key verification mode to NO_VALIDATION
#[tokio::test]
async fn update_keys_unusable_key_without_admin_sig_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    // Admin, Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let admin_key = PrivateKey::generate_ed25519();
    let freeze_key = PrivateKey::generate_ed25519();
    let wipe_key = PrivateKey::generate_ed25519();
    let kyc_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();
    let pause_key = PrivateKey::generate_ed25519();
    let fee_schedule_key = PrivateKey::generate_ed25519();
    let metadata_key = PrivateKey::generate_ed25519();

    // Create the NFT with all keys.
    let token_id = TokenCreateTransaction::new()
        .name("Test NFT")
        .symbol("TNFT")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .admin_key(admin_key.public_key())
        .freeze_key(freeze_key.public_key())
        .wipe_key(wipe_key.public_key())
        .kyc_key(kyc_key.public_key())
        .supply_key(supply_key.public_key())
        .pause_key(pause_key.public_key())
        .fee_schedule_key(fee_schedule_key.public_key())
        .metadata_key(metadata_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .freeze_with(&client)?
        .sign(admin_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.admin_key, Some(Key::Single(admin_key.public_key())));
    assert_eq!(token_info.freeze_key, Some(Key::Single(freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    // Generate an unusable key.
    let unusable_key = PublicKey::from_str_ed25519(
        "0x0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();

    // Fails to update the immutable token keys to unusable key  without admin signature (sign implicitly with operator key).
    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .wipe_key(unusable_key.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .kyc_key(unusable_key.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .freeze_key(unusable_key.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .pause_key(unusable_key.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .supply_key(unusable_key.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .fee_schedule_key(unusable_key.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .metadata_key(unusable_key.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .admin_key(unusable_key.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    Ok(())
}

// HIP-540 (https://hips.hedera.com/hip/hip-540)
// Cannot update the Admin Key to an unusable key (i.e. all-zeros key),
// signing with an Admin Key, and setting the key verification mode to NO_VALIDATION
#[tokio::test]
async fn update_admin_key_to_usuable_key_fail() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    // Admin and Freeze keys
    let admin_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();

    // Create the NFT with admin and supply key.
    let token_id = TokenCreateTransaction::new()
        .name("Test NFT")
        .symbol("TNFT")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .admin_key(admin_key.public_key())
        .supply_key(supply_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .freeze_with(&client)?
        .sign(admin_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.admin_key, Some(Key::Single(admin_key.public_key())));

    // Generate an unusable key.
    let unusable_key = PublicKey::from_str_ed25519(
        "0x0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();

    // Update the Admin Key to an unusable key (i.e., all-zeros key),
    // signing with an Admin Key, and setting the key verification mode to NO_VALIDATION
    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .admin_key(unusable_key.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .freeze_with(&client)?
        .sign(admin_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    Ok(())
}

// HIP-540 (https://hips.hedera.com/hip/hip-540)
// Can update all of token’s lower-privilege keys to an unusable key (i.e. all-zeros key),
// when signing with a respective lower-privilege key, and setting the key verification mode to NO_VALIDATION
#[tokio::test]
async fn update_keys_with_lower_privilege_keys_sigs() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    // Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let freeze_key = PrivateKey::generate_ed25519();
    let wipe_key = PrivateKey::generate_ed25519();
    let kyc_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();
    let pause_key = PrivateKey::generate_ed25519();
    let fee_schedule_key = PrivateKey::generate_ed25519();
    let metadata_key = PrivateKey::generate_ed25519();

    // Create the NFT with all lower-privilege keys.
    let token_id = TokenCreateTransaction::new()
        .name("Test NFT")
        .symbol("TNFT")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .freeze_key(freeze_key.public_key())
        .supply_key(supply_key.public_key())
        .wipe_key(wipe_key.public_key())
        .kyc_key(kyc_key.public_key())
        .pause_key(pause_key.public_key())
        .fee_schedule_key(fee_schedule_key.public_key())
        .metadata_key(metadata_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.freeze_key, Some(Key::Single(freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    // Generate an unusable key.
    let unusable_key = PublicKey::from_str_ed25519(
        "0x0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();

    // Update all of token’s lower-privilege keys to an unusable key (i.e., all-zeros key),
    // when signing with a respective lower-privilege key,
    // and setting the key verification mode to NO_VALIDATION
    let _ = TokenUpdateTransaction::new()
        .token_id(token_id)
        .freeze_key(unusable_key)
        .wipe_key(unusable_key)
        .kyc_key(unusable_key)
        .supply_key(unusable_key)
        .pause_key(unusable_key)
        .fee_schedule_key(unusable_key)
        .metadata_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .freeze_with(&client)?
        .sign(freeze_key.clone())
        .sign(wipe_key.clone())
        .sign(kyc_key.clone())
        .sign(supply_key.clone())
        .sign(pause_key.clone())
        .sign(fee_schedule_key.clone())
        .sign(metadata_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.freeze_key, Some(Key::Single(unusable_key)));
    assert_eq!(token_info.wipe_key, Some(Key::Single(unusable_key)));
    assert_eq!(token_info.kyc_key, Some(Key::Single(unusable_key)));
    assert_eq!(token_info.supply_key, Some(Key::Single(unusable_key)));
    assert_eq!(token_info.pause_key, Some(Key::Single(unusable_key)));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(unusable_key)));
    assert_eq!(token_info.metadata_key, Some(Key::Single(unusable_key)));

    Ok(())
}

// HIP-540 (https://hips.hedera.com/hip/hip-540)
// Can update all of token’s lower-privilege keys when signing with an old lower-privilege key
// and with a new lower-privilege key, and setting key verification mode to FULL_VALIDATION
#[tokio::test]
async fn update_keys_with_new_and_old_lower_privilege_keys_sigs() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    // Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let freeze_key = PrivateKey::generate_ed25519();
    let wipe_key = PrivateKey::generate_ed25519();
    let kyc_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();
    let pause_key = PrivateKey::generate_ed25519();
    let fee_schedule_key = PrivateKey::generate_ed25519();
    let metadata_key = PrivateKey::generate_ed25519();

    // New Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let new_freeze_key = PrivateKey::generate_ed25519();
    let new_wipe_key = PrivateKey::generate_ed25519();
    let new_kyc_key = PrivateKey::generate_ed25519();
    let new_supply_key = PrivateKey::generate_ed25519();
    let new_pause_key = PrivateKey::generate_ed25519();
    let new_fee_schedule_key = PrivateKey::generate_ed25519();
    let new_metadata_key = PrivateKey::generate_ed25519();

    // Create the NFT with all of token’s lower-privilege keys set to the respective new keys.
    let token_id = TokenCreateTransaction::new()
        .name("Test NFT")
        .symbol("TNFT")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .freeze_key(freeze_key.public_key())
        .supply_key(supply_key.public_key())
        .wipe_key(wipe_key.public_key())
        .kyc_key(kyc_key.public_key())
        .pause_key(pause_key.public_key())
        .fee_schedule_key(fee_schedule_key.public_key())
        .metadata_key(metadata_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.freeze_key, Some(Key::Single(freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    // Update all of token’s lower-privilege keys when signing with an old respective lower-privilege key,
    // and setting key verification mode to NO_VALIDATION
    let _ = TokenUpdateTransaction::new()
        .token_id(token_id)
        .freeze_key(new_freeze_key.public_key())
        .wipe_key(new_wipe_key.public_key())
        .kyc_key(new_kyc_key.public_key())
        .supply_key(new_supply_key.public_key())
        .pause_key(new_pause_key.public_key())
        .fee_schedule_key(new_fee_schedule_key.public_key())
        .metadata_key(new_metadata_key.public_key())
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(freeze_key.clone())
        .sign(kyc_key.clone())
        .sign(wipe_key.clone())
        .sign(supply_key.clone())
        .sign(pause_key.clone())
        .sign(fee_schedule_key.clone())
        .sign(metadata_key.clone())
        .sign(new_freeze_key.clone())
        .sign(new_kyc_key.clone())
        .sign(new_wipe_key.clone())
        .sign(new_supply_key.clone())
        .sign(new_pause_key.clone())
        .sign(new_fee_schedule_key.clone())
        .sign(new_metadata_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.freeze_key, Some(Key::Single(new_freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(new_wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(new_kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(new_supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(new_pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(new_fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(new_metadata_key.public_key())));

    Ok(())
}

// HIP-540 (https://hips.hedera.com/hip/hip-540)
// Can update all of token’s lower-privilege keys when signing ONLY with an old lower-privilege key,
// and setting key verification mode to NO_VALIDATION
#[tokio::test]
async fn update_keys_with_all_old_lower_privilege_keys_sigs() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    // Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let freeze_key = PrivateKey::generate_ed25519();
    let wipe_key = PrivateKey::generate_ed25519();
    let kyc_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();
    let pause_key = PrivateKey::generate_ed25519();
    let fee_schedule_key = PrivateKey::generate_ed25519();
    let metadata_key = PrivateKey::generate_ed25519();

    // New Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let new_freeze_key = PrivateKey::generate_ed25519();
    let new_wipe_key = PrivateKey::generate_ed25519();
    let new_kyc_key = PrivateKey::generate_ed25519();
    let new_supply_key = PrivateKey::generate_ed25519();
    let new_pause_key = PrivateKey::generate_ed25519();
    let new_fee_schedule_key = PrivateKey::generate_ed25519();
    let new_metadata_key = PrivateKey::generate_ed25519();

    // Create the NFT with all of token’s lower-privilege keys.
    let token_id = TokenCreateTransaction::new()
        .name("Test NFT")
        .symbol("TNFT")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .freeze_key(freeze_key.public_key())
        .supply_key(supply_key.public_key())
        .wipe_key(wipe_key.public_key())
        .kyc_key(kyc_key.public_key())
        .pause_key(pause_key.public_key())
        .fee_schedule_key(fee_schedule_key.public_key())
        .metadata_key(metadata_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.freeze_key, Some(Key::Single(freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    // Update all of token’s lower-privilege keys when signing with all older respective lower-privilege keys,
    // and setting key verification mode to NO_VALIDATION
    let _ = TokenUpdateTransaction::new()
        .token_id(token_id)
        .freeze_key(new_freeze_key.public_key())
        .wipe_key(new_wipe_key.public_key())
        .kyc_key(new_kyc_key.public_key())
        .supply_key(new_supply_key.public_key())
        .pause_key(new_pause_key.public_key())
        .fee_schedule_key(new_fee_schedule_key.public_key())
        .metadata_key(new_metadata_key.public_key())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .freeze_with(&client)?
        .sign(freeze_key.clone())
        .sign(kyc_key.clone())
        .sign(wipe_key.clone())
        .sign(supply_key.clone())
        .sign(pause_key.clone())
        .sign(fee_schedule_key.clone())
        .sign(metadata_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.freeze_key, Some(Key::Single(new_freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(new_wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(new_kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(new_supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(new_pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(new_fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(new_metadata_key.public_key())));

    Ok(())
}

// HIP-540 (https://hips.hedera.com/hip/hip-540)
// Cannot remove all of token’s lower-privilege keys when updating them to an empty KeyList,
// signing with a respective lower-privilege key, and setting the key verification mode to NO_VALIDATION
#[tokio::test]
async fn remove_empty_keylist_keys_lower_privilege_keys_sigs_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    // Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let freeze_key = PrivateKey::generate_ed25519();
    let wipe_key = PrivateKey::generate_ed25519();
    let kyc_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();
    let pause_key = PrivateKey::generate_ed25519();
    let fee_schedule_key = PrivateKey::generate_ed25519();
    let metadata_key = PrivateKey::generate_ed25519();

    // Create the NFT with all of token’s lower-privilege keys.
    let token_id = TokenCreateTransaction::new()
        .name("Test NFT")
        .symbol("TNFT")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .freeze_key(freeze_key.public_key())
        .supply_key(supply_key.public_key())
        .wipe_key(wipe_key.public_key())
        .kyc_key(kyc_key.public_key())
        .pause_key(pause_key.public_key())
        .fee_schedule_key(fee_schedule_key.public_key())
        .metadata_key(metadata_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.freeze_key, Some(Key::Single(freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    let empty_keylist = KeyList::new();

    // Remove all of token’s lower-privilege keys
    // when updating them to an empty KeyList (trying to remove keys one by one to check all errors),
    // signing with a respective lower-privilege key,
    // and setting the key verification mode to NO_VALIDATION
    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .wipe_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .freeze_with(&client)?
        .sign(wipe_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::TokenIsImmutable, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .kyc_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .freeze_with(&client)?
        .sign(kyc_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::TokenIsImmutable, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .freeze_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .freeze_with(&client)?
        .sign(freeze_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::TokenIsImmutable, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .pause_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .freeze_with(&client)?
        .sign(pause_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::TokenIsImmutable, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .supply_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .freeze_with(&client)?
        .sign(supply_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::TokenIsImmutable, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .fee_schedule_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .freeze_with(&client)?
        .sign(fee_schedule_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::TokenIsImmutable, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .metadata_key(empty_keylist.clone())
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .freeze_with(&client)?
        .sign(metadata_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::TokenIsImmutable, transaction_id: _ })
    );

    Ok(())
}

// HIP-540 (https://hips.hedera.com/hip/hip-540)
// Cannot update all of token’s lower-privilege keys to an unusable key (i.e. all-zeros key),
// when signing with a key that is different from a respective lower-privilege key, and setting
// the key verification mode to NO_VALIDATION
#[tokio::test]
async fn update_keys_unusable_key_unknown_key_sig_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    // Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let freeze_key = PrivateKey::generate_ed25519();
    let wipe_key = PrivateKey::generate_ed25519();
    let kyc_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();
    let pause_key = PrivateKey::generate_ed25519();
    let fee_schedule_key = PrivateKey::generate_ed25519();
    let metadata_key = PrivateKey::generate_ed25519();

    // Create the NFT with all of token’s lower-privilege keys.
    let token_id = TokenCreateTransaction::new()
        .name("Test NFT")
        .symbol("TNFT")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .freeze_key(freeze_key.public_key())
        .supply_key(supply_key.public_key())
        .wipe_key(wipe_key.public_key())
        .kyc_key(kyc_key.public_key())
        .pause_key(pause_key.public_key())
        .fee_schedule_key(fee_schedule_key.public_key())
        .metadata_key(metadata_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.freeze_key, Some(Key::Single(freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    // Generate an unusable key.
    let unusable_key = PublicKey::from_str_ed25519(
        "0x0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();

    // Update all of token’s lower-privilege keys to an unusable key (i.e. all-zeros key)
    // (trying to remove keys one by one to check all errors),
    // signing with a key that is different from a respective lower-privilege key (implicitly with an operator key),
    // and setting the key verification mode to NO_VALIDATION
    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .wipe_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .kyc_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .freeze_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .pause_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .supply_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .fee_schedule_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .metadata_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::NoValidation)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    Ok(())
}

// HIP-540 (https://hips.hedera.com/hip/hip-540)
// Cannot update all of token’s lower-privilege keys to an unusable key (i.e. all-zeros key),
// when signing ONLY with an old respective lower-privilege key, and setting the key
// verification mode to FULL_VALIDATION
#[tokio::test]
async fn update_with_unusable_key_with_old_key_sig_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    // Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let freeze_key = PrivateKey::generate_ed25519();
    let wipe_key = PrivateKey::generate_ed25519();
    let kyc_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();
    let pause_key = PrivateKey::generate_ed25519();
    let fee_schedule_key = PrivateKey::generate_ed25519();
    let metadata_key = PrivateKey::generate_ed25519();

    // Create the NFT with all of token’s lower-privilege keys.
    let token_id = TokenCreateTransaction::new()
        .name("Test NFT")
        .symbol("TNFT")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .freeze_key(freeze_key.public_key())
        .supply_key(supply_key.public_key())
        .wipe_key(wipe_key.public_key())
        .kyc_key(kyc_key.public_key())
        .pause_key(pause_key.public_key())
        .fee_schedule_key(fee_schedule_key.public_key())
        .metadata_key(metadata_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.freeze_key, Some(Key::Single(freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    // Generate an unusable key.
    let unusable_key = PublicKey::from_str_ed25519(
        "0x0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();

    // Update all of token’s lower-privilege keys to an unusable key (i.e., all-zeros key)
    // (trying to remove keys one by one to check all errors),
    // signing ONLY with an old respective lower-privilege key,
    // and setting the key verification mode to FULL_VALIDATION
    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .wipe_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(wipe_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .kyc_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(kyc_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .freeze_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(freeze_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .pause_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(pause_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .supply_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(supply_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .fee_schedule_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(fee_schedule_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .metadata_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(metadata_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    Ok(())
}

// HIP-540 (https://hips.hedera.com/hip/hip-540)
// Cannot update all of token’s lower-privilege keys to an unusable key (i.e. all-zeros key),
// when signing with an old respective lower-privilege key and new respective lower-privilege key,
// and setting the key verification mode to FULL_VALIDATION
#[tokio::test]
async fn update_unusable_key_old_new_key_sig_full_validation_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    // Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let freeze_key = PrivateKey::generate_ed25519();
    let wipe_key = PrivateKey::generate_ed25519();
    let kyc_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();
    let pause_key = PrivateKey::generate_ed25519();
    let fee_schedule_key = PrivateKey::generate_ed25519();
    let metadata_key = PrivateKey::generate_ed25519();

    // New Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let new_freeze_key = PrivateKey::generate_ed25519();
    let new_wipe_key = PrivateKey::generate_ed25519();
    let new_kyc_key = PrivateKey::generate_ed25519();
    let new_supply_key = PrivateKey::generate_ed25519();
    let new_pause_key = PrivateKey::generate_ed25519();
    let new_fee_schedule_key = PrivateKey::generate_ed25519();
    let new_metadata_key = PrivateKey::generate_ed25519();

    // Create the NFT with all of token’s lower-privilege keys.
    let token_id = TokenCreateTransaction::new()
        .name("Test NFT")
        .symbol("TNFT")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .freeze_key(freeze_key.public_key())
        .supply_key(supply_key.public_key())
        .wipe_key(wipe_key.public_key())
        .kyc_key(kyc_key.public_key())
        .pause_key(pause_key.public_key())
        .fee_schedule_key(fee_schedule_key.public_key())
        .metadata_key(metadata_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.freeze_key, Some(Key::Single(freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    // Generate an unusable key.
    let unusable_key = PublicKey::from_str_ed25519(
        "0x0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();

    // Update all of token’s lower-privilege keys to an unusable key (i.e., all-zeros key)
    // (trying to remove keys one by one to check all errors),
    // signing with an old respective lower-privilege key and new respective lower-privilege key,
    // and setting the key verification mode to FULL_VALIDATION
    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .wipe_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(wipe_key)
        .sign(new_wipe_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .kyc_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(kyc_key)
        .sign(new_kyc_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .freeze_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(freeze_key)
        .sign(new_freeze_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .pause_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(pause_key)
        .sign(new_pause_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .supply_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(supply_key)
        .sign(new_supply_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .fee_schedule_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(fee_schedule_key)
        .sign(new_fee_schedule_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .metadata_key(unusable_key)
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(metadata_key)
        .sign(new_metadata_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    Ok(())
}

// HIP-540 (https://hips.hedera.com/hip/hip-540)
// Cannot update all of token’s lower-privilege keys, when signing ONLY with an
// old respective lower-privilege key, and setting the key verification mode to
// FULL_VALIDATION
#[tokio::test]
async fn update_keys_old_key_sig_full_validation_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    // Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let freeze_key = PrivateKey::generate_ed25519();
    let wipe_key = PrivateKey::generate_ed25519();
    let kyc_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();
    let pause_key = PrivateKey::generate_ed25519();
    let fee_schedule_key = PrivateKey::generate_ed25519();
    let metadata_key = PrivateKey::generate_ed25519();

    // New Freeze, Wipe, Kyc, Supply, Pause, Fee Schedule, and Metadata keys.
    let new_freeze_key = PrivateKey::generate_ed25519();
    let new_wipe_key = PrivateKey::generate_ed25519();
    let new_kyc_key = PrivateKey::generate_ed25519();
    let new_supply_key = PrivateKey::generate_ed25519();
    let new_pause_key = PrivateKey::generate_ed25519();
    let new_fee_schedule_key = PrivateKey::generate_ed25519();
    let new_metadata_key = PrivateKey::generate_ed25519();

    // Create the NFT with all of token’s lower-privilege keys.
    let token_id = TokenCreateTransaction::new()
        .name("Test NFT")
        .symbol("TNFT")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .freeze_key(freeze_key.public_key())
        .supply_key(supply_key.public_key())
        .wipe_key(wipe_key.public_key())
        .kyc_key(kyc_key.public_key())
        .pause_key(pause_key.public_key())
        .fee_schedule_key(fee_schedule_key.public_key())
        .metadata_key(metadata_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let token_info = TokenInfoQuery::new().token_id(token_id).execute(&client).await?;

    assert_eq!(token_info.freeze_key, Some(Key::Single(freeze_key.public_key())));
    assert_eq!(token_info.wipe_key, Some(Key::Single(wipe_key.public_key())));
    assert_eq!(token_info.kyc_key, Some(Key::Single(kyc_key.public_key())));
    assert_eq!(token_info.supply_key, Some(Key::Single(supply_key.public_key())));
    assert_eq!(token_info.pause_key, Some(Key::Single(pause_key.public_key())));
    assert_eq!(token_info.fee_schedule_key, Some(Key::Single(fee_schedule_key.public_key())));
    assert_eq!(token_info.metadata_key, Some(Key::Single(metadata_key.public_key())));

    // Update all of token’s lower-privilege keys
    // (trying to update keys one by one to check all errors),
    // signing ONLY with an old respective lower-privilege key,
    // and setting the key verification mode to FULL_VALIDATION
    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .wipe_key(new_wipe_key.public_key())
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(wipe_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .kyc_key(new_kyc_key.public_key())
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(kyc_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .freeze_key(new_freeze_key.public_key())
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(freeze_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .pause_key(new_pause_key.public_key())
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(pause_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .supply_key(new_supply_key.public_key())
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(supply_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .fee_schedule_key(new_fee_schedule_key.public_key())
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(fee_schedule_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    let tx = TokenUpdateTransaction::new()
        .token_id(token_id)
        .metadata_key(new_metadata_key.public_key())
        .key_verification_mode(TokenKeyValidation::FullValidation)
        .freeze_with(&client)?
        .sign(metadata_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        tx,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    Ok(())
}

fn generate_lower_privileges_keys(
) -> (PrivateKey, PrivateKey, PrivateKey, PrivateKey, PrivateKey, PrivateKey, PrivateKey) {
    let wipe_key = PrivateKey::generate_ed25519();
    let kyc_key = PrivateKey::generate_ed25519();
    let freeze_key = PrivateKey::generate_ed25519();
    let pause_key = PrivateKey::generate_ed25519();
    let supply_key = PrivateKey::generate_ed25519();
    let fee_schedule_key = PrivateKey::generate_ed25519();
    let metadata_key = PrivateKey::generate_ed25519();

    (wipe_key, kyc_key, freeze_key, pause_key, supply_key, fee_schedule_key, metadata_key)
}
