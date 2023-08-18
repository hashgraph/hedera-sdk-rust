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

use crate::transaction::TransactionExecute;
use crate::{
    AccountId,
    AccountInfoQuery,
    Client,
    Error,
    Key,
    PublicKey,
    Transaction,
};

async fn query_pk(client: &Client, account_id: AccountId) -> crate::Result<PublicKey> {
    let key = AccountInfoQuery::new().account_id(account_id).execute(client).await?.key;

    match key {
        Key::Single(it) => Ok(it),
        _ => {
            Err(Error::signature_verify("`{account_id}`: unsupported key kind: {key:?}".to_owned()))
        }
    }
}

/// Verify the `signature` for `msg` via the given account's public key.
///
/// # Errors
/// - [`Error::SignatureVerify`] if the signature algorithm doesn't match the account's public key.
/// - [`Error::SignatureVerify`] if the signature is invalid for the account's public key.
/// - See [`AccountInfoQuery::execute`]
pub async fn verify_signature(
    client: &Client,
    account_id: AccountId,
    msg: &[u8],
    signature: &[u8],
) -> crate::Result<()> {
    let key = query_pk(client, account_id).await?;

    key.verify(msg, signature)
}

/// Returns `Ok(())` if the given account's public key has signed the given transaction.
/// # Errors
/// - [`Error::SignatureVerify`] if the private key associated with the account's public key did _not_ sign this transaction,
///   or the signature associated was invalid.
/// - See [`AccountInfoQuery::execute`]
pub async fn verify_transaction_signature<D: TransactionExecute>(
    client: &Client,
    account_id: AccountId,
    transaction: &mut Transaction<D>,
) -> crate::Result<()> {
    let key = query_pk(client, account_id).await?;

    key.verify_transaction(transaction)
}
