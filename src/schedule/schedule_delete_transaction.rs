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

use crate::ledger_id::RefLedgerId;
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
    BoxGrpcFuture,
    Error,
    ScheduleId,
    Transaction,
    ValidateChecksums,
};

/// Marks a schedule in the network's action queue as deleted. Must be signed
/// by the admin key of the target schedule. A deleted schedule cannot
/// receive any additional signing keys, nor will it be executed.
pub type ScheduleDeleteTransaction = Transaction<ScheduleDeleteTransactionData>;

#[derive(Debug, Default, Clone)]
pub struct ScheduleDeleteTransactionData {
    schedule_id: Option<ScheduleId>,
}

impl ScheduleDeleteTransaction {
    /// Returns the schedule to delete.
    #[must_use]
    pub fn get_schedule_id(&self) -> Option<ScheduleId> {
        self.data().schedule_id
    }

    /// Sets the schedule to delete.
    pub fn schedule_id(&mut self, id: ScheduleId) -> &mut Self {
        self.data_mut().schedule_id = Some(id);
        self
    }
}
impl TransactionData for ScheduleDeleteTransactionData {}

impl TransactionExecute for ScheduleDeleteTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { ScheduleServiceClient::new(channel).delete_schedule(request).await })
    }
}

impl ValidateChecksums for ScheduleDeleteTransactionData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        self.schedule_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for ScheduleDeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::ScheduleDelete(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for ScheduleDeleteTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::ScheduleDelete(self.to_protobuf())
    }
}

impl From<ScheduleDeleteTransactionData> for AnyTransactionData {
    fn from(transaction: ScheduleDeleteTransactionData) -> Self {
        Self::ScheduleDelete(transaction)
    }
}

impl FromProtobuf<services::ScheduleDeleteTransactionBody> for ScheduleDeleteTransactionData {
    fn from_protobuf(pb: services::ScheduleDeleteTransactionBody) -> crate::Result<Self> {
        Ok(Self { schedule_id: Option::from_protobuf(pb.schedule_id)? })
    }
}

impl ToProtobuf for ScheduleDeleteTransactionData {
    type Protobuf = services::ScheduleDeleteTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::ScheduleDeleteTransactionBody { schedule_id: self.schedule_id.to_protobuf() }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::transaction::test_helpers::{
        transaction_body,
        unused_private_key,
        TEST_NODE_ACCOUNT_IDS,
        TEST_TX_ID,
    };
    use crate::{
        AnyTransaction,
        Hbar,
        ScheduleDeleteTransaction,
    };

    fn make_transaction() -> ScheduleDeleteTransaction {
        let mut tx = ScheduleDeleteTransaction::new();

        tx.node_account_ids(TEST_NODE_ACCOUNT_IDS)
            .transaction_id(TEST_TX_ID)
            .schedule_id("0.0.444".parse().unwrap())
            .max_transaction_fee(Hbar::new(1))
            .freeze()
            .unwrap()
            .sign(unused_private_key());

        tx
    }

    #[test]
    fn serialize() {
        let tx = make_transaction();

        let tx = transaction_body(tx);

        expect![[r#"
            TransactionBody {
                transaction_id: Some(
                    TransactionId {
                        transaction_valid_start: Some(
                            Timestamp {
                                seconds: 1554158542,
                                nanos: 0,
                            },
                        ),
                        account_id: Some(
                            AccountId {
                                shard_num: 0,
                                realm_num: 0,
                                account: Some(
                                    AccountNum(
                                        5006,
                                    ),
                                ),
                            },
                        ),
                        scheduled: false,
                        nonce: 0,
                    },
                ),
                node_account_id: Some(
                    AccountId {
                        shard_num: 0,
                        realm_num: 0,
                        account: Some(
                            AccountNum(
                                5005,
                            ),
                        ),
                    },
                ),
                transaction_fee: 100000000,
                transaction_valid_duration: Some(
                    Duration {
                        seconds: 120,
                    },
                ),
                generate_record: false,
                memo: "",
                data: Some(
                    ScheduleDelete(
                        ScheduleDeleteTransactionBody {
                            schedule_id: Some(
                                ScheduleId {
                                    shard_num: 0,
                                    realm_num: 0,
                                    schedule_num: 444,
                                },
                            ),
                        },
                    ),
                ),
            }
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
