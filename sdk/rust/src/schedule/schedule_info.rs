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

use hedera_proto::services;
use prost::Message;
use time::OffsetDateTime;

use super::schedulable_transaction_body::SchedulableTransactionBody;
use crate::protobuf::ToProtobuf;
use crate::transaction::TransactionBody;
use crate::{
    AccountId,
    AnyTransaction,
    FromProtobuf,
    Key,
    KeyList,
    LedgerId,
    ScheduleId,
    Transaction,
    TransactionId,
};

// TODO: scheduled_transaction
/// Response from [`ScheduleInfoQuery`][crate::ScheduleInfoQuery].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct ScheduleInfo {
    /// The ID of the schedule for which information is requested.
    pub schedule_id: ScheduleId,

    /// The account that created the scheduled transaction.
    pub creator_account_id: AccountId,

    /// The account paying for the execution of the scheduled transaction.
    pub payer_account_id: Option<AccountId>,

    /// The signatories that have provided signatures so far for the schedule
    /// transaction.
    pub signatories: KeyList,

    /// The key which is able to delete the schedule transaction if set.
    pub admin_key: Option<Key>,

    /// The transaction id that will be used in the record of the scheduled transaction (if
    /// it executes).
    pub scheduled_transaction_id: TransactionId,

    scheduled_transaction: SchedulableTransactionBody,

    /// When set to true, the transaction will be evaluated for execution at `expiration_time`
    /// instead of when all required signatures are received.
    pub wait_for_expiry: bool,

    /// Publicly visible information about the Schedule entity.
    pub memo: String,

    /// The date and time the schedule transaction will expire
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::TimestampNanoSeconds>>")
    )]
    pub expiration_time: Option<OffsetDateTime>,

    /// The time the schedule transaction was executed.
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::TimestampNanoSeconds>>")
    )]
    pub executed_at: Option<OffsetDateTime>,

    /// The time the schedule transaction was deleted.
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::TimestampNanoSeconds>>")
    )]
    pub deleted_at: Option<OffsetDateTime>,

    /// The ledger ID the response was returned from
    pub ledger_id: LedgerId,
}

impl ScheduleInfo {
    /// Create a new `ScheduleInfo` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::<services::ScheduleInfo>::from_bytes(bytes)
    }

    /// Returns the scheduled transaction.
    ///
    /// This is *not* guaranteed to be a constant time operation.
    pub fn scheduled_transaction(&self) -> crate::Result<AnyTransaction> {
        // note: this can't error *right now* but the API *will* be faliable eventually, and as such, returns a result to make the change non-breaking.
        Ok(Transaction::from_parts(
            TransactionBody {
                data: (*self.scheduled_transaction.data).clone().into(),
                node_account_ids: None,
                transaction_valid_duration: None,
                max_transaction_fee: None,
                transaction_memo: self.scheduled_transaction.transaction_memo.clone(),
                transaction_id: Some(self.scheduled_transaction_id),
                operator: None,
                is_frozen: true,
            },
            Vec::new(),
        ))
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        services::ScheduleInfo {
            schedule_id: Some(self.schedule_id.to_protobuf()),
            expiration_time: self.expiration_time.to_protobuf(),
            memo: self.memo.clone(),
            admin_key: self.admin_key.to_protobuf(),
            signers: (!self.signatories.is_empty())
                .then(|| services::KeyList { keys: self.signatories.keys.to_protobuf() }),
            creator_account_id: Some(self.creator_account_id.to_protobuf()),
            payer_account_id: self.payer_account_id.to_protobuf(),
            scheduled_transaction_id: Some(self.scheduled_transaction_id.to_protobuf()),
            ledger_id: self.ledger_id.to_bytes(),
            wait_for_expiry: self.wait_for_expiry,

            // unimplemented fields
            scheduled_transaction_body: Some(
                self.scheduled_transaction.to_scheduled_body_protobuf(),
            ),
            data: None,
        }
        .encode_to_vec()
    }
}

impl FromProtobuf<services::response::Response> for ScheduleInfo {
    #[allow(deprecated)]
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, ScheduleGetInfo, services::response::Response);
        let info = pb_getf!(response, schedule_info)?;
        Self::from_protobuf(info)
    }
}

impl FromProtobuf<services::ScheduleInfo> for ScheduleInfo {
    fn from_protobuf(pb: services::ScheduleInfo) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let schedule_id = pb_getf!(pb, schedule_id)?;
        let creator_account_id = pb_getf!(pb, creator_account_id)?;
        let payer_account_id = Option::from_protobuf(pb.payer_account_id)?;
        let admin_key = Option::from_protobuf(pb.admin_key)?;
        let ledger_id = LedgerId::from_bytes(pb.ledger_id);

        let scheduled_transaction_id =
            TransactionId::from_protobuf(pb_getf!(pb, scheduled_transaction_id)?)?;

        let transaction_body =
            SchedulableTransactionBody::from_protobuf(pb_getf!(pb, scheduled_transaction_body)?)?;

        let signatories = pb.signers.map(KeyList::from_protobuf).transpose()?.unwrap_or_default();

        let (executed_at, deleted_at) = match pb.data {
            Some(services::schedule_info::Data::DeletionTime(deleted)) => {
                (None, Some(deleted.into()))
            }

            Some(services::schedule_info::Data::ExecutionTime(executed)) => {
                (Some(executed.into()), None)
            }

            None => (None, None),
        };

        Ok(Self {
            schedule_id: ScheduleId::from_protobuf(schedule_id)?,
            executed_at,
            deleted_at,
            memo: pb.memo,
            creator_account_id: AccountId::from_protobuf(creator_account_id)?,
            payer_account_id,
            expiration_time: pb.expiration_time.map(Into::into),
            admin_key,
            scheduled_transaction_id,
            signatories,
            wait_for_expiry: pb.wait_for_expiry,
            ledger_id,
            scheduled_transaction: transaction_body,
        })
    }
}
