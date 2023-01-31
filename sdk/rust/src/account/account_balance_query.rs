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
use services::crypto_get_account_balance_query::BalanceSource;
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    Query,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    AccountBalance,
    AccountId,
    BoxGrpcFuture,
    ContractId,
    Error,
    LedgerId,
    ToProtobuf,
    ValidateChecksums,
};

/// Get the balance of a cryptocurrency account.
///
/// This returns only the balance, so it is a smaller reply
/// than [`AccountInfoQuery`][crate::AccountInfoQuery],
/// which returns the balance plus additional information.
pub type AccountBalanceQuery = Query<AccountBalanceQueryData>;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
pub struct AccountBalanceQueryData {
    #[cfg_attr(feature = "ffi", serde(flatten))]
    source: AccountBalanceSource,
}

impl Default for AccountBalanceQueryData {
    fn default() -> Self {
        Self { source: AccountBalanceSource::AccountId(AccountId::from(0)) }
    }
}

impl From<AccountBalanceQueryData> for AnyQueryData {
    #[inline]
    fn from(data: AccountBalanceQueryData) -> Self {
        Self::AccountBalance(data)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
enum AccountBalanceSource {
    AccountId(AccountId),
    ContractId(ContractId),
}

impl AccountBalanceQuery {
    /// Get the account ID for which information is requested.
    #[must_use]
    pub fn get_account_id(&self) -> Option<AccountId> {
        match self.data.source {
            AccountBalanceSource::AccountId(it) => Some(it),
            AccountBalanceSource::ContractId(_) => None,
        }
    }

    /// Sets the account ID for which information is requested.
    ///
    /// This is mutually exclusive with [`contract_id`](Self::contract_id).
    pub fn account_id(&mut self, id: AccountId) -> &mut Self {
        self.data.source = AccountBalanceSource::AccountId(id);
        self
    }

    /// Get the contract ID for which information is requested.
    #[must_use]
    pub fn get_contract_id(&self) -> Option<ContractId> {
        match self.data.source {
            AccountBalanceSource::ContractId(it) => Some(it),
            AccountBalanceSource::AccountId(_) => None,
        }
    }

    /// Sets the contract ID for which information is requested.
    ///
    /// This is mutually exclusive with [`account_id`](Self::account_id).
    pub fn contract_id(&mut self, id: ContractId) -> &mut Self {
        self.data.source = AccountBalanceSource::ContractId(id);
        self
    }
}

impl ToQueryProtobuf for AccountBalanceQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let source = Some(&self.source).as_ref().map(|source| match source {
            AccountBalanceSource::AccountId(id) => BalanceSource::AccountId(id.to_protobuf()),
            AccountBalanceSource::ContractId(id) => BalanceSource::ContractId(id.to_protobuf()),
        });

        services::Query {
            query: Some(services::query::Query::CryptogetAccountBalance(
                services::CryptoGetAccountBalanceQuery {
                    balance_source: source,
                    header: Some(header),
                },
            )),
        }
    }
}

impl QueryExecute for AccountBalanceQueryData {
    type Response = AccountBalance;

    fn is_payment_required(&self) -> bool {
        false
    }

    fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> BoxGrpcFuture<'_, services::Response> {
        Box::pin(async { CryptoServiceClient::new(channel).crypto_get_balance(request).await })
    }
}

impl ValidateChecksums for AccountBalanceQueryData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        match self.source {
            AccountBalanceSource::AccountId(account_id) => account_id.validate_checksums(ledger_id),
            AccountBalanceSource::ContractId(contract_id) => {
                contract_id.validate_checksums(ledger_id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "ffi")]
    mod ffi {
        use assert_matches::assert_matches;

        use crate::account::account_balance_query::AccountBalanceSource;
        use crate::query::AnyQueryData;
        use crate::{
            AccountBalanceQuery,
            AccountId,
            AnyQuery,
        };

        // language=JSON
        const ACCOUNT_BALANCE: &str = r#"{
  "$type": "accountBalance",
  "accountId": "0.0.1001"
}"#;

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut query = AccountBalanceQuery::new();
            query.account_id(AccountId::from(1001));

            let s = serde_json::to_string_pretty(&query)?;
            assert_eq!(s, ACCOUNT_BALANCE);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let query: AnyQuery = serde_json::from_str(ACCOUNT_BALANCE)?;

            let data = assert_matches!(query.data, AnyQueryData::AccountBalance(query) => query);
            let source = assert_matches!(data.source, AccountBalanceSource::AccountId(id) => id);

            assert_eq!(source.num, 1001);

            Ok(())
        }
    }
}
