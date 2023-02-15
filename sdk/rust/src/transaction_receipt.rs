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

use std::ops::Not;

use hedera_proto::services;

use crate::protobuf::ToProtobuf;
use crate::{
    AccountId,
    ContractId,
    Error,
    FileId,
    FromProtobuf,
    ScheduleId,
    Status,
    TokenId,
    TopicId,
    TransactionId,
};

/// The summary of a transaction's result so far, if the transaction has reached consensus.
/// Response from [`TransactionReceiptQuery`][crate::TransactionReceiptQuery].

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct TransactionReceipt {
    // fixme(sr): better doc comment.
    /// The ID of the transaction that this is a receipt for.
    pub transaction_id: Option<TransactionId>,

    /// The consensus status of the transaction; is UNKNOWN if consensus has not been reached, or if
    /// the associated transaction did not have a valid payer signature.
    pub status: Status,

    /// In the receipt for an `AccountCreateTransaction`, the id of the newly created account.
    pub account_id: Option<AccountId>,

    /// In the receipt for a `FileCreateTransaction`, the id of the newly created file.
    pub file_id: Option<FileId>,

    /// In the receipt for a `ContractCreateTransaction`, the id of the newly created contract.
    pub contract_id: Option<ContractId>,

    // The exchange rates in effect when the transaction reached consensus.
    // TODO: pub exchange_rate: ExchangeRate,
    /// In the receipt for a `TopicCreateTransaction`, the id of the newly created topic.
    pub topic_id: Option<TopicId>,

    /// In the receipt for a `TopicMessageSubmitTransaction`, the new sequence number of the topic
    /// that received the message.
    pub topic_sequence_number: u64,

    // TODO: use a hash type (for display/debug/serialize purposes)
    /// In the receipt for a `TopicMessageSubmitTransaction`, the new running hash of the
    /// topic that received the message.
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::base64::Base64>>")
    )]
    pub topic_running_hash: Option<Vec<u8>>,

    /// In the receipt of a `TopicMessageSubmitTransaction`, the version of the SHA-384
    /// digest used to update the running hash.
    pub topic_running_hash_version: u64,

    /// In the receipt for a `TokenCreateTransaction`, the id of the newly created token.
    pub token_id: Option<TokenId>,

    /// Populated in the receipt of `TokenMint`, `TokenWipe`, and `TokenBurn` transactions.
    ///
    /// For fungible tokens, the current total supply of this token.
    /// For non-fungible tokens, the total number of NFTs issued for a given token id.
    pub total_supply: u64,

    /// In the receipt for a `ScheduleCreateTransaction`, the id of the newly created schedule.
    pub schedule_id: Option<ScheduleId>,

    /// In the receipt of a `ScheduleCreateTransaction` or `ScheduleSignTransaction` that resolves
    /// to `Success`, the `TransactionId` that should be used to query for the receipt or
    /// record of the relevant scheduled transaction.
    pub scheduled_transaction_id: Option<TransactionId>,

    /// In the receipt of a `TokenMintTransaction` for tokens of type `NonFungibleUnique`,
    /// the serial numbers of the newly created NFTs.
    pub serials: Vec<i64>,

    /// The receipts of processing all transactions with the given id, in consensus time order.
    pub duplicates: Vec<TransactionReceipt>,

    /// The receipts (if any) of all child transactions spawned by the transaction with the
    /// given top-level id, in consensus order.
    pub children: Vec<TransactionReceipt>,
}

impl TransactionReceipt {
    /// Create a new `FileInfo` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`] if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`] if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::<services::TransactionReceipt>::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        ToProtobuf::to_bytes(self)
    }

    /// Validate [`status`](Self.status) and return an `Err` if it isn't [`Status::Ok`]
    ///
    /// # Errors
    /// - [`Error::ReceiptStatus`] if `validate && self.status != Status::Ok`
    pub fn validate_status(&self, validate: bool) -> crate::Result<&Self> {
        if validate && self.status != Status::Ok {
            Err(Error::ReceiptStatus { status: self.status, transaction_id: self.transaction_id })
        } else {
            Ok(self)
        }
    }

    fn from_protobuf(
        receipt: services::TransactionReceipt,
        duplicates: Vec<Self>,
        children: Vec<Self>,
        transaction_id: Option<&TransactionId>,
    ) -> crate::Result<Self> {
        let status = if let Some(status) = Status::from_i32(receipt.status) {
            status
        } else {
            return Err(Error::ResponseStatusUnrecognized(receipt.status));
        };

        let account_id = Option::from_protobuf(receipt.account_id)?;
        let file_id = Option::from_protobuf(receipt.file_id)?;
        let contract_id = Option::from_protobuf(receipt.contract_id)?;
        let topic_id = Option::from_protobuf(receipt.topic_id)?;
        let token_id = Option::from_protobuf(receipt.token_id)?;
        let schedule_id = Option::from_protobuf(receipt.schedule_id)?;

        let scheduled_transaction_id = Option::from_protobuf(receipt.scheduled_transaction_id)?;

        Ok(Self {
            status,
            total_supply: receipt.new_total_supply,
            serials: receipt.serial_numbers,
            topic_running_hash_version: receipt.topic_running_hash_version,
            topic_sequence_number: receipt.topic_sequence_number,
            topic_running_hash: receipt
                .topic_running_hash
                .is_empty()
                .not()
                .then_some(receipt.topic_running_hash),
            scheduled_transaction_id,
            account_id,
            file_id,
            contract_id,
            topic_id,
            token_id,
            schedule_id,
            duplicates,
            children,
            transaction_id: transaction_id.copied(),
        })
    }

    pub(crate) fn from_response_protobuf(
        pb: services::response::Response,
        transaction_id: Option<&TransactionId>,
    ) -> crate::Result<Self> {
        let pb = pb_getv!(pb, TransactionGetReceipt, services::response::Response);

        let receipt = pb_getf!(pb, receipt)?;

        let duplicates = Vec::from_protobuf(pb.duplicate_transaction_receipts)?;

        let children = Vec::from_protobuf(pb.child_transaction_receipts)?;

        Self::from_protobuf(receipt, duplicates, children, transaction_id)
    }
}

impl FromProtobuf<services::response::Response> for TransactionReceipt {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Self::from_response_protobuf(pb, None)
    }
}

impl FromProtobuf<services::TransactionReceipt> for TransactionReceipt {
    fn from_protobuf(receipt: services::TransactionReceipt) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Self::from_protobuf(receipt, Vec::new(), Vec::new(), None)
    }
}

impl ToProtobuf for TransactionReceipt {
    type Protobuf = services::TransactionReceipt;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::TransactionReceipt {
            status: self.status as i32,
            account_id: self.account_id.to_protobuf(),
            file_id: self.file_id.to_protobuf(),
            contract_id: self.contract_id.to_protobuf(),
            exchange_rate: None,
            topic_id: self.topic_id.to_protobuf(),
            topic_sequence_number: self.topic_sequence_number,
            topic_running_hash: self.topic_running_hash.clone().unwrap_or_default(),
            topic_running_hash_version: self.topic_running_hash_version,
            token_id: self.token_id.to_protobuf(),
            new_total_supply: self.total_supply,
            schedule_id: self.schedule_id.to_protobuf(),
            scheduled_transaction_id: self.scheduled_transaction_id.to_protobuf(),
            serial_numbers: self.serials.clone(),
        }
    }
}
