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

use std::time::Duration;

use clap::Parser;
use hedera::{Client, NodeAddressBookQuery};

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, env, default_value = "testnet")]
    hedera_network: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let args = Args::parse();

    let client = Client::for_name(&args.hedera_network)?;

    let nodes = NodeAddressBookQuery::default()
        .execute_with_timeout(&client, Duration::from_secs(2))
        .await?;

    let _ = dbg!(&(&nodes.node_addresses[0].node_account_id));
    let _ = dbg!(std::str::from_utf8(
        &nodes.node_addresses[0].tls_certificate_hash
    ));

    Ok(())
}
