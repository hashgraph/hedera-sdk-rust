use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

use crate::account::AccountInfo;
use crate::query::{AnyQueryData, QueryExecute, ToQueryProtobuf};
use crate::{AccountAddress, Query, ToProtobuf};

/// Get all the information about an account, including the balance.
///
/// This does not get the list of account records.
///
pub type AccountInfoQuery = Query<AccountInfoQueryData>;

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoQueryData {
    account_id: Option<AccountAddress>,
}

impl From<AccountInfoQueryData> for AnyQueryData {
    #[inline]
    fn from(data: AccountInfoQueryData) -> Self {
        Self::AccountInfo(data)
    }
}

impl AccountInfoQuery {
    /// Sets the account ID for which information is requested.
    pub fn account_id(&mut self, id: impl Into<AccountAddress>) -> &mut Self {
        self.data.account_id = Some(id.into());
        self
    }
}

impl ToQueryProtobuf for AccountInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let account_id = self.account_id.as_ref().map(|id| id.to_protobuf());

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

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        CryptoServiceClient::new(channel).get_account_info(request).await
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use crate::query::AnyQueryData;
    use crate::{AccountAddress, AccountId, AccountInfoQuery, AnyQuery};

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
        let account_id =
            assert_matches!(data.account_id, Some(AccountAddress::AccountId(id)) => id);

        assert_eq!(account_id.num, 1001);
        assert_eq!(query.payment.body.data.amount, Some(50));
        assert_eq!(query.payment.body.transaction_memo, "query payment");
        assert_eq!(query.payment.body.payer_account_id, Some(AccountId::from(6189)));

        Ok(())
    }
}
