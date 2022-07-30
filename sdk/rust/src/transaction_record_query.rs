use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use hedera_proto::services::response::Response;
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    Error,
    FromProtobuf,
    Query,
    Status,
    ToProtobuf,
    TransactionId,
    TransactionRecord,
};

/// Get the record of a transaction, given its transaction ID.
///
pub type TransactionRecordQuery = Query<TransactionRecordQueryData>;

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRecordQueryData {
    transaction_id: Option<TransactionId>,
    include_children: bool,
    include_duplicates: bool,
    validate_status: bool,
}

impl From<TransactionRecordQueryData> for AnyQueryData {
    #[inline]
    fn from(data: TransactionRecordQueryData) -> Self {
        Self::TransactionRecord(data)
    }
}

impl TransactionRecordQuery {
    /// Set the ID of the transaction for which the record is being requested.
    pub fn transaction_id(&mut self, transaction_id: TransactionId) -> &mut Self {
        self.data.transaction_id = Some(transaction_id);
        self
    }

    /// Whether the response should include the records of any child transactions spawned by the
    /// top-level transaction with the given transaction.
    pub fn include_children(&mut self, include: bool) -> &mut Self {
        self.data.include_children = include;
        self
    }

    /// Whether records of processing duplicate transactions should be returned.
    pub fn include_duplicates(&mut self, include: bool) -> &mut Self {
        self.data.include_duplicates = include;
        self
    }

    /// Whether the record status should be validated.
    pub fn validate_status(&mut self, validate: bool) -> &mut Self {
        self.data.validate_status = validate;
        self
    }
}

impl ToQueryProtobuf for TransactionRecordQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let transaction_id = self.transaction_id.as_ref().map(|id| id.to_protobuf());

        services::Query {
            query: Some(services::query::Query::TransactionGetRecord(
                services::TransactionGetRecordQuery {
                    header: Some(header),
                    transaction_id,
                    include_child_records: self.include_children,
                    include_duplicates: self.include_duplicates,
                },
            )),
        }
    }
}

#[async_trait]
impl QueryExecute for TransactionRecordQueryData {
    type Response = TransactionRecord;

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
        CryptoServiceClient::new(channel).get_tx_record_by_tx_id(request).await
    }

    fn should_retry_pre_check(&self, status: Status) -> bool {
        matches!(status, Status::ReceiptNotFound | Status::RecordNotFound)
    }

    fn make_response(&self, response: Response) -> crate::Result<Self::Response> {
        let record = <TransactionRecord as FromProtobuf<_>>::from_protobuf(response)?;

        if self.validate_status && record.receipt.status != Status::Success {
            return Err(Error::ReceiptStatus {
                // NOTE: it should be impossible to get here without a transaction ID set
                transaction_id: self.transaction_id.unwrap(),
                status: record.receipt.status,
            });
        }

        Ok(record)
    }
}
