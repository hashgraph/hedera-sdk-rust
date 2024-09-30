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
use hedera::{AccountId, Client, Hbar, PrivateKey, TransferTransaction};

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, env)]
    operator_account_id: AccountId,

    #[clap(long, env)]
    operator_key: PrivateKey,

    #[clap(long)]
    sender: Option<AccountId>,

    #[clap(long, default_value = "0.0.1001")]
    receiver: AccountId,

    #[clap(long, default_value = "10 μℏ")]
    amount: Hbar,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let args = Args::parse();

    let client = Client::for_testnet()?;

    client.set_operator(args.operator_account_id, args.operator_key);

    let sender = args.sender.unwrap_or(args.operator_account_id);

    let _ = TransferTransaction::new()
        .hbar_transfer(sender, -args.amount)
        .hbar_transfer(args.receiver, args.amount)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    Ok(())
}
