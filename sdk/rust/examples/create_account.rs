use assert_matches::assert_matches;
use clap::Parser;
use hedera::{AccountCreateTransaction, AccountId, Client, PrivateKey, TransactionReceiptQuery};

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

    let new_key = PrivateKey::generate_ed25519();

    println!("private key = {new_key}");
    println!("public key = {}", new_key.public_key());

    let response = AccountCreateTransaction::new()
        .key(new_key.public_key())
        .execute(&client)
        .await?;

    let receipt = TransactionReceiptQuery::new()
        .transaction_id(response.transaction_id)
        .execute(&client)
        .await?;

    let new_account_id = assert_matches!(receipt.account_id, Some(id) => id);

    println!("account address = {new_account_id}");

    Ok(())
}
