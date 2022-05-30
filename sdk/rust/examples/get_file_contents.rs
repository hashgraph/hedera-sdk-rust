use clap::Parser;
use hedera::{AccountId, Client, FileContentsQuery, FileId, PrivateKey};

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, env)]
    payer_account_id: AccountId,

    #[clap(long, env)]
    default_signer: PrivateKey,

    #[clap(long, env, default_value = "0.0.34945328")]
    file: FileId,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();
    let args = Args::parse();

    let client = Client::for_testnet();

    client.set_payer_account_id(args.payer_account_id);
    client.add_default_signer(args.default_signer);

    let cr = FileContentsQuery::new()
        .file_id(args.file)
        .execute(&client)
        .await?;

    let contents = String::from_utf8(cr.contents)?;

    println!("contents: {contents}");

    Ok(())
}
