use hedera::{
    AccountCreateTransaction,
    PrivateKey,
    ScheduleCreateTransaction,
    ScheduleInfoQuery,
};

use crate::common::{
    setup_nonfree,
    TestEnvironment,
};

#[tokio::test]
#[ignore = "not implemented in Hedera yet"]
async fn basic() -> anyhow::Result<()> {
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
    info.scheduled_transaction().unwrap();

    Ok(())
}
