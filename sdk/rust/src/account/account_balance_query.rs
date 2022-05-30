use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use services::crypto_get_account_balance_query::BalanceSource;
use tonic::transport::Channel;

use crate::query::{AnyQueryData, Query, QueryExecute, ToQueryProtobuf};
use crate::{
    AccountBalanceResponse, AccountId, AccountIdOrAlias, ContractIdOrEvmAddress, ToProtobuf
};

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
    AccountId(AccountIdOrAlias),
    ContractId(ContractIdOrEvmAddress),
}

impl AccountBalanceQuery {
    /// Sets the account ID for which information is requested.
    ///
    /// This is mutually exclusive with [`contract_id`](#method.contract_id).
    ///
    pub fn account_id(&mut self, id: impl Into<AccountIdOrAlias>) -> &mut Self {
        self.data.source = AccountBalanceSource::AccountId(id.into());
        self
    }

    /// Sets the contract ID for which information is requested.
    ///
    /// This is mutually exclusive with [`account_id`](#method.account_id).
    ///
    pub fn contract_id(&mut self, id: ContractIdOrEvmAddress) -> &mut Self {
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
    use assert_matches::assert_matches;

    use crate::account::account_balance_query::AccountBalanceSource;
    use crate::query::AnyQueryData;
    use crate::{AccountBalanceQuery, AccountId, AccountIdOrAlias, AnyQuery};

    // language=JSON
    const ACCOUNT_BALANCE: &str = r#"{
  "accountBalance": {
    "accountId": "0.0.1001"
  }
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
        let source = assert_matches!(source, AccountIdOrAlias::AccountId(id) => id);

        assert_eq!(source.num, 1001);

        Ok(())
    }
}
