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
use futures_util::{stream, StreamExt, TryStreamExt};
use hedera::{
    AccountId, Client, NftId, PrivateKey, TokenCreateTransaction, TokenId, TokenMintTransaction, TokenNftInfoQuery, TokenType, TokenUpdateNftsTransaction
};
use time::{Duration, OffsetDateTime};

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, env)]
    operator_account_id: AccountId,

    #[clap(long, env)]
    operator_key: PrivateKey,

    #[clap(long, env, default_value = "localnode")]
    hedera_network: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    let args = Args::parse();

    let client = Client::for_name(&args.hedera_network)?;

    client.set_operator(args.operator_account_id, args.operator_key);

    let metadata_key = PrivateKey::generate_ed25519();
    let nft_count = 4;
    let initial_metadata_list: Vec<Vec<u8>> = repeat(vec![9, 1, 6]).take(nft_count).collect();
    let updated_metadata: Vec<u8> = vec![3, 4];

    let token_id = TokenCreateTransaction::new()
        .name("ffff")
        .symbol("F")
        .token_type(TokenType::NonFungibleUnique)
        .treasury_account_id(client.get_operator_account_id().unwrap())
        .admin_key(client.get_operator_public_key().unwrap())
        .supply_key(client.get_operator_public_key().unwrap())
        .metadata_key(metadata_key.public_key())
        .expiration_time(OffsetDateTime::now_utc() + Duration::minutes(5))
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .token_id
        .unwrap();

    // Mint the token
    let serials = TokenMintTransaction::new()
        .metadata(initial_metadata_list.clone())
        .token_id(token_id)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .serials;

    println!(
        "Metadata after mint= {:?}",
        get_metadata_list(&client, &token_id, &serials).await?
    );

    let serials = TokenUpdateNftsTransaction::new()
        .token_id(token_id)
        .serials(serials.into_iter().take(2).collect())
        .metadata(updated_metadata)
        .sign(metadata_key)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .serials;

    // Check if metadata has updated correctly
    println!(
        "Metadata after mint= {:?}",
        get_metadata_list(&client, &token_id, &serials).await?
    );

    Ok(())
}

async fn get_metadata_list(
    client: &Client,
    token_id: &TokenId,
    serials: &Vec<i64>,
) -> anyhow::Result<Vec<Vec<u8>>> {
    let list = stream::iter(serials.into_iter().map(|it| NftId {
        token_id: token_id.to_owned(),
        serial: *it as u64,
    }))
    .then(|nft_id| {
        let client_clone = client;
        async move {
            match TokenNftInfoQuery::new()
                .nft_id(nft_id)
                .execute(&client_clone)
                .await
            {
                Ok(info) => Ok(info.metadata),
                Err(err) => anyhow::bail!("error calling TokenNftInfoQuery: {err}"), // CHANGE ERROR MESSAGE
            }
        }
    })
    .try_collect::<Vec<_>>()
    .await?;

    Ok(list)
}
