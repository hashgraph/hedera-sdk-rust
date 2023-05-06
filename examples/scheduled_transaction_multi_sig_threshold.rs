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
    AccountBalanceQuery, AccountCreateTransaction, AccountId, Client, Hbar, Key, KeyList, PrivateKey, ScheduleInfoQuery, ScheduleSignTransaction, TransactionRecordQuery, TransferTransaction
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
    // Generate four new Ed25519 private, public key pairs.

    let mut private_keys = Vec::with_capacity(4);
    let mut public_keys = Vec::with_capacity(4);

    for i in 0..4 {
        let key = PrivateKey::generate_ed25519();
        public_keys.push(key.public_key());

        println!("public key {}: {}", i + 1, key.public_key());
        println!("private key {}, {}", i + 1, key);

        private_keys.push(key);
    }

    // require 3 of the 4 keys we generated to sign on anything modifying this account
    let transaction_key = KeyList {
        keys: public_keys.iter().cloned().map(Key::from).collect(),
        threshold: Some(3),
    };

    let receipt = AccountCreateTransaction::new()
        .key(transaction_key)
        .initial_balance(Hbar::from_tinybars(1))
        .account_memo("3-of-4 multi-sig account")
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let multi_sig_account_id = receipt.account_id.unwrap();

    println!("3-of-4 multi-sig account ID: {multi_sig_account_id}");

    let balance = AccountBalanceQuery::new()
        .account_id(multi_sig_account_id)
        .execute(&client)
        .await?;

    println!(
        "Balance of account {multi_sig_account_id}: {}.",
        balance.hbars
    );

    // schedule crypto transfer from multi-sig account to operator account
    let mut transfer_transaction = TransferTransaction::new();
    transfer_transaction
        .hbar_transfer(multi_sig_account_id, Hbar::from_tinybars(-1))
        .hbar_transfer(args.operator_account_id, Hbar::from_tinybars(1));

    let tx_schedule_receipt = transfer_transaction
        .schedule()
        .freeze_with(&client)?
        .sign(private_keys[0].clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    println!("Schedule status: {:?}", tx_schedule_receipt.status);
    let schedule_id = tx_schedule_receipt.schedule_id.unwrap();
    println!("Schedule ID: {schedule_id}");
    let scheduled_tx_id = tx_schedule_receipt.scheduled_transaction_id.unwrap();
    println!("Scheduled tx ID: {scheduled_tx_id}");

    // add 2 signature
    let tx_schedule_sign1_receipt = ScheduleSignTransaction::new()
        .schedule_id(schedule_id)
        .freeze_with(&client)?
        .sign(private_keys[1].clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    println!(
        "1. ScheduleSignTransaction status: {:?}",
        tx_schedule_sign1_receipt.status
    );

    // add 3 signature
    let tx_schedule_sign2_receipt = ScheduleSignTransaction::new()
        .schedule_id(schedule_id)
        .freeze_with(&client)?
        .sign(private_keys[2].clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    println!(
        "2. ScheduleSignTransaction status: {:?}",
        tx_schedule_sign2_receipt.status
    );

    // query schedule
    let schedule_info = ScheduleInfoQuery::new()
        .schedule_id(schedule_id)
        .execute(&client)
        .await?;

    println!("{:?}", schedule_info);

    // query triggered scheduled tx
    let record_scheduled_tx = TransactionRecordQuery::new()
        .transaction_id(scheduled_tx_id)
        .execute(&client)
        .await?;

    println!("{:?}", record_scheduled_tx);

    Ok(())
}
