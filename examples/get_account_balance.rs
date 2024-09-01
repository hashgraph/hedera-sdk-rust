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

use hedera::{AccountBalanceQuery, AccountId, Client, NodeAddressBookQuery};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let client = Client::for_mainnet();
    let client = Client::for_testnet()?;
    dbg!(NodeAddressBookQuery::new()
        .execute(&client)
        .await?
        .node_addresses
        .into_iter()
        .map(|it| (it.node_account_id, it.service_endpoints))
        .collect::<Vec<_>>());

    let id = AccountId::from(7);

    let ab = AccountBalanceQuery::new()
        .account_id(id)
        // .node_account_ids([AccountId::from(7)])
        .execute(&client)
        .await?;

    println!("balance = {}", ab.hbars);

    Ok(())
}
