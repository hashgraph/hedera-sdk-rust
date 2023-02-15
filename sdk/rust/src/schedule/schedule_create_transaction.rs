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
use hedera_proto::services::schedule_service_client::ScheduleServiceClient;
use time::OffsetDateTime;
use tonic::transport::Channel;

use super::schedulable_transaction_body::SchedulableTransactionBody;
use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::transaction::{
    AnyTransactionData,
    ChunkInfo,
    ToSchedulableTransactionDataProtobuf,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::{
    AccountId,
    BoxGrpcFuture,
    Error,
    Key,
    LedgerId,
    Transaction,
    ValidateChecksums,
};

/// Create a new schedule entity (or simply, schedule) in the network's action queue.
///
/// Upon `SUCCESS`, the receipt contains the `ScheduleId` of the created schedule. A schedule
/// entity includes a `scheduled_transaction_body` to be executed.
///
/// When the schedule has collected enough signing keys to satisfy the schedule's signing
/// requirements, the schedule can be executed.
///
pub type ScheduleCreateTransaction = Transaction<ScheduleCreateTransactionData>;

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", default))]
pub struct ScheduleCreateTransactionData {
    scheduled_transaction: Option<SchedulableTransactionBody>,

    schedule_memo: Option<String>,

    admin_key: Option<Key>,

    payer_account_id: Option<AccountId>,

    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::TimestampNanoSeconds>>")
    )]
    expiration_time: Option<OffsetDateTime>,

    wait_for_expiry: bool,
}

impl ScheduleCreateTransaction {
    // note(sr): not sure what the right way to go about this is?
    // pub fn get_scheduled_transaction(&self) -> Option<&SchedulableTransactionBody> {
    //     self.data().scheduled_transaction.as_ref()
    // }

    /// Sets the scheduled transaction.
    ///
    /// # Panics
    /// panics if the transaction is not schedulable, a transaction can be non-schedulable due to:
    /// - being a transaction kind that's non-schedulable, IE, `EthereumTransaction`, or
    /// - being a chunked transaction with multiple chunks.
    pub fn scheduled_transaction<D>(&mut self, transaction: Transaction<D>) -> &mut Self
    where
        D: TransactionExecute,
    {
        let body = transaction.into_body();

        // this gets infered right but `foo.into().try_into()` looks really really weird.
        let data: AnyTransactionData = body.data.into();

        self.data_mut().scheduled_transaction = Some(SchedulableTransactionBody {
            max_transaction_fee: body.max_transaction_fee,
            transaction_memo: body.transaction_memo,
            data: Box::new(data.try_into().unwrap()),
        });

        self
    }

    /// Returns the timestamp for when the transaction should be evaluated for execution and then expire.
    #[must_use]
    pub fn get_expiration_time(&self) -> Option<OffsetDateTime> {
        self.data().expiration_time
    }

    /// Sets the timestamp for when the transaction should be evaluated for execution and then expire.
    pub fn expiration_time(&mut self, time: OffsetDateTime) -> &mut Self {
        self.data_mut().expiration_time = Some(time);
        self
    }

    /// Returns `true` if the transaction will be evaluated at `expiration_time` instead
    /// of when all the required signatures are received, `false` otherwise.
    #[must_use]
    pub fn get_wait_for_expiry(&self) -> bool {
        self.data().wait_for_expiry
    }

    /// Sets if the transaction will be evaluated for execution at `expiration_time` instead
    /// of when all required signatures are received.
    pub fn wait_for_expiry(&mut self, wait: bool) -> &mut Self {
        self.data_mut().wait_for_expiry = wait;
        self
    }

    /// Returns the id of the account to be charged the service fee for the scheduled transaction at
    /// the consensus time it executes (if ever).
    #[must_use]
    pub fn get_payer_account_id(&self) -> Option<AccountId> {
        self.data().payer_account_id
    }

    /// Sets the id of the account to be charged the service fee for the scheduled transaction at
    /// the consensus time that it executes (if ever).
    pub fn payer_account_id(&mut self, id: AccountId) -> &mut Self {
        self.data_mut().payer_account_id = Some(id);
        self
    }

    /// Returns the memo for the schedule entity.
    #[must_use]
    pub fn get_schedule_memo(&self) -> Option<&str> {
        self.data().schedule_memo.as_deref()
    }

    /// Sets the memo for the schedule entity.
    pub fn schedule_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.data_mut().schedule_memo = Some(memo.into());
        self
    }

    /// Returns the Hedera key which can be used to sign a `ScheduleDelete` and remove the schedule.
    #[must_use]
    pub fn get_admin_key(&self) -> Option<&Key> {
        self.data().admin_key.as_ref()
    }

    /// Sets the Hedera key which can be used to sign a `ScheduleDelete` and remove the schedule.
    pub fn admin_key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.data_mut().admin_key = Some(key.into());
        self
    }
}

impl TransactionData for ScheduleCreateTransactionData {}

impl TransactionExecute for ScheduleCreateTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { ScheduleServiceClient::new(channel).create_schedule(request).await })
    }
}

impl ValidateChecksums for ScheduleCreateTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.payer_account_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for ScheduleCreateTransactionData {
    // not really anything I can do about this
    #[allow(clippy::too_many_lines)]
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        let body = self.scheduled_transaction.as_ref().map(|scheduled| {
            let data = scheduled.data.to_schedulable_transaction_data_protobuf();

            services::SchedulableTransactionBody {
                data: Some(data),
                memo: scheduled.transaction_memo.clone(),
                // FIXME: does not use the client to default the max transaction fee
                transaction_fee: scheduled
                    .max_transaction_fee
                    .unwrap_or_else(|| scheduled.data.default_max_transaction_fee())
                    .to_tinybars() as u64,
            }
        });

        let payer_account_id = self.payer_account_id.to_protobuf();
        let admin_key = self.admin_key.to_protobuf();
        let expiration_time = self.expiration_time.map(Into::into);

        services::transaction_body::Data::ScheduleCreate(services::ScheduleCreateTransactionBody {
            scheduled_transaction_body: body,
            memo: self.schedule_memo.clone().unwrap_or_default(),
            admin_key,
            payer_account_id,
            expiration_time,
            wait_for_expiry: self.wait_for_expiry,
        })
    }
}

impl From<ScheduleCreateTransactionData> for AnyTransactionData {
    fn from(transaction: ScheduleCreateTransactionData) -> Self {
        Self::ScheduleCreate(transaction)
    }
}

impl FromProtobuf<services::ScheduleCreateTransactionBody> for ScheduleCreateTransactionData {
    fn from_protobuf(pb: services::ScheduleCreateTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            scheduled_transaction: Option::from_protobuf(pb.scheduled_transaction_body)?,
            schedule_memo: Some(pb.memo),
            admin_key: Option::from_protobuf(pb.admin_key)?,
            payer_account_id: Option::from_protobuf(pb.payer_account_id)?,
            expiration_time: pb.expiration_time.map(Into::into),
            wait_for_expiry: pb.wait_for_expiry,
        })
    }
}
