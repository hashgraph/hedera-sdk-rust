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
use tonic::transport::Channel;

use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    BoxGrpcFuture,
    Error,
    LedgerId,
    ScheduleId,
    Transaction,
    TransactionId,
    ValidateChecksums,
};

/// Adds zero or more signing keys to a schedule.
pub type ScheduleSignTransaction = Transaction<ScheduleSignTransactionData>;

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", default))]
pub struct ScheduleSignTransactionData {
    schedule_id: Option<ScheduleId>,
}

impl ScheduleSignTransaction {
    /// Returns the schedule to add signing keys to.
    #[must_use]
    pub fn get_schedule_id(&self) -> Option<ScheduleId> {
        self.data().schedule_id
    }

    /// Sets the schedule to add signing keys to.
    pub fn schedule_id(&mut self, id: ScheduleId) -> &mut Self {
        self.data_mut().schedule_id = Some(id);
        self
    }
}

impl TransactionExecute for ScheduleSignTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { ScheduleServiceClient::new(channel).delete_schedule(request).await })
    }
}

impl ValidateChecksums for ScheduleSignTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.schedule_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for ScheduleSignTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let schedule_id = self.schedule_id.to_protobuf();

        services::transaction_body::Data::ScheduleSign(services::ScheduleSignTransactionBody {
            schedule_id,
        })
    }
}

impl From<ScheduleSignTransactionData> for AnyTransactionData {
    fn from(transaction: ScheduleSignTransactionData) -> Self {
        Self::ScheduleSign(transaction)
    }
}

impl FromProtobuf<services::ScheduleSignTransactionBody> for ScheduleSignTransactionData {
    fn from_protobuf(pb: services::ScheduleSignTransactionBody) -> crate::Result<Self> {
        Ok(Self { schedule_id: Option::from_protobuf(pb.schedule_id)? })
    }
}
