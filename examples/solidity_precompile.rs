mod contract;
mod resources;

use clap::Parser;
use contract::{ContractHelper, ContractStep};
use hedera::{
    AccountCreateTransaction, AccountId, Client, ContractFunctionParameters, Hbar, PrivateKey
};

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, env)]
    operator_account_id: AccountId,

    #[clap(long, env)]
    operator_key: PrivateKey,

    #[clap(long, env, default_value = "testnet")]
    hedera_network: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let args = Args::parse();

    let client = Client::for_name(&args.hedera_network)?;

    client.set_operator(args.operator_account_id, args.operator_key.clone());

    // We need a new account for the contract to interact with in some of its steps

    let alice_private_key = PrivateKey::generate_ed25519();
    let alice_public_key = alice_private_key.public_key();
    let alice_account_id = AccountCreateTransaction::new()
        .key(alice_public_key)
        .initial_balance(Hbar::from_tinybars(1000))
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .account_id
        .unwrap();

    // Instantiate ContractHelper

    let mut constructor_params = ContractFunctionParameters::new();

    constructor_params
        .add_address(&args.operator_account_id.to_solidity_address()?)
        .add_address(&alice_account_id.to_solidity_address()?);

    let contract_id = contract::create_contract(
        &client,
        &resources::precompile_bytecode(),
        constructor_params,
    )
    .await?;
    let steps = vec![
        // step 0 tests pseudo random number generator (PRNG)
        ContractStep {
            result_inspector: Some(|result| {
                println!(
                    "getPseudoRandomSeed() returned {:?}",
                    result.get_bytes32(0).map(hex::encode)
                )
            }),
            ..Default::default()
        },
        // step 1 creates a fungible token
        ContractStep {
            payable_amount: Some(Hbar::new(20)),
            signers: vec![alice_private_key.clone()],
            ..Default::default()
        },
        // step 2 mints it
        Default::default(),
        // step 3 associates Alice with it (which requires Alice's signature)
        ContractStep {
            signers: vec![alice_private_key.clone()],
            ..Default::default()
        },
        // step 4 transfers it to Alice.
        Default::default(),
        // step 5 approves an allowance of the fungible token with operator as the owner and Alice as the spender [NOT WORKING]
        ContractStep {
            signers: vec![alice_private_key.clone()],
            ..Default::default()
        },
        // steps 6 - 10 test misc functions on the fungible token (see PrecompileExample.sol for details).
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        // step 11 creates an NFT token with a custom fee, and with the admin and supply set to Alice's key
        ContractStep {
            parameters: Some(Box::new(move || {
                let mut params = ContractFunctionParameters::new();
                // when contracts work with a public key, they handle the raw bytes of the public key
                params.add_bytes(&alice_public_key.to_bytes_raw());
                params
            })),
            payable_amount: Some(Hbar::new(40)),
            // Because we're setting the adminKey for the created NFT token to Alice's key,
            // Alice must sign the ContractExecuteTransaction.
            signers: vec![alice_private_key.clone()],
            ..Default::default()
        },
        // step 12 mints some NFTs
        // and Alice must sign for minting because her key is the supply key.
        ContractStep {
            signers: vec![alice_private_key.clone()],
            parameters: Some(Box::new(|| {
                let mut params = ContractFunctionParameters::new();
                // add three metadatas
                params.add_bytes_array(&[&[0x1b, 0x2b, 0x3b]]);
                params
            })),

            ..Default::default()
        },
        // step 13 associates Alice with the NFT token
        // and alice must sign to become associated with the token.
        ContractStep {
            signers: vec![alice_private_key.clone()],
            ..Default::default()
        },
        // step 14 transfers some NFTs to Alice
        Default::default(),
        // step 15 approves an NFT allowance with operator as the owner and Alice as the spender [NOT WORKING]
        Default::default(),
        // step 16 burn some NFTs
        ContractStep {
            signers: vec![alice_private_key],
            ..Default::default()
        },
    ];

    ContractHelper::new(contract_id, steps)
        .execute(&client)
        .await?;

    println!("All steps completed with valid results.");

    Ok(())
}
