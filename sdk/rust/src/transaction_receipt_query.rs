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
    Query,
    Status,
    ToProtobuf,
    TransactionId,
    TransactionReceipt,
};

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
    include_children: bool,
    include_duplicates: bool,
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

    /// Whether the response should include the receipts of any child transactions spawned by the
    /// top-level transaction with the given transaction.
    pub fn include_children(&mut self, include: bool) -> &mut Self {
        self.data.include_children = include;
        self
    }

    /// Whether receipts of processing duplicate transactions should be returned.
    pub fn include_duplicates(&mut self, include: bool) -> &mut Self {
        self.data.include_duplicates = include;
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
                    include_child_receipts: self.include_children,
                    include_duplicates: self.include_duplicates,
                },
            )),
        }
    }
}

#[async_trait]
impl QueryExecute for TransactionReceiptQueryData {
    type Response = TransactionReceipt;

    fn is_payment_required(&self) -> bool {
        false
    }

    fn transaction_id(&self) -> Option<TransactionId> {
        self.transaction_id
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        CryptoServiceClient::new(channel).get_transaction_receipts(request).await
    }

    fn should_retry_pre_check(&self, status: Status) -> bool {
        matches!(status, Status::ReceiptNotFound | Status::RecordNotFound)
    }

    fn should_retry(&self, response: &services::Response) -> bool {
        // extract the receipt status from the receipt
        // without altering or freeing the memory from the response

        let receipt_status = loop {
            if let Some(services::response::Response::TransactionGetReceipt(r)) = &response.response
            {
                if let Some(receipt) = &r.receipt {
                    if let Some(status) = Status::from_i32(receipt.status) {
                        break status;
                    }
                }
            }

            return false;
        };

        matches!(receipt_status, Status::Unknown)
    }
}
