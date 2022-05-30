use assert_matches::assert_matches;
use clap::Parser;
use hedera::{AccountId, Client, FileCreateTransaction, PrivateKey};

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

    let receipt = FileCreateTransaction::new()
        .contents(&b"Hedera Hashgraph is great!"[..])
        .execute(&client)
        .await?
        .get_successful_receipt(&client)
        .await?;

    let new_file_id = assert_matches!(receipt.file_id, Some(id) => id);

    println!("file address = {new_file_id}");

    Ok(())
}
