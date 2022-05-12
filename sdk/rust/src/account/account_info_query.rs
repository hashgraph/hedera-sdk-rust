use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

use crate::account::AccountInfo;
use crate::query::{AnyQueryData, QueryData, QueryExecute, ToQueryProtobuf};
use crate::{AccountIdOrAlias, Query, ToProtobuf};

/// Get all the information about an account, including the balance.
///
/// This does not get the list of account records.
///
pub type AccountInfoQuery = Query<AccountInfoQueryData>;

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct AccountInfoQueryData {
    account_id: Option<AccountIdOrAlias>,
}

impl From<AccountInfoQueryData> for AnyQueryData {
    #[inline]
    fn from(data: AccountInfoQueryData) -> Self {
        Self::AccountInfo(data)
    }
}

impl QueryData for AccountInfoQueryData {}

impl AccountInfoQuery {
    /// Sets the account ID for which information is requested.
    pub fn account_id(&mut self, id: AccountIdOrAlias) -> &mut Self {
        self.data.account_id = Some(id);
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
impl QueryExecute for AccountInfoQuery {
    type Response = AccountInfo;

    async fn execute(
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        CryptoServiceClient::new(channel).get_account_info(request).await
    }
}
