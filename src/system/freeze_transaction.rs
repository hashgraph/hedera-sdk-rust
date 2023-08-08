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
use hedera_proto::services::freeze_service_client::FreezeServiceClient;
use time::OffsetDateTime;
use tonic::transport::Channel;

use crate::protobuf::FromProtobuf;
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
    FileId,
    FreezeType,
    ToProtobuf,
    Transaction,
    ValidateChecksums,
};

/// Sets the freezing period in which the platform will stop creating
/// events and accepting transactions.
///
/// This is used before safely shut down the platform for maintenance.
///
pub type FreezeTransaction = Transaction<FreezeTransactionData>;

#[derive(Debug, Clone, Default)]
pub struct FreezeTransactionData {
    start_time: Option<OffsetDateTime>,
    file_id: Option<FileId>,
    file_hash: Option<Vec<u8>>,
    freeze_type: FreezeType,
}

impl FreezeTransaction {
    /// Returns the start time.
    #[must_use]
    pub fn get_start_time(&self) -> Option<OffsetDateTime> {
        self.data().start_time
    }

    /// Sets the start time.
    pub fn start_time(&mut self, time: OffsetDateTime) -> &mut Self {
        self.data_mut().start_time = Some(time);
        self
    }

    /// Returns the freeze type.
    #[must_use]
    pub fn get_freeze_type(&self) -> FreezeType {
        self.data().freeze_type
    }

    /// Sets the freeze type.
    pub fn freeze_type(&mut self, ty: FreezeType) -> &mut Self {
        self.data_mut().freeze_type = ty;
        self
    }

    /// Returns the file ID.
    #[must_use]
    pub fn get_file_id(&self) -> Option<FileId> {
        self.data().file_id
    }

    /// Sets the file ID.
    pub fn file_id(&mut self, id: FileId) -> &mut Self {
        self.data_mut().file_id = Some(id);
        self
    }

    /// Returns the file hash.
    #[must_use]
    pub fn get_file_hash(&self) -> Option<&[u8]> {
        self.data().file_hash.as_deref()
    }

    /// Sets the file hash.
    pub fn file_hash(&mut self, hash: Vec<u8>) -> &mut Self {
        self.data_mut().file_hash = Some(hash);
        self
    }
}

impl TransactionData for FreezeTransactionData {}

impl TransactionExecute for FreezeTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { FreezeServiceClient::new(channel).freeze(request).await })
    }
}

impl ValidateChecksums for FreezeTransactionData {
    fn validate_checksums(&self, ledger_id: &crate::ledger_id::RefLedgerId) -> Result<(), Error> {
        self.file_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for FreezeTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::Freeze(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for FreezeTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::Freeze(self.to_protobuf())
    }
}

impl From<FreezeTransactionData> for AnyTransactionData {
    fn from(transaction: FreezeTransactionData) -> Self {
        Self::Freeze(transaction)
    }
}

impl FromProtobuf<services::FreezeTransactionBody> for FreezeTransactionData {
    fn from_protobuf(pb: services::FreezeTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            start_time: pb.start_time.map(Into::into),
            file_id: Option::from_protobuf(pb.update_file)?,
            file_hash: Some(pb.file_hash),
            freeze_type: FreezeType::from(pb.freeze_type),
        })
    }
}

impl ToProtobuf for FreezeTransactionData {
    type Protobuf = services::FreezeTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::FreezeTransactionBody {
            update_file: self.file_id.to_protobuf(),
            file_hash: self.file_hash.clone().unwrap_or_default(),
            start_time: self.start_time.map(Into::into),
            freeze_type: self.freeze_type as _,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use hex_literal::hex;

    use crate::transaction::test_helpers::{
        transaction_body,
        unused_private_key,
        VALID_START,
    };
    use crate::{
        AnyTransaction,
        FreezeTransaction,
        FreezeType,
        Hbar,
        TransactionId,
    };

    fn make_transaction() -> FreezeTransaction {
        let mut tx = FreezeTransaction::new();

        tx.node_account_ids(["0.0.5005".parse().unwrap(), "0.0.5006".parse().unwrap()])
            .transaction_id(TransactionId {
                account_id: "5006".parse().unwrap(),
                valid_start: VALID_START,
                nonce: None,
                scheduled: false,
            })
            .file_id(("4.5.6").parse().unwrap())
            .file_hash(hex!("1723904587120938954702349857").to_vec())
            .start_time(VALID_START)
            .freeze_type(FreezeType::FreezeAbort)
            .max_transaction_fee(Hbar::from_tinybars(100_000))
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
                transaction_fee: 100000,
                transaction_valid_duration: Some(
                    Duration {
                        seconds: 120,
                    },
                ),
                generate_record: false,
                memo: "",
                data: Some(
                    Freeze(
                        FreezeTransactionBody {
                            start_hour: 0,
                            start_min: 0,
                            end_hour: 0,
                            end_min: 0,
                            update_file: Some(
                                FileId {
                                    shard_num: 4,
                                    realm_num: 5,
                                    file_num: 6,
                                },
                            ),
                            file_hash: [
                                23,
                                35,
                                144,
                                69,
                                135,
                                18,
                                9,
                                56,
                                149,
                                71,
                                2,
                                52,
                                152,
                                87,
                            ],
                            start_time: Some(
                                Timestamp {
                                    seconds: 1554158542,
                                    nanos: 0,
                                },
                            ),
                            freeze_type: FreezeAbort,
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
