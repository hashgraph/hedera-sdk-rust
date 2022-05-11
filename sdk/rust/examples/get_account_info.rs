use hedera::{AccountId, AccountInfoQuery, Client, PrivateKey};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::try_init_timed()?;

    let client = Client::for_testnet();

    let payer_key: PrivateKey =
        "7f7ac6c8025a15ff1e07ef57c7295601379a4e9a526560790ae85252393868f0".parse()?;

    client.set_payer_account_id(AccountId::from(6189));

    let id = AccountId::from(1001);

    let info = AccountInfoQuery::new()
        .account_id(id.into())
        .payment_amount(100)
        .payment_signer(&payer_key)
        .execute(&client)
        .await?;

    println!("info = {:#?}", info);

    Ok(())
}
