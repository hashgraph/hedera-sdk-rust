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

use assert_matches::assert_matches;
use clap::Parser;
use hedera::{AccountCreateTransaction, AccountId, Client, PrivateKey};

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, env)]
    operator_account_id: AccountId,

    #[clap(long, env)]
    operator_key: PrivateKey,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let args = Args::parse();

    let client = Client::for_testnet()?;

    client.set_operator(args.operator_account_id, args.operator_key);

    let new_key = PrivateKey::generate_ed25519();

    println!("private key = {new_key}");
    println!("public key = {}", new_key.public_key());

    let response = AccountCreateTransaction::new()
        .key(new_key.public_key())
        .execute(&client)
        .await?;

    let receipt = response.get_receipt(&client).await?;

    let new_account_id = assert_matches!(receipt.account_id, Some(id) => id);

    println!("account address = {new_account_id}");

    Ok(())
}
