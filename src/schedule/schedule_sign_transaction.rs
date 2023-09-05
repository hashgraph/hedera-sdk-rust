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
    ChunkInfo,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::{
    BoxGrpcFuture,
    Error,
    ScheduleId,
    Transaction,
    ValidateChecksums,
};

/// Adds zero or more signing keys to a schedule.
pub type ScheduleSignTransaction = Transaction<ScheduleSignTransactionData>;

#[derive(Debug, Default, Clone)]
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

impl TransactionData for ScheduleSignTransactionData {}

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
    fn validate_checksums(&self, ledger_id: &crate::ledger_id::RefLedgerId) -> Result<(), Error> {
        self.schedule_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for ScheduleSignTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

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

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::transaction::test_helpers::{
        check_body,
        transaction_body,
    };
    use crate::{
        AnyTransaction,
        ScheduleSignTransaction,
    };

    fn make_transaction() -> ScheduleSignTransaction {
        let mut tx = ScheduleSignTransaction::new_for_tests();

        tx.schedule_id("0.0.444".parse().unwrap()).freeze().unwrap();

        tx
    }

    #[test]
    fn serialize() {
        let tx = make_transaction();

        let tx = transaction_body(tx);

        let tx = check_body(tx);

        expect![[r#"
            ScheduleSign(
                ScheduleSignTransactionBody {
                    schedule_id: Some(
                        ScheduleId {
                            shard_num: 0,
                            realm_num: 0,
                            schedule_num: 444,
                        },
                    ),
                },
            )
        "#]]
        .assert_debug_eq(&tx)
    }

    #[test]
    fn to_from_bytes() {
        let tx = make_transaction();

        let tx2 = AnyTransaction::from_bytes(&tx.to_bytes().unwrap()).unwrap();

        let tx = transaction_body(tx);

        let tx2 = transaction_body(tx2);

        assert_eq!(tx, tx2);
    }
}
