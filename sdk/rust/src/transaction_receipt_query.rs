use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

use crate::query::{AnyQueryData, QueryExecute, ToQueryProtobuf};
use crate::{Query, ToProtobuf, TransactionId, TransactionReceipt};

// TODO: support children
// TODO: support duplicates

/// Get the receipt of a transaction, given its transaction ID.
///
/// Once a transaction reaches consensus, then information about whether it succeeded or failed
/// will be available until the end of the receipt period.
///
pub type TransactionReceiptQuery = Query<TransactionReceiptQueryData>;

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceiptQueryData {
    transaction_id: Option<TransactionId>,
}

impl From<TransactionReceiptQueryData> for AnyQueryData {
    #[inline]
    fn from(data: TransactionReceiptQueryData) -> Self {
        Self::TransactionReceipt(data)
    }
}

impl TransactionReceiptQuery {
    /// Set the ID of the transaction for which the receipt is being requested.
    pub fn transaction_id(&mut self, transaction_id: TransactionId) -> &mut Self {
        self.data.transaction_id = Some(transaction_id);
        self
    }
}

impl ToQueryProtobuf for TransactionReceiptQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let transaction_id = self.transaction_id.as_ref().map(|id| id.to_protobuf());

        services::Query {
            query: Some(services::query::Query::TransactionGetReceipt(
                services::TransactionGetReceiptQuery {
                    header: Some(header),
                    transaction_id,
                    include_child_receipts: false,
                    include_duplicates: false,
                },
            )),
        }
    }
}

#[async_trait]
impl QueryExecute for TransactionReceiptQueryData {
    type Response = TransactionReceipt;

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        CryptoServiceClient::new(channel).get_transaction_receipts(request).await
    }
}
