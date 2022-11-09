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

use std::collections::HashMap;

use hedera_proto::services;
use time::OffsetDateTime;

use crate::{
    AccountId,
    AssessedCustomFee,
    ContractFunctionResult,
    FromProtobuf,
    Hbar,
    PublicKey,
    ScheduleId,
    Tinybar,
    TokenAssociation,
    TokenId,
    TokenNftTransfer,
    TransactionId,
    TransactionReceipt,
    Transfer,
};

/// The complete record for a transaction on Hedera that has reached consensus.
/// Response from [`TransactionRecordQuery`][crate::TransactionRecordQuery].
#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct TransactionRecord {
    /// The status (reach consensus, or failed, or is unknown) and the ID of
    /// any new account/file/instance created.
    pub receipt: TransactionReceipt,

    /// The hash of the Transaction that executed (not the hash of any Transaction that failed for
    /// having a duplicate TransactionID).
    #[cfg_attr(feature = "ffi", serde(with = "serde_with::As::<serde_with::base64::Base64>"))]
    pub transaction_hash: Vec<u8>,

    /// The consensus timestamp.
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<serde_with::TimestampNanoSeconds>")
    )]
    pub consensus_timestamp: OffsetDateTime,

    /// Record of the value returned by the smart contract function or constructor.
    pub contract_function_result: Option<ContractFunctionResult>,

    /// All hbar transfers as a result of this transaction, such as fees, or
    /// transfers performed by the transaction, or by a smart contract it calls,
    /// or by the creation of threshold records that it triggers.
    pub transfers: Vec<Transfer>,

    /// All fungible token transfers as a result of this transaction.
    pub token_transfers: HashMap<TokenId, HashMap<AccountId, i64>>,

    /// All NFT Token transfers as a result of this transaction.
    pub token_nft_transfers: HashMap<TokenId, Vec<TokenNftTransfer>>,

    /// The ID of the transaction this record represents.
    pub transaction_id: TransactionId,

    /// The memo that was submitted as part of the transaction.
    pub transaction_memo: String,

    /// The actual transaction fee charged.
    pub transaction_fee: Hbar,

    /// Reference to the scheduled transaction ID that this transaction record represents.
    pub schedule_ref: Option<ScheduleId>,

    /// All custom fees that were assessed during a CryptoTransfer, and must be paid if the
    /// transaction status resolved to SUCCESS.
    pub assessed_custom_fees: Vec<AssessedCustomFee>,

    /// All token associations implicitly created while handling this transaction
    pub automatic_token_associations: Vec<TokenAssociation>,

    /// In the record of an internal transaction, the consensus timestamp of the user
    /// transaction that spawned it.
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::TimestampNanoSeconds>>")
    )]
    pub parent_consensus_timestamp: Option<OffsetDateTime>,

    /// In the record of an internal CryptoCreate transaction triggered by a user
    /// transaction with a (previously unused) alias, the new account's alias.
    pub alias_key: Option<PublicKey>,

    /// The records of processing all child transaction spawned by the transaction with the given
    /// top-level id, in consensus order. Always empty if the top-level status is UNKNOWN.
    pub children: Vec<Self>,

    /// The records of processing all consensus transaction with the same id as the distinguished
    /// record above, in chronological order.
    pub duplicates: Vec<Self>,

    /// The keccak256 hash of the ethereumData. This field will only be populated for
    /// `EthereumTransaction`.
    pub ethereum_hash: Vec<u8>,
    // /// In the record of a PRNG transaction with no output range, a pseudorandom 384-bit string.
    // TODO: pub prng_bytes: Vec<u8>,
    //
    // /// In the record of a PRNG transaction with an output range, the output of a PRNG
    // /// whose input was a 384-bit string.
    // TODO: pub prng_number: i32,
}
// TODO: paid_staking_rewards

