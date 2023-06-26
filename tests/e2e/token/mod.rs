mod associate;
mod burn;
mod delete;
mod dissociate;
mod fee_schedule_update;
mod freeze;
mod grant_kyc;
mod info;
mod mint;
mod revoke_kyc;
mod unfreeze;

use hedera::{
    Client,
    TokenBurnTransaction,
    TokenCreateTransaction,
    TokenDeleteTransaction,
    TokenId,
    TokenMintTransaction,
    TransactionReceipt,
    TransactionResponse,
};
use time::{
    Duration,
    OffsetDateTime,
};
use tokio::task::JoinSet;

use crate::account::Account;
use crate::common::{
    setup_global,
    Operator,
    TestEnvironment,
};

struct FungibleToken {
    id: TokenId,
    owner: Account,
}

impl FungibleToken {
    async fn create(client: &Client, owner: &Account, initial_supply: u64) -> hedera::Result<Self> {
        let owner_public_key = owner.key.public_key();
        let token_id = TokenCreateTransaction::new()
            .name("ffff")
            .symbol("F")
            .decimals(3)
            .initial_supply(initial_supply)
            .treasury_account_id(owner.id)
            .admin_key(owner_public_key)
            .freeze_key(owner_public_key)
            .wipe_key(owner_public_key)
            .kyc_key(owner_public_key)
            .supply_key(owner_public_key)
            .fee_schedule_key(owner_public_key)
            .freeze_default(false)
            .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
            .sign(owner.key.clone())
            .execute(client)
            .await?
            .get_receipt(client)
            .await?
            .token_id
            .unwrap();

        Ok(Self { id: token_id, owner: owner.clone() })
    }

    async fn burn(&self, client: &Client, supply: u64) -> hedera::Result<()> {
        hedera::TokenBurnTransaction::new()
            .token_id(self.id)
            .amount(supply)
            .sign(self.owner.key.clone())
            .execute(client)
            .await?
            .get_receipt(&client)
            .await?;

        Ok(())
    }

    async fn delete(self, client: &Client) -> hedera::Result<()> {
        TokenDeleteTransaction::new()
            .token_id(self.id)
            .sign(self.owner.key)
            .execute(&client)
            .await?
            .get_receipt(&client)
            .await?;

        Ok(())
    }
}

struct Nft {
    id: TokenId,
    owner: Account,
}

impl Nft {
    async fn create(client: &Client, owner: &Account) -> hedera::Result<Self> {
        let owner_public_key = owner.key.public_key();
        let token_id = TokenCreateTransaction::new()
            .name("ffff")
            .symbol("F")
            .token_type(hedera::TokenType::NonFungibleUnique)
            .treasury_account_id(owner.id)
            .admin_key(owner_public_key)
            .freeze_key(owner_public_key)
            .wipe_key(owner_public_key)
            .supply_key(owner_public_key)
            .fee_schedule_key(owner_public_key)
            .freeze_default(false)
            .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
            .sign(owner.key.clone())
            .execute(client)
            .await?
            .get_receipt(client)
            .await?
            .token_id
            .unwrap();

        Ok(Self { id: token_id, owner: owner.clone() })
    }

    // fixme: find a better name
    async fn mint_incremental(
        &self,
        client: &Client,
        nfts_to_mint: u8,
    ) -> hedera::Result<TransactionReceipt> {
        self.mint(&client, (0..nfts_to_mint).map(|it| [it])).await
    }

    async fn mint<Bytes: AsRef<[u8]>>(
        &self,
        client: &Client,
        metadata: impl IntoIterator<Item = Bytes>,
    ) -> hedera::Result<TransactionReceipt> {
        async fn inner(
            nft: &Nft,
            client: &Client,
            mut tx: TokenMintTransaction,
        ) -> hedera::Result<TransactionReceipt> {
            tx.token_id(nft.id)
                .sign(nft.owner.key.clone())
                .execute(client)
                .await?
                .get_receipt(client)
                .await
        }

        let mut tx = TokenMintTransaction::new();

        tx.metadata(metadata);

        inner(self, client, tx).await
    }

