use hedera::{AccountBalanceQuery, AccountId, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::try_init_timed()?;

    let client = Client::for_testnet();

    let id = AccountId::from(1001);

    let ab = AccountBalanceQuery::new()
        .account_id(id.into())
        .execute(&client)
        .await?;

    println!("balance = {}", ab.balance);

    Ok(())
}
