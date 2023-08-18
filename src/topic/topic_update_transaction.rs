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
use hedera_proto::services::consensus_service_client::ConsensusServiceClient;
use time::{
    Duration,
    OffsetDateTime,
};
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
    AccountId,
    BoxGrpcFuture,
    Error,
    Key,
    TopicId,
    Transaction,
    ValidateChecksums,
};

/// Change properties for the given topic.
///
/// Any null field is ignored (left unchanged).
///
pub type TopicUpdateTransaction = Transaction<TopicUpdateTransactionData>;

#[derive(Debug, Clone, Default)]
pub struct TopicUpdateTransactionData {
    /// The topic ID which is being updated in this transaction.
    topic_id: Option<TopicId>,

    /// The new expiration time to extend to (ignored if equal to or before the current one).
    expiration_time: Option<OffsetDateTime>,

    /// Short publicly visible memo about the topic. No guarantee of uniqueness.
    topic_memo: Option<String>,

    /// Access control for `TopicUpdateTransaction` and `TopicDeleteTransaction`.
    admin_key: Option<Key>,

    /// Access control for `TopicMessageSubmitTransaction`.
    submit_key: Option<Key>,

    /// The initial lifetime of the topic and the amount of time to attempt to
    /// extend the topic's lifetime by automatically at the topic's expiration time, if
    /// the `auto_renew_account_id` is configured.
    auto_renew_period: Option<Duration>,

    /// Optional account to be used at the topic's expiration time to extend the life of the topic.
    auto_renew_account_id: Option<AccountId>,
}

impl TopicUpdateTransaction {
    /// Returns the topic ID which is being updated.
    #[must_use]
    pub fn get_topic_id(&self) -> Option<TopicId> {
        self.data().topic_id
    }

    /// Sets the topic ID which is being updated.
    pub fn topic_id(&mut self, id: impl Into<TopicId>) -> &mut Self {
        self.data_mut().topic_id = Some(id.into());
        self
    }

    /// Returns the new expiration time to extend to (ignored if equal to or before the current one).
    #[must_use]
    pub fn get_expiration_time(&self) -> Option<OffsetDateTime> {
        self.data().expiration_time
    }

    /// Sets the new expiration time to extend to (ignored if equal to or before the current one).
    pub fn expiration_time(&mut self, at: OffsetDateTime) -> &mut Self {
        self.data_mut().expiration_time = Some(at);
        self
    }

    /// Returns the new topic memo for the topic.
    #[must_use]
    pub fn get_topic_memo(&self) -> Option<&str> {
        self.data().topic_memo.as_deref()
    }

    /// Sets the short publicly visible memo about the topic.
    ///
    /// No guarantee of uniqueness.
    pub fn topic_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.data_mut().topic_memo = Some(memo.into());
        self
    }

    /// Returns the access control for [`TopicUpdateTransaction`] and [`TopicDeleteTransaction`](crate::TopicDeleteTransaction).
    #[must_use]
    pub fn get_admin_key(&self) -> Option<&Key> {
        self.data().admin_key.as_ref()
    }

    /// Sets the access control for [`TopicUpdateTransaction`] and [`TopicDeleteTransaction`](crate::TopicDeleteTransaction).
    pub fn admin_key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.data_mut().admin_key = Some(key.into());
        self
    }

    /// Clears the access control for [`TopicUpdateTransaction`] and [`TopicDeleteTransaction`](crate::TopicDeleteTransaction).
    pub fn clear_admin_key(&mut self) -> &mut Self {
        self.data_mut().admin_key = Some(Key::KeyList(crate::KeyList::new()));
        self
    }

    /// Returns the access control for [`TopicMessageSubmitTransaction`](crate::TopicMessageSubmitTransaction).
    #[must_use]
    pub fn get_submit_key(&self) -> Option<&Key> {
        self.data().submit_key.as_ref()
    }

    /// Sets the access control for [`TopicMessageSubmitTransaction`](crate::TopicMessageSubmitTransaction).
    pub fn submit_key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.data_mut().submit_key = Some(key.into());
        self
    }
    /// Clears the access control for [`TopicUpdateTransaction`] and [`TopicDeleteTransaction`](crate::TopicDeleteTransaction).
    pub fn clear_submit_key(&mut self) -> &mut Self {
        self.data_mut().submit_key = Some(Key::KeyList(crate::KeyList::new()));
        self
    }

    /// Returns the initial lifetime of the topic and the amount of time to attempt to
    /// extend the topic's lifetime by automatically at the topic's expiration time.
    #[must_use]
    pub fn get_auto_renew_period(&self) -> Option<Duration> {
        self.data().auto_renew_period
    }

    /// Sets the initial lifetime of the topic and the amount of time to attempt to
    /// extend the topic's lifetime by automatically at the topic's expiration time.
    pub fn auto_renew_period(&mut self, period: Duration) -> &mut Self {
        self.data_mut().auto_renew_period = Some(period);
        self
    }

    /// Returns the account to be used at the topic's expiration time to extend the life of the topic.
    #[must_use]
    pub fn get_auto_renew_account_id(&self) -> Option<AccountId> {
        self.data().auto_renew_account_id
    }

    /// Sets the account to be used at the topic's expiration time to extend the life of the topic.
    pub fn auto_renew_account_id(&mut self, id: AccountId) -> &mut Self {
        self.data_mut().auto_renew_account_id = Some(id);
        self
    }

    /// Clear the auto renew account ID for this topic.
    pub fn clear_auto_renew_account_id(&mut self) -> &mut Self {
        self.auto_renew_account_id(AccountId {
            shard: 0,
            realm: 0,
            num: 0,
            alias: None,
            evm_address: None,
            checksum: None,
        })
    }
}

