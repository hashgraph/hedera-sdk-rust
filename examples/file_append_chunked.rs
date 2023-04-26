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
    AccountId, FileAppendTransaction, FileContentsQuery, FileCreateTransaction, Hbar, PrivateKey
};

mod resources;

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
    let _ = dotenvy::dotenv().ok();

    let args = Args::parse();

    let client = hedera::Client::for_name(&args.hedera_network)?;

    client.set_operator(args.operator_account_id, args.operator_key.clone());

    let response = FileCreateTransaction::new()
        .keys([args.operator_key.public_key()])
        .contents("[sdk::rust::example::file_append_chunked]\n\n")
        .max_transaction_fee(Hbar::new(2))
        .execute(&client)
        .await?;

    let receipt = response.get_receipt(&client).await?;

    let file_id = receipt.file_id.unwrap();

    println!("file_id: {file_id}");

    let responses = FileAppendTransaction::new()
        .node_account_ids([response.node_account_id])
        .file_id(file_id)
        .contents(resources::BIG_CONTENTS)
        .max_transaction_fee(Hbar::new(5))
        .execute_all(&client)
        .await?;

    let _ = responses.last().unwrap().get_receipt(&client).await?;

    let contents = FileContentsQuery::new()
        .file_id(file_id)
        .execute(&client)
        .await?;

    println!(
        "File content size according to `FileInfoQuery`: `{}` bytes",
        contents.contents.len()
    );

    Ok(())
}
