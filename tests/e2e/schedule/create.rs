use assert_matches::assert_matches;
use hedera::{
    AccountCreateTransaction,
    AccountDeleteTransaction,
    Hbar,
    KeyList,
    PrivateKey,
    ScheduleCreateTransaction,
    ScheduleInfoQuery,
    ScheduleSignTransaction,
    Status,
    TopicCreateTransaction,
    TopicMessageSubmitTransaction,
    TransferTransaction,
};
use time::OffsetDateTime;

use crate::account::Account;
use crate::common::{
    setup_nonfree,
    TestEnvironment,
};

#[tokio::test]
#[ignore = "not implemented in Hedera yet"]
async fn create_account() -> anyhow::Result<()> {
    let Some(TestEnvironment { config, client }) = setup_nonfree() else { return Ok(()) };

    let Some(op) = &config.operator else {
        log::debug!("skipping test due to missing operator");
        return Ok(());
    };

    let key = PrivateKey::generate_ed25519();

    let mut transaction = AccountCreateTransaction::new();
    transaction.key(key.public_key());

    let schedule_id = ScheduleCreateTransaction::new()
        .scheduled_transaction(transaction)
        .admin_key(op.private_key.public_key())
        .payer_account_id(op.account_id)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .schedule_id
        .unwrap();

    let info = ScheduleInfoQuery::new().schedule_id(schedule_id).execute(&client).await?;

    assert!(info.executed_at.is_some());

    Ok(())
}

#[tokio::test]
#[ignore = "not implemented in Hedera yet"]
async fn create_account_schedule() -> anyhow::Result<()> {
    let Some(TestEnvironment { config, client }) = setup_nonfree() else { return Ok(()) };

    let Some(op) = &config.operator else {
        log::debug!("skipping test due to missing operator");
        return Ok(());
    };

    let key = PrivateKey::generate_ed25519();

    let mut transaction = AccountCreateTransaction::new();
    transaction.key(key.public_key());

    let schedule_id = transaction
        .schedule()
        .admin_key(op.private_key.public_key())
        .payer_account_id(op.account_id)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .schedule_id
        .unwrap();

    let info = ScheduleInfoQuery::new().schedule_id(schedule_id).execute(&client).await?;

    assert!(info.executed_at.is_some());
    info.scheduled_transaction().unwrap();

    Ok(())
}

#[tokio::test]
async fn transfer() -> anyhow::Result<()> {
    let Some(TestEnvironment { config, client }) = setup_nonfree() else { return Ok(()) };

    let Some(op) = &config.operator else {
        log::debug!("skipping test due to missing operator");
        return Ok(());
    };

    let key1 = PrivateKey::generate_ed25519();
    let key2 = PrivateKey::generate_ed25519();
    let key3 = PrivateKey::generate_ed25519();

    let key_list = KeyList::from([key1.public_key(), key2.public_key(), key3.public_key()]);

    // Create the account with the `KeyList`
    let mut transaction = AccountCreateTransaction::new();
    let receipt = transaction
        .key(key_list)
        .initial_balance(Hbar::new(1))
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let account_id = receipt.account_id.unwrap();

    // Create a transfer transaction with 2/3 signatures.
    let mut transfer = TransferTransaction::new();

    transfer.hbar_transfer(account_id, Hbar::new(-1)).hbar_transfer(op.account_id, Hbar::new(1));

    // Schedule the transactoin
    let mut scheduled = transfer.schedule();

    let receipt = scheduled.execute(&client).await?.get_receipt(&client).await?;

    // Get the schedule ID from the receipt
    let schedule_id = receipt.schedule_id.unwrap();

    // Get the schedule info to see if `signatories` is populated with 2/3 signatures
    let info = ScheduleInfoQuery::new().schedule_id(schedule_id).execute(&client).await?;

    assert_eq!(info.executed_at, None);

    // Finally send this last signature to Hedera. This last signature _should_ mean the transaction executes
    // since all 3 signatures have been provided.
    ScheduleSignTransaction::new()
        .schedule_id(schedule_id)
        .sign(key1.clone())
        .sign(key2.clone())
        .sign(key3.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let info = ScheduleInfoQuery::new().schedule_id(schedule_id).execute(&client).await?;

    assert!(info.executed_at.is_some());

    AccountDeleteTransaction::new()
        .account_id(account_id)
        .transfer_account_id(op.account_id)
        .sign(key1)
        .sign(key2)
        .sign(key3)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    Ok(())
}

// token balances are deprecated.
// async fn canScheduleTokenTransfer() -> anyhow::Result<()> {}

#[tokio::test]
async fn double_schedule_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config, client }) = setup_nonfree() else { return Ok(()) };

    let Some(op) = &config.operator else {
        log::debug!("skipping test due to missing operator");
        return Ok(());
    };

    let account = Account::create(Hbar::new(1), &client).await?;

    let mut transfer = TransferTransaction::new();

    transfer.hbar_transfer(op.account_id, Hbar::new(-1)).hbar_transfer(account.id, Hbar::new(1));

    let schedule_id_1 = transfer
        .schedule()
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .schedule_id
        .unwrap();

    let info1 = ScheduleInfoQuery::new().schedule_id(schedule_id_1).execute(&client).await?;

    assert!(info1.executed_at.is_some());

    let mut transfer = TransferTransaction::new();

    transfer.hbar_transfer(op.account_id, Hbar::new(-1)).hbar_transfer(account.id, Hbar::new(1));

    let res = transfer.schedule().execute(&client).await?.get_receipt(&client).await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus {
            status: Status::IdenticalScheduleAlreadyCreated,
            transaction_id: _
        })
    );

    account.delete(&client).await?;
    Ok(())
}

#[tokio::test]
async fn topic_message() -> anyhow::Result<()> {
    let Some(TestEnvironment { config, client }) = setup_nonfree() else { return Ok(()) };

    let Some(op) = &config.operator else {
        log::debug!("skipping test due to missing operator");
        return Ok(());
    };

    // This is the submit key
    let key = PrivateKey::generate_ed25519();

    let topic_id = TopicCreateTransaction::new()
        .admin_key(op.private_key.public_key())
        .auto_renew_account_id(op.account_id)
        .topic_memo("HCS Topic_")
        .submit_key(key.public_key())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .topic_id
        .unwrap();

    let mut transaction = TopicMessageSubmitTransaction::new();

    transaction.topic_id(topic_id).message("scheduled hcs message");

    // create schedule
    let schedule_id = transaction
        .schedule()
        .admin_key(op.private_key.public_key())
        .payer_account_id(op.account_id)
        .schedule_memo(format!(
            "mirror scheduled E2E signature on create and sign_{}",
            OffsetDateTime::now_utc()
        ))
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .schedule_id
        .unwrap();

    let info = ScheduleInfoQuery::new().schedule_id(schedule_id).execute(&client).await?;

    assert_eq!(info.schedule_id, schedule_id);

    ScheduleSignTransaction::new()
        .schedule_id(schedule_id)
        .sign(key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let info = ScheduleInfoQuery::new().schedule_id(schedule_id).execute(&client).await?;

    assert!(info.executed_at.is_some());

    Ok(())
}
