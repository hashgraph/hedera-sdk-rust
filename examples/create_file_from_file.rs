use failure::{format_err, Error};
use hedera::{Client, SecretKey, Status};
use std::{env, thread::sleep, time::Duration};
use std::str::FromStr;
use std::io::prelude::*;
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::try_init()?;

    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee
    let operator = "0:0:2".parse()?;
    let client = Client::builder("testnet.hedera.com:50003")
        .node("0:0:3".parse()?)
        .operator(operator, || env::var("OPERATOR_SECRET"))
        .build()?;

    let operator_secret : String = env::var("OPERATOR_SECRET")?;
    let secret = SecretKey::from_str(&operator_secret)?;
    let public = secret.public();

    // load file from file system
    let mut my_file = File::open("examples/Hedera.txt")?;
    let mut file_contents = Vec::new();
    // read the whole file
    my_file.read_to_end(&mut file_contents)?;

    // Create a file
    let id = client
        .create_file()
        .expires_in(Duration::from_secs(2_592_000))
        .key(public)
        .contents(file_contents)
        .memo("[hedera-sdk-rust][example] create_file")
        .sign(&env::var("OPERATOR_SECRET")?.parse()?) // sign as the owner of the file
        .execute_async()
        .await?;

    println!("creating file; transaction = {}", id);

    // If we got here we know we passed pre-check
    // Depending on your requirements that may be enough for some kinds of transactions
    sleep(Duration::from_secs(2));

    // Get the receipt and check the status to prove it was successful
    let mut tx = client.transaction(id).receipt();
    let receipt = tx.get_async().await?;

    if receipt.status != Status::Success {
        Err(format_err!(
            "transaction has a non-successful status: {:?}",
            receipt.status
        ))?;
    }

    let file = receipt.file_id.unwrap();
    println!("file ID = {}", file);

    Ok(())
}
