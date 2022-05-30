use clap::Parser;
use hedera::{AccountId, AccountInfoQuery, Client, PrivateKey};

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, env)]
    payer_account_id: AccountId,

    #[clap(long, env)]
    default_signer: PrivateKey,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();
    let args = Args::parse();

    let client = Client::for_testnet();

    client.set_payer_account_id(args.payer_account_id);
    client.add_default_signer(args.default_signer);

    let id = AccountId::from(34938045);

    let info = AccountInfoQuery::new()
        .account_id(id)
        .execute(&client)
        .await?;

    println!("info = {:#?}", info);

    Ok(())
}
