/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use std::iter::repeat;

use clap::Parser;
use hedera::{
    AccountBalanceQuery, AccountCreateTransaction, AccountId, Client, Hbar, PrivateKey, TokenAirdropTransaction, TokenAssociateTransaction, TokenCreateTransaction, TokenDeleteTransaction, TokenGrantKycTransaction, TokenMintTransaction, TokenWipeTransaction, TransferTransaction
};
use time::{Duration, OffsetDateTime};

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
    let Args {
        operator_account_id,
        operator_key,
        hedera_network,
    } = Args::parse();

    let client = Client::for_name(&hedera_network)?;

    client.set_operator(operator_account_id, operator_key.clone());
    let private_key_1 = PrivateKey::generate_ed25519();
    let alice = AccountCreateTransaction::new()
        .key(private_key_1.public_key())
        .initial_balance(Hbar::new(10))
        .max_automatic_token_associations(-1)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .account_id
        .unwrap();

    let private_key_2 = PrivateKey::generate_ed25519();
    let bob = AccountCreateTransaction::new()
        .key(private_key_2.public_key())
        .max_automatic_token_associations(1)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .account_id
        .unwrap();

    let private_key_3 = PrivateKey::generate_ed25519();
    let carol = AccountCreateTransaction::new()
        .key(private_key_3.public_key())
        .max_automatic_token_associations(0)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .account_id
        .unwrap();

    let treasury_key = PrivateKey::generate_ed25519();
    let treasury_account_id = AccountCreateTransaction::new()
        .key(treasury_key.public_key())
        .initial_balance(Hbar::new(10))
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .account_id
        .unwrap();

    /*
     * Step 2:
     * Create FT and NFT and mint
     */
    let token_id = TokenCreateTransaction::new()
        .name("ffff")
        .symbol("F")
        .decimals(3)
        .initial_supply(100)
        .max_supply(100)
        .treasury_account_id(treasury_account_id)
        .token_supply_type(hedera::TokenSupplyType::Finite)
        .admin_key(operator_key.clone().public_key())
        .freeze_key(operator_key.clone().public_key())
        .supply_key(operator_key.clone().public_key())
        .pause_key(operator_key.clone().public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::hours(2))
        .freeze_with(&client)?
        .sign(treasury_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    let nft_id = TokenCreateTransaction::new()
        .name("example NFT")
        .symbol("F")
        .decimals(3)
        .max_supply(10)
        .treasury_account_id(treasury_account_id)
        .token_supply_type(hedera::TokenSupplyType::Finite)
        .token_type(hedera::TokenType::NonFungibleUnique)
        .admin_key(operator_key.clone().public_key())
        .freeze_key(operator_key.clone().public_key())
        .supply_key(operator_key.clone().public_key())
        .pause_key(operator_key.clone().public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::hours(2))
        .freeze_with(&client)?
        .sign(treasury_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    _ = TokenMintTransaction::new()
        .token_id(nft_id)
        .metadata(repeat(vec![9, 1, 6]).take(4).collect::<Vec<Vec<_>>>())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    /*
     * Step 3:
     * Airdrop fungible tokens to all 3 accounts
     */
    println!("Airdropping tokens to all accounts");

    let airdrop_record = TokenAirdropTransaction::new()
        .token_transfer(token_id, alice, 10)
        .token_transfer(token_id, treasury_account_id, -10)
        .token_transfer(token_id, bob, 10)
        .token_transfer(token_id, treasury_account_id, -10)
        .token_transfer(token_id, carol, 10)
        .token_transfer(token_id, treasury_account_id, -10)
        .execute(&client)
        .await?
        .get_record(&client)
        .await?;

    /*
     * Step 4:
     * Get the transaction record and see one pending airdrop (for carol)
     */
    println!(
        "Pending airdrop length: {}",
        airdrop_record.pending_airdrop_records.len()
    );
    println!(
        "Pending airdrops: {:?}",
        airdrop_record.pending_airdrop_records.get(0)
    );

    /*
     * Step 5:
     * Query to verify alice and bob received the airdrops and carol did not
     */
    let alice_balance = AccountBalanceQuery::new()
        .account_id(alice)
        .execute(&client)
        .await?;

    let bob_balance = AccountBalanceQuery::new()
        .account_id(bob)
        .execute(&client)
        .await?;

    let carol_balance = AccountBalanceQuery::new()
        .account_id(carol)
        .execute(&client)
        .await?;

    println!(
        "Alice ft balance after airdrop: {}",
        alice_balance.tokens.get(&token_id).unwrap()
    );
    println!(
        "Bob ft balance after airdrop: {}",
        bob_balance.tokens.get(&token_id).unwrap()
    );
    println!(
        "Carol ft balance after airdrop: {}",
        carol_balance.tokens.get(&token_id).unwrap()
    );

    Ok(())
}
