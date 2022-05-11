use std::str::FromStr;

use hedera::{AccountId, Client, PrivateKey, TransferTransaction};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::try_init_timed()?;

    let client = Client::for_testnet();

    client.set_payer_account_id(AccountId::from(6189));
    client.add_default_signer(PrivateKey::from_str(
        "7f7ac6c8025a15ff1e07ef57c7295601379a4e9a526560790ae85252393868f0",
    )?);

    let sender_id = AccountId::from(1001);
    let receiver_id = AccountId::from(1002);

    let amount = 10_000;

    TransferTransaction::new()
        .hbar_transfer(sender_id, -amount)
        .hbar_transfer(receiver_id, amount)
        .execute(&client)
        .await?;

    Ok(())
}
