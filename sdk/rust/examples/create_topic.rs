use assert_matches::assert_matches;
use clap::Parser;
use hedera::{AccountId, Client, PrivateKey, TopicCreateTransaction};

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

    let receipt = TopicCreateTransaction::new()
        .execute(&client)
        .await?
        .get_successful_receipt(&client)
        .await?;

    let new_topic_id = assert_matches!(receipt.topic_id, Some(id) => id);

    println!("topic address = {new_topic_id}");

    Ok(())
}