    async fn burn(
        &self,
        client: &Client,
        serials: impl IntoIterator<Item = i64>,
    ) -> hedera::Result<()> {
        // non generic inner function to save generic instantiations... Not that that's a huge concern here.
        async fn inner(
            nft: &Nft,
            client: &Client,
            mut tx: TokenBurnTransaction,
        ) -> hedera::Result<()> {
            tx.token_id(nft.id).sign(nft.owner.key.clone()).execute(client).await.map(drop)
        }

        let mut tx = TokenBurnTransaction::new();

        tx.serials(serials);

        inner(self, client, tx).await
    }

    async fn delete(self, client: &Client) -> hedera::Result<()> {
        TokenDeleteTransaction::new()
            .token_id(self.id)
            .sign(self.owner.key)
            .execute(&client)
            .await?
            .get_receipt(&client)
            .await?;

        Ok(())
    }
}

#[tokio::test]
async fn mint_several_nfts_at_once() -> anyhow::Result<()> {
    async fn setup(op: &Operator, client: &Client) -> anyhow::Result<TokenId> {
        let token_id = TokenCreateTransaction::new()
            .name("sdk::rust::e2e::mint_many")
            .symbol("µ")
            .token_type(hedera::TokenType::NonFungibleUnique)
            .treasury_account_id(op.account_id)
            .admin_key(op.private_key.clone().public_key())
            .supply_key(op.private_key.clone().public_key())
            .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
            .freeze_default(false)
            .execute(&client)
            .await?
            .get_receipt(&client)
            .await?
            .token_id
            .ok_or_else(|| anyhow::anyhow!("Token creation failed"))?;

        log::info!("successfully created token {token_id}");

        Ok(token_id)
    }

    async fn teardown(client: &Client, token_id: TokenId) -> anyhow::Result<()> {
        TokenDeleteTransaction::new()
            .token_id(token_id)
            .execute(&client)
            .await?
            .get_receipt(&client)
            .await?;

        Ok(())
    }

    const MINT_TRANSACTIONS: usize = 5;
    // mint faster by using less transactions.
    const MAX_MINTS_PER_TX: usize = 10;

    let TestEnvironment { config, client } = setup_global();

    let Some(op) = &config.operator else {
        log::debug!("skipping test due to lack of operator");
        return Ok(())
    };

    if !config.run_nonfree_tests {
        log::debug!("skipping non-free test");
        return Ok(());
    }

    let token_id = setup(&op, &client).await?;

    let mut tasks = JoinSet::new();

    for _ in 0..MINT_TRANSACTIONS {
        // give the tasks a bit of time between spawning to avoid hammering the network.
        tokio::time::sleep(std::time::Duration::from_millis(25)).await;
        tasks.spawn({
            let client = client.clone();
            async move { create_nft(&client, token_id, MAX_MINTS_PER_TX).await }
        });
    }

    let mut responses = Vec::with_capacity(MINT_TRANSACTIONS);

    // note: we collect the responses to test simultaniously waiting for multiple receipts next.
    while let Some(response) = tasks.join_next().await {
        let response = response??;

        responses.push(response);
    }

    let mut tasks = JoinSet::new();

    for response in responses {
        // give the tasks a bit of time between spawning to avoid hammering the network.
        tokio::time::sleep(std::time::Duration::from_millis(25)).await;

        let client = client.clone();
        tasks.spawn(async move { response.get_receipt(&client).await });
    }

    while let Some(receipt) = tasks.join_next().await {
        // we error for status here.
        let _receipt = receipt??;
    }

    teardown(&client, token_id).await?;

    Ok(())
}

async fn create_nft(
    client: &Client,
    token_id: TokenId,
    nfts: usize,
) -> hedera::Result<TransactionResponse> {
    TokenMintTransaction::default()
        .token_id(token_id)
        .metadata(vec![Vec::from([0x12, 0x34]); nfts])
        .execute(client)
        .await
}
