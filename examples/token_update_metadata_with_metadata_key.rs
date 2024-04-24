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

use clap::Parser;
use hedera::{
    AccountId, Client, PrivateKey, TokenCreateTransaction, TokenInfoQuery, TokenType, TokenUpdateTransaction
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

    let args = Args::parse();

    let client = Client::for_name(&args.hedera_network)?;

    client.set_operator(args.operator_account_id, args.operator_key.clone());

    // Generate a metadata key
    let metadata_key = PrivateKey::generate_ed25519();
    // Initial metadata
    let metadata: Vec<u8> = vec![1];
    // New metadata
    let new_metadata: Vec<u8> = vec![1, 2];

    let token_create_receipt = TokenCreateTransaction::new()
        .name("ffff")
        .symbol("F")
        .token_type(TokenType::FungibleCommon)
        .decimals(3)
        .initial_supply(1000000)
        .metadata(metadata)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .metadata_key(metadata_key.public_key())
        .freeze_with(&client)?
        .sign(metadata_key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    println!(
        "Status of token create transaction: {:?}",
        token_create_receipt.status
    );

    // Get token id
    let token_id = token_create_receipt.token_id.unwrap();
    println!("Token id: {token_id:?}");

    let token_info = TokenInfoQuery::new()
        .token_id(token_id)
        .execute(&client)
        .await?;

    println!("Token metadata: {:?}", token_info.metadata);

    let token_update_receipt = TokenUpdateTransaction::new()
        .token_id(token_id)
        .metadata(new_metadata)
        .freeze_with(&client)?
        .sign(metadata_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    println!(
        "Status of token update transaction: {:?}",
        token_update_receipt.status
    );

    let token_nft_info = TokenInfoQuery::new()
        .token_id(token_id)
        .execute(&client)
        .await?;

    println!("Updated token metadata: {:?}", token_nft_info.metadata);

    Ok(())
}
