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
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

use crate::ledger_id::RefLedgerId;
use crate::query::{AnyQueryData, QueryExecute, ToQueryProtobuf};
use crate::{
    AccountId, AllProxyStakers, BoxGrpcFuture, Error, Query, ToProtobuf, ValidateChecksums,
};

/// Get all the accounts that are proxy staking to this account.
/// For each of them, give the amount currently staked.
pub type AccountStakersQuery = Query<AccountStakersQueryData>;

#[derive(Debug, Clone, Default)]
pub struct AccountStakersQueryData {
    account_id: Option<AccountId>,
}

impl From<AccountStakersQueryData> for AnyQueryData {
    #[inline]
    fn from(data: AccountStakersQueryData) -> Self {
        Self::AccountStakers(data)
    }
}

impl AccountStakersQuery {
    /// Gets the account ID for which the records should be retrieved.
    #[must_use]
    pub fn get_account_id(&self) -> Option<AccountId> {
        self.data.account_id
    }

    /// Sets the account ID for which the records should be retrieved.
    pub fn account_id(&mut self, id: AccountId) -> &mut Self {
        self.data.account_id = Some(id);
        self
    }
}

impl ToQueryProtobuf for AccountStakersQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let account_id = self.account_id.to_protobuf();

        services::Query {
            query: Some(services::query::Query::CryptoGetProxyStakers(
                services::CryptoGetStakersQuery { account_id, header: Some(header) },
            )),
        }
    }
}

impl QueryExecute for AccountStakersQueryData {
    type Response = AllProxyStakers;

    fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> BoxGrpcFuture<'_, services::Response> {
        Box::pin(async {
            CryptoServiceClient::new(channel).get_stakers_by_account_id(request).await
        })
    }
}

impl ValidateChecksums for AccountStakersQueryData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        self.account_id.validate_checksums(ledger_id)
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::query::ToQueryProtobuf;
    use crate::{AccountStakersQuery, Hbar};

    #[test]
    fn serialize() {
        expect![[r#"
            Query {
                query: Some(
                    CryptoGetProxyStakers(
                        CryptoGetStakersQuery {
                            header: Some(
                                QueryHeader {
                                    payment: None,
                                    response_type: AnswerOnly,
                                },
                            ),
                            account_id: Some(
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
                        },
                    ),
                ),
            }
        "#]].assert_debug_eq(
            &AccountStakersQuery::new()
                .account_id(crate::AccountId {
                    shard: 0,
                    realm: 0,
                    num: 5005,
                    alias: None,
                    evm_address: None,
                    checksum: None,
                })
                .max_payment_amount(Hbar::from_tinybars(100_000))
                .data
                .to_query_protobuf(Default::default()),
        );
    }
}
