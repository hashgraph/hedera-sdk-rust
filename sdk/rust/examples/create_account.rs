use std::str::FromStr;
use std::time::Duration;

use assert_matches::assert_matches;
use hedera::{AccountCreateTransaction, AccountId, Client, PrivateKey, TransactionReceiptQuery};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::for_testnet();

    client.set_payer_account_id(AccountId::from(6189));
    client.add_default_signer(PrivateKey::from_str(
        "7f7ac6c8025a15ff1e07ef57c7295601379a4e9a526560790ae85252393868f0",
    )?);

    let new_key = PrivateKey::generate_ed25519();

    println!("private key = {new_key}");
    println!("public key = {}", new_key.public_key());

    let response = AccountCreateTransaction::new()
        .key(new_key.public_key())
        .execute(&client)
        .await?;

    // TODO: <TransactionReceiptQuery> should auto-retry
    sleep(Duration::from_secs(5)).await;

    let receipt = TransactionReceiptQuery::new()
        .transaction_id(response.transaction_id)
        .execute(&client)
        .await?;

    let new_account_id = assert_matches!(receipt.account_id, Some(id) => id);

    println!("account address = {new_account_id}");

    Ok(())
}