impl TransactionRecord {
    fn from_protobuf(
        record: services::TransactionRecord,
        duplicates: Vec<Self>,
        children: Vec<Self>,
    ) -> crate::Result<Self> {
        use services::transaction_record::Body;
        let receipt = pb_getf!(record, receipt)?;
        let receipt = TransactionReceipt::from_protobuf(receipt)?;

        let consensus_timestamp = pb_getf!(record, consensus_timestamp)?;
        let transaction_id = pb_getf!(record, transaction_id)?;
        let schedule_ref = Option::from_protobuf(record.schedule_ref)?;
        let parent_consensus_timestamp = record.parent_consensus_timestamp.map(Into::into);

        let alias_key =
            (!record.alias.is_empty()).then(|| PublicKey::from_bytes(&record.alias)).transpose()?;

        let automatic_token_associations = Vec::from_protobuf(record.automatic_token_associations)?;

        let contract_function_result = record.body.map(|it| match it {
            Body::ContractCallResult(it) | Body::ContractCreateResult(it) => it,
        });

        let contract_function_result = Option::from_protobuf(contract_function_result)?;

        let transfers = record.transfer_list.map_or_else(Vec::new, |it| it.account_amounts);
        let transfers = Vec::from_protobuf(transfers)?;

        let (token_transfers, token_nft_transfers) = {
            let mut token_transfers = HashMap::with_capacity(record.token_transfer_lists.len());

            let mut token_nft_transfers: HashMap<TokenId, Vec<TokenNftTransfer>> =
                HashMap::with_capacity(record.token_transfer_lists.len());

            for transfer_list in record.token_transfer_lists {
                let token_id = pb_getf!(transfer_list, token)?;
                let token_id = TokenId::from_protobuf(token_id)?;

                // `.insert` would be the most idiomatic way, but this matches behavior with Java.
                let token_transfers = token_transfers
                    .entry(token_id)
                    .or_insert_with(|| HashMap::with_capacity(transfer_list.transfers.len()));

                for it in transfer_list.transfers {
                    let account_id = AccountId::from_protobuf(pb_getf!(it, account_id)?)?;
                    token_transfers.insert(account_id, it.amount);
                }

                let nft_transfers: Result<Vec<_>, _> = transfer_list
                    .nft_transfers
                    .into_iter()
                    .map(|it| TokenNftTransfer::from_protobuf(it, token_id))
                    .collect();
                let nft_transfers = nft_transfers?;

                token_nft_transfers.entry(token_id).or_default().extend_from_slice(&nft_transfers);
            }

            (token_transfers, token_nft_transfers)
        };

        Ok(Self {
            receipt,
            transaction_hash: record.transaction_hash,
            consensus_timestamp: consensus_timestamp.into(),
            contract_function_result,
            transaction_id: TransactionId::from_protobuf(transaction_id)?,
            transaction_memo: record.memo,
            transaction_fee: Hbar::from_tinybars(record.transaction_fee as Tinybar),
            schedule_ref,
            automatic_token_associations,
            parent_consensus_timestamp,
            duplicates,
            ethereum_hash: record.ethereum_hash,
            children,
            alias_key,
            transfers,
            token_transfers,
            token_nft_transfers,
            assessed_custom_fees: Vec::from_protobuf(record.assessed_custom_fees)?,
        })
    }
}

impl FromProtobuf<services::response::Response> for TransactionRecord {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let pb = pb_getv!(pb, TransactionGetRecord, services::response::Response);

        let record = pb_getf!(pb, transaction_record)?;

        let duplicates = Vec::from_protobuf(pb.duplicate_transaction_records)?;

        let children = Vec::from_protobuf(pb.child_transaction_records)?;

        Self::from_protobuf(record, duplicates, children)
    }
}

impl FromProtobuf<services::TransactionRecord> for TransactionRecord {
    fn from_protobuf(receipt: services::TransactionRecord) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Self::from_protobuf(receipt, Vec::new(), Vec::new())
    }
}
