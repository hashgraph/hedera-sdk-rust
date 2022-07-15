use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::schedule_service_client::ScheduleServiceClient;
use serde::{
    Deserialize,
    Serialize,
};
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    ScheduleId,
    Transaction,
    TransactionId,
};

/// Marks a schedule in the network's action queue as deleted. Must be signed
/// by the admin key of the target schedule. A deleted schedule cannot
/// receive any additional signing keys, nor will it be executed.
pub type ScheduleDeleteTransaction = Transaction<ScheduleDeleteTransactionData>;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ScheduleDeleteTransactionData {
    schedule_id: Option<ScheduleId>,
}

impl ScheduleDeleteTransaction {
    /// Set the schedule to delete.
    pub fn schedule_id(&mut self, id: ScheduleId) -> &mut Self {
        self.body.data.schedule_id = Some(id);
        self
    }
}

#[async_trait]
impl TransactionExecute for ScheduleDeleteTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        ScheduleServiceClient::new(channel).delete_schedule(request).await
    }
}

impl ToTransactionDataProtobuf for ScheduleDeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let schedule_id = self.schedule_id.as_ref().map(ScheduleId::to_protobuf);

        services::transaction_body::Data::ScheduleDelete(services::ScheduleDeleteTransactionBody {
            schedule_id,
        })
    }
}

impl From<ScheduleDeleteTransactionData> for AnyTransactionData {
    fn from(transaction: ScheduleDeleteTransactionData) -> Self {
        Self::ScheduleDelete(transaction)
    }
}