impl TransactionData for TopicUpdateTransactionData {}

impl TransactionExecute for TopicUpdateTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { ConsensusServiceClient::new(channel).update_topic(request).await })
    }
}

impl ValidateChecksums for TopicUpdateTransactionData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        self.topic_id.validate_checksums(ledger_id)?;
        self.auto_renew_account_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for TopicUpdateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::ConsensusUpdateTopic(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for TopicUpdateTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::ConsensusUpdateTopic(self.to_protobuf())
    }
}

impl From<TopicUpdateTransactionData> for AnyTransactionData {
    fn from(transaction: TopicUpdateTransactionData) -> Self {
        Self::TopicUpdate(transaction)
    }
}

impl FromProtobuf<services::ConsensusUpdateTopicTransactionBody> for TopicUpdateTransactionData {
    fn from_protobuf(pb: services::ConsensusUpdateTopicTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            topic_id: Option::from_protobuf(pb.topic_id)?,
            expiration_time: pb.expiration_time.map(Into::into),
            topic_memo: pb.memo,
            admin_key: Option::from_protobuf(pb.admin_key)?,
            submit_key: Option::from_protobuf(pb.submit_key)?,
            auto_renew_period: pb.auto_renew_period.map(Into::into),
            auto_renew_account_id: Option::from_protobuf(pb.auto_renew_account)?,
        })
    }
}

impl ToProtobuf for TopicUpdateTransactionData {
    type Protobuf = services::ConsensusUpdateTopicTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        let topic_id = self.topic_id.to_protobuf();
        let expiration_time = self.expiration_time.map(Into::into);
        let admin_key = self.admin_key.to_protobuf();
        let submit_key = self.submit_key.to_protobuf();
        let auto_renew_period = self.auto_renew_period.map(Into::into);
        let auto_renew_account_id = self.auto_renew_account_id.to_protobuf();

        services::ConsensusUpdateTopicTransactionBody {
            auto_renew_account: auto_renew_account_id,
            memo: self.topic_memo.clone(),
            expiration_time,
            topic_id,
            admin_key,
            submit_key,
            auto_renew_period,
        }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use time::Duration;

    use crate::transaction::test_helpers::{
        transaction_body,
        unused_private_key,
        VALID_START,
    };
    use crate::{
        AnyTransaction,
        Hbar,
        TopicId,
        TopicUpdateTransaction,
        TransactionId,
    };

