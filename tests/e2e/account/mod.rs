mod account_id_population;
mod allowance_approve;
mod allowance_delete;
mod balance;
mod create;
mod delete;
mod info;
mod update;

use hedera::{
    AccountId,
    Hbar,
    PrivateKey,
};

#[derive(Clone)]
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

    pub async fn create_ecdsa(
        initial_balance: Hbar,
        client: &hedera::Client,
    ) -> hedera::Result<Self> {
        let key = PrivateKey::generate_ecdsa();

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
