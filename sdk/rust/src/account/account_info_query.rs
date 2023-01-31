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

use crate::account::AccountInfo;
use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    AccountId,
    Error,
    LedgerId,
    Query,
    ToProtobuf,
    ValidateChecksums,
};

/// Get all the information about an account, including the balance.
///
/// This does not get the list of account records.
///
pub type AccountInfoQuery = Query<AccountInfoQueryData>;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct AccountInfoQueryData {
    account_id: Option<AccountId>,
}

impl From<AccountInfoQueryData> for AnyQueryData {
    #[inline]
    fn from(data: AccountInfoQueryData) -> Self {
        Self::AccountInfo(data)
    }
}

impl AccountInfoQuery {
    /// Gets the account ID for which information is requested.
    #[must_use]
    pub fn get_account_id(&self) -> Option<AccountId> {
        self.data.account_id
    }

    /// Sets the account ID for which information is requested.
    pub fn account_id(&mut self, id: AccountId) -> &mut Self {
        self.data.account_id = Some(id);
        self
    }
}

impl ToQueryProtobuf for AccountInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let account_id = self.account_id.to_protobuf();

        services::Query {
            query: Some(services::query::Query::CryptoGetInfo(services::CryptoGetInfoQuery {
                account_id,
                header: Some(header),
            })),
        }
    }
}

#[async_trait]
impl QueryExecute for AccountInfoQueryData {
    type Response = AccountInfo;

    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.account_id.validate_checksums_for_ledger_id(ledger_id)
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        CryptoServiceClient::new(channel).get_account_info(request).await
    }
}

// hack(sr): these tests currently don't compile due to `payer_account_id`
#[cfg(feature = "false")]
mod tests {
    use assert_matches::assert_matches;

    use crate::query::AnyQueryData;
    use crate::{
        AccountId,
        AccountInfoQuery,
        AnyQuery,
    };

    // language=JSON
    const ACCOUNT_INFO: &str = r#"{
  "$type": "accountInfo",
  "accountId": "0.0.1001",
  "payment": {
    "amount": 50,
    "transactionMemo": "query payment",
    "payerAccountId": "0.0.6189"
  }
}"#;

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut query = AccountInfoQuery::new();
        query
            .account_id(AccountId::from(1001))
            .payer_account_id(AccountId::from(6189))
            .payment_amount(50)
            .payment_transaction_memo("query payment");

        let s = serde_json::to_string_pretty(&query)?;
        assert_eq!(s, ACCOUNT_INFO);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let query: AnyQuery = serde_json::from_str(ACCOUNT_INFO)?;

        let data = assert_matches!(query.data, AnyQueryData::AccountInfo(query) => query);
        let account_id = assert_matches!(data.account_id, Some(id) => id);

        assert_eq!(account_id.num, 1001);
        assert_eq!(query.payment.body.data.amount, Some(50));
        assert_eq!(query.payment.body.transaction_memo, "query payment");
        assert_eq!(query.payment.body.payer_account_id, Some(AccountId::from(6189)));

        Ok(())
    }
}
