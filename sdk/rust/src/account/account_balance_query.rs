use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use services::crypto_get_account_balance_query::BalanceSource;
use tonic::transport::Channel;

use crate::query::{AnyQueryData, Query, QueryExecute, ToQueryProtobuf};
use crate::{AccountAddress, AccountBalanceResponse, AccountId, ContractAddress, ToProtobuf};

/// Get the balance of a cryptocurrency account.
///
/// This returns only the balance, so it is a smaller reply
/// than [`AccountInfoQuery`][crate::AccountInfoQuery], which returns the balance plus
/// additional information.
///
pub type AccountBalanceQuery = Query<AccountBalanceQueryData>;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct AccountBalanceQueryData {
    #[serde(flatten)]
    source: AccountBalanceSource,
}

impl Default for AccountBalanceQueryData {
    fn default() -> Self {
        Self { source: AccountBalanceSource::AccountId(AccountId::from(0).into()) }
    }
}

impl From<AccountBalanceQueryData> for AnyQueryData {
    #[inline]
    fn from(data: AccountBalanceQueryData) -> Self {
        Self::AccountBalance(data)
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum AccountBalanceSource {
    AccountId(AccountAddress),
    ContractId(ContractAddress),
}

impl AccountBalanceQuery {
    /// Sets the account ID for which information is requested.
    ///
    /// This is mutually exclusive with [`contract_id`](#method.contract_id).
    ///
    pub fn account_id(&mut self, id: impl Into<AccountAddress>) -> &mut Self {
        self.data.source = AccountBalanceSource::AccountId(id.into());
        self
    }

    /// Sets the contract ID for which information is requested.
    ///
    /// This is mutually exclusive with [`account_id`](#method.account_id).
    ///
    pub fn contract_id(&mut self, id: ContractAddress) -> &mut Self {
        self.data.source = AccountBalanceSource::ContractId(id.into());
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

#[async_trait]
impl QueryExecute for AccountBalanceQueryData {
    type Response = AccountBalanceResponse;

    fn is_payment_required(&self) -> bool {
        false
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        CryptoServiceClient::new(channel).crypto_get_balance(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_matches::assert_matches;

    use crate::account::account_balance_query::AccountBalanceSource;
    use crate::query::AnyQueryData;
    use crate::{AccountAddress, AccountBalanceQuery, AccountId, AnyQuery};
    use crate::mock::{AnyMockResponseInput, Mocker};

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
        let source = assert_matches!(source, AccountAddress::AccountId(id) => id);

        assert_eq!(source.num, 1001);

        Ok(())
    }

    #[tokio::test]
    async fn mock_query_balance() -> anyhow::Result<()> {
        let responses = vec![
            services::Response {
                response: Some(services::response::Response::CryptogetAccountBalance(services::CryptoGetAccountBalanceResponse {
                    header: Some(services::ResponseHeader::default()),
                    account_id: Some(services::AccountId {
                       account: Some(services::account_id::Account::AccountNum(3)),
                        ..Default::default()
                    }),
                    balance: 10,
                    ..Default::default()
                })),
            }.into()
        ];

        let mocker = Mocker::new(responses).await?;

        let response = AccountBalanceQuery::new()
            .account_id(AccountId::from(3))
            .execute(&mocker.client)
            .await?;

        assert_eq!(response.account_id, AccountId::from(3));
        assert_eq!(response.balance, 10);

        Ok(())
    }
}
