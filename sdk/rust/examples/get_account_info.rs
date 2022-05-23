use std::str::FromStr;

use hedera::{AccountId, AccountInfoQuery, Client, PrivateKey};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::for_testnet();

    client.set_payer_account_id(AccountId::from(6189));
    client.add_default_signer(PrivateKey::from_str(
        "7f7ac6c8025a15ff1e07ef57c7295601379a4e9a526560790ae85252393868f0",
    )?);

    let id = AccountId::from(1001);

    let info = AccountInfoQuery::new()
        .account_id(id)
        .execute(&client)
        .await?;

    println!("info = {:#?}", info);

    Ok(())
}
