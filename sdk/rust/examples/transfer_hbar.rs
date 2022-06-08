use clap::Parser;
use hedera::{AccountAddress, AccountId, Client, PrivateKey, TransferTransaction};

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, env)]
    payer_account_id: AccountId,

    #[clap(long, env)]
    default_signer: PrivateKey,

    #[clap(long)]
    sender: Option<AccountId>,

    #[clap(long, default_value = "0.0.1001")]
    receiver: AccountAddress,

    #[clap(long, default_value = "1000")]
    amount: i64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();
    let args = Args::parse();

    let client = Client::for_testnet();

    client.set_payer_account_id(args.payer_account_id);
    client.add_default_signer(args.default_signer);

    let sender = args.sender.unwrap_or(args.payer_account_id);

    TransferTransaction::new()
        .hbar_transfer(sender, -args.amount)
        .hbar_transfer(args.receiver, args.amount)
        .execute(&client)
        .await?
        .wait_for_successful_consensus(&client)
        .await?;

    Ok(())
}
