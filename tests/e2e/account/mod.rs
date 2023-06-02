mod allowance_approve;
mod create;
mod delete;
mod update;

use hedera::{
    AccountBalanceQuery,
    AccountId,
    Hbar,
    PrivateKey,
};

use crate::common::{
    setup_global,
    TestEnvironment,
};

#[tokio::test]
async fn account_balance_query() -> anyhow::Result<()> {
    let TestEnvironment { config, client } = setup_global();

    let Some(op) = &config.operator else {
        log::debug!("skipping test due to lack of operator");

        return Ok(())
    };

    let balance = AccountBalanceQuery::new().account_id(op.account_id).execute(&client).await?;

    log::trace!("successfully queried balance: {balance:?}");

    anyhow::ensure!(balance.account_id == op.account_id);

    Ok(())
}

pub struct Account {
    pub key: PrivateKey,
    pub id: AccountId,
}

impl Account {
    pub async fn create(initial_balance: Hbar, client: &hedera::Client) -> hedera::Result<Self> {
        let key = PrivateKey::generate_ed25519();

        let receipt = hedera::AccountCreateTransaction::new()
            .key(key.public_key())
            .initial_balance(initial_balance)
            .execute(client)
            .await?
            .get_receipt(client)
            .await?;

        let account_id = receipt.account_id.unwrap();

        Ok(Self { key, id: account_id })
    }

    pub async fn delete(self, client: &hedera::Client) -> hedera::Result<()> {
        hedera::AccountDeleteTransaction::new()
            .account_id(self.id)
            .transfer_account_id(client.get_operator_account_id().unwrap())
            .freeze_with(client)?
            .sign(self.key)
            .execute(client)
            .await?
            .get_receipt(client)
            .await?;

        Ok(())
    }
}
