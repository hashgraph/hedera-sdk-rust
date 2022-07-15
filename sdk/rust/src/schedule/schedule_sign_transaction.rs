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

/// Adds zero or more signing keys to a schedule.
pub type ScheduleSignTransaction = Transaction<ScheduleSignTransactionData>;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ScheduleSignTransactionData {
    schedule_id: Option<ScheduleId>,
}

impl ScheduleSignTransaction {
    /// Set the schedule to add signing keys to.
    pub fn schedule_id(&mut self, id: ScheduleId) -> &mut Self {
        self.body.data.schedule_id = Some(id);
        self
    }
}

#[async_trait]
impl TransactionExecute for ScheduleSignTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        ScheduleServiceClient::new(channel).delete_schedule(request).await
    }
}

impl ToTransactionDataProtobuf for ScheduleSignTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let schedule_id = self.schedule_id.as_ref().map(ScheduleId::to_protobuf);

        services::transaction_body::Data::ScheduleSign(services::ScheduleSignTransactionBody {
            schedule_id,
        })
    }
}

impl From<ScheduleSignTransactionData> for AnyTransactionData {
    fn from(transaction: ScheduleSignTransactionData) -> Self {
        Self::ScheduleSign(transaction)
    }
}
