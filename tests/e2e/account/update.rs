use hedera::{
    AccountCreateTransaction,
    AccountInfoQuery,
    AccountUpdateTransaction,
    Hbar,
    Key,
    PrivateKey,
};
use time::Duration;

use crate::common::{
    setup_nonfree,
    TestEnvironment,
};

#[tokio::test]
async fn set_key() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let key1 = PrivateKey::generate_ed25519();
    let key2 = PrivateKey::generate_ed25519();

    let account_id = AccountCreateTransaction::new()
        .key(key1.public_key())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .account_id
        .unwrap();

    let info = AccountInfoQuery::new().account_id(account_id).execute(&client).await?;

    // none of these are relevant to this test (they should already be tested by the create E2E tests)
    // assertThat(info.accountId).isEqualTo(accountId);
    // assertThat(info.isDeleted).isFalse();
    // assertThat(info.balance).isEqualTo(new Hbar(0));
    // assertThat(info.autoRenewPeriod).isEqualTo(Duration.ofDays(90));
    // assertThat(info.proxyAccountId).isNull();
    // assertThat(info.proxyReceived).isEqualTo(Hbar.ZERO);

    assert_eq!(info.key, Key::Single(key1.public_key()));

    AccountUpdateTransaction::new()
        .account_id(account_id)
        .key(key2.public_key())
        .freeze_with(&client)?
        .sign(key1)
        .sign(key2.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let info = AccountInfoQuery::new().account_id(account_id).execute(&client).await?;

    assert_eq!(info.account_id, account_id);
    assert_eq!(info.is_deleted, false);
    assert_eq!(info.key, Key::Single(key2.public_key()));
    assert_eq!(info.balance, Hbar::ZERO);
    assert_eq!(info.auto_renew_period, Some(Duration::days(90)));

    #[allow(deprecated)]
    {
        assert_eq!(info.proxy_account_id, None);
    }
    assert_eq!(info.proxy_received, Hbar::ZERO);

    Ok(())
}

#[tokio::test]
async fn missing_account_id_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let res = AccountUpdateTransaction::new().execute(&client).await?.get_receipt(&client).await;

    assert_matches::assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus {
            status: hedera::Status::AccountIdDoesNotExist,
            transaction_id: _
        })
    );

    Ok(())
}