    fn make_transaction() -> TopicUpdateTransaction {
        let mut tx = TopicUpdateTransaction::new();

        tx.node_account_ids(["0.0.5005".parse().unwrap(), "0.0.5006".parse().unwrap()])
            .transaction_id(TransactionId {
                account_id: "5006".parse().unwrap(),
                valid_start: VALID_START,
                nonce: None,
                scheduled: false,
            })
            .topic_id("0.0.5007".parse::<TopicId>().unwrap())
            .clear_admin_key()
            .clear_auto_renew_account_id()
            .clear_submit_key()
            .topic_memo("")
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
                    ConsensusUpdateTopic(
                        ConsensusUpdateTopicTransactionBody {
                            topic_id: Some(
                                TopicId {
                                    shard_num: 0,
                                    realm_num: 0,
                                    topic_num: 5007,
                                },
                            ),
                            memo: Some(
                                "",
                            ),
                            expiration_time: None,
                            admin_key: Some(
                                Key {
                                    key: Some(
                                        KeyList(
                                            KeyList {
                                                keys: [],
                                            },
                                        ),
                                    ),
                                },
                            ),
                            submit_key: Some(
                                Key {
                                    key: Some(
                                        KeyList(
                                            KeyList {
                                                keys: [],
                                            },
                                        ),
                                    ),
                                },
                            ),
                            auto_renew_period: None,
                            auto_renew_account: Some(
                                AccountId {
                                    shard_num: 0,
                                    realm_num: 0,
                                    account: Some(
                                        AccountNum(
                                            0,
                                        ),
                                    ),
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

    fn make_transaction2() -> TopicUpdateTransaction {
        let mut tx = TopicUpdateTransaction::new();

        tx.node_account_ids(["0.0.5005".parse().unwrap(), "0.0.5006".parse().unwrap()])
            .transaction_id(TransactionId {
                account_id: "5006".parse().unwrap(),
                valid_start: VALID_START,
                nonce: None,
                scheduled: false,
            })
            .topic_id("0.0.5007".parse::<TopicId>().unwrap())
            .admin_key(unused_private_key().public_key())
            .auto_renew_account_id("0.0.5009".parse().unwrap())
            .auto_renew_period(Duration::days(1))
            .submit_key(unused_private_key().public_key())
            .topic_memo("Hello memo")
            .expiration_time(VALID_START)
            .max_transaction_fee(Hbar::from_tinybars(100_000))
            .freeze()
            .unwrap()
            .sign(unused_private_key());

        tx
    }

    #[test]
    fn serialize2() {
        let tx = make_transaction2();

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
                    ConsensusUpdateTopic(
                        ConsensusUpdateTopicTransactionBody {
                            topic_id: Some(
                                TopicId {
                                    shard_num: 0,
                                    realm_num: 0,
                                    topic_num: 5007,
                                },
                            ),
                            memo: Some(
                                "Hello memo",
                            ),
                            expiration_time: Some(
                                Timestamp {
                                    seconds: 1554158542,
                                    nanos: 0,
                                },
                            ),
                            admin_key: Some(
                                Key {
                                    key: Some(
                                        Ed25519(
                                            [
                                                224,
                                                200,
                                                236,
                                                39,
                                                88,
                                                165,
                                                135,
                                                159,
                                                250,
                                                194,
                                                38,
                                                161,
                                                60,
                                                12,
                                                81,
                                                107,
                                                121,
                                                158,
                                                114,
                                                227,
                                                81,
                                                65,
                                                160,
                                                221,
                                                130,
                                                143,
                                                148,
                                                211,
                                                121,
                                                136,
                                                164,
                                                183,
                                            ],
                                        ),
                                    ),
                                },
                            ),
                            submit_key: Some(
                                Key {
                                    key: Some(
                                        Ed25519(
                                            [
                                                224,
                                                200,
                                                236,
                                                39,
                                                88,
                                                165,
                                                135,
                                                159,
                                                250,
                                                194,
                                                38,
                                                161,
                                                60,
                                                12,
                                                81,
                                                107,
                                                121,
                                                158,
                                                114,
                                                227,
                                                81,
                                                65,
                                                160,
                                                221,
                                                130,
                                                143,
                                                148,
                                                211,
                                                121,
                                                136,
                                                164,
                                                183,
                                            ],
                                        ),
                                    ),
                                },
                            ),
                            auto_renew_period: Some(
                                Duration {
                                    seconds: 86400,
                                },
                            ),
                            auto_renew_account: Some(
                                AccountId {
                                    shard_num: 0,
                                    realm_num: 0,
                                    account: Some(
                                        AccountNum(
                                            5009,
                                        ),
                                    ),
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
    fn to_from_bytes2() {
        let tx = make_transaction2();

        let tx2 = AnyTransaction::from_bytes(&tx.to_bytes().unwrap()).unwrap();

        let tx = transaction_body(tx);

        let tx2 = transaction_body(tx2);

        assert_eq!(tx, tx2);
    }
}
