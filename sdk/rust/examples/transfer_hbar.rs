use hedera::{AccountId, Client, PrivateKey, TransferTransaction};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::try_init_timed()?;

    let client = Client::for_testnet();

    let payer_key: PrivateKey =
        "7f7ac6c8025a15ff1e07ef57c7295601379a4e9a526560790ae85252393868f0".parse()?;

    client.set_payer_account_id(AccountId::from(6189));
    // client.set_default_signers([&payer_key]);

    let sender_id = AccountId::from(1001);
    let receiver_id = AccountId::from(1002);

    let amount = 10_000;

    TransferTransaction::new()
        .hbar_transfer(sender_id.into(), -amount)
        .hbar_transfer(receiver_id.into(), amount)
        // TODO: .payer_account_id(AccountId::from(6189))
        .signer(&payer_key)
        .execute(&client)
        .await?;

    Ok(())
}
