use hedera::{AccountId, AccountInfoQuery, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::for_testnet();

    let id = AccountId::from(1001);
    let info = AccountInfoQuery::new().account_id(id.into()).execute(&client).await?;

    println!("info = {:#?}", info);

    Ok(())
}
