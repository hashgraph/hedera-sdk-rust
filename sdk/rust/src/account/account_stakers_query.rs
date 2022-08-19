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

use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    AccountId,
    AllProxyStakers,
    Query,
    ToProtobuf,
};

/// Get all the accounts that are proxy staking to this account.
/// For each of them, give the amount currently staked.
pub type AccountStakersQuery = Query<AccountStakersQueryData>;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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
    /// Sets the account ID for which the records should be retrieved.
    pub fn account_id(&mut self, id: AccountId) -> &mut Self {
        self.data.account_id = Some(id.into());
        self
    }
}

impl ToQueryProtobuf for AccountStakersQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let account_id = self.account_id.as_ref().map(|id| id.to_protobuf());

        services::Query {
            query: Some(services::query::Query::CryptoGetProxyStakers(
                services::CryptoGetStakersQuery { account_id, header: Some(header) },
            )),
        }
    }
}

#[async_trait]
impl QueryExecute for AccountStakersQueryData {
    type Response = AllProxyStakers;

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        CryptoServiceClient::new(channel).get_stakers_by_account_id(request).await
    }
}
