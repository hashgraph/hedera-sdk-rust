use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::freeze_service_client::FreezeServiceClient;
use serde::{
    Deserialize,
    Serialize,
};
use time::OffsetDateTime;
use tonic::transport::Channel;

use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    FileId,
    FreezeType,
    ToProtobuf,
    Transaction,
};

/// Set the freezing period in which the platform will stop creating
/// events and accepting transactions.
///
/// This is used before safely shut down the platform for maintenance.
///
pub type FreezeTransaction = Transaction<FreezeTransactionData>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct FreezeTransactionData {
    start_time: Option<OffsetDateTime>,
    file_id: Option<FileId>,
    file_hash: Option<Vec<u8>>,
    freeze_type: FreezeType,
}

impl FreezeTransaction {
    /// Sets the start time.
    pub fn start_time(&mut self, time: OffsetDateTime) -> &mut Self {
        self.body.data.start_time = Some(time);
        self
    }

    /// Sets the freeze type.
    pub fn freeze_type(&mut self, ty: FreezeType) -> &mut Self {
        self.body.data.freeze_type = ty;
        self
    }

    /// Sets the file ID.
    pub fn file_id(&mut self, id: FileId) -> &mut Self {
        self.body.data.file_id = Some(id);
        self
    }

    /// Sets the file hash.
    pub fn file_hash(&mut self, hash: Vec<u8>) -> &mut Self {
        self.body.data.file_hash = Some(hash);
        self
    }
}

#[async_trait]
impl TransactionExecute for FreezeTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        FreezeServiceClient::new(channel).freeze(request).await
    }
}

impl ToTransactionDataProtobuf for FreezeTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let start_time = self.start_time.map(Into::into);
        let file_id = self.file_id.as_ref().map(FileId::to_protobuf);

        services::transaction_body::Data::Freeze(services::FreezeTransactionBody {
            update_file: file_id,
            file_hash: self.file_hash.clone().unwrap_or_default(),
            start_time,
            freeze_type: self.freeze_type as _,
            ..Default::default()
        })
    }
}

impl From<FreezeTransactionData> for AnyTransactionData {
    fn from(transaction: FreezeTransactionData) -> Self {
        Self::Freeze(transaction)
    }
}
