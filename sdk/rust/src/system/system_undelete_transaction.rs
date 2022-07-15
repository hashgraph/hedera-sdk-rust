use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::file_service_client::FileServiceClient;
use hedera_proto::services::smart_contract_service_client::SmartContractServiceClient;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    ContractId,
    FileId,
    Transaction,
};

pub type SystemUndeleteTransaction = Transaction<SystemUndeleteTransactionData>;

/// Undelete a file or smart contract that was deleted by SystemDelete.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SystemUndeleteTransactionData {
    pub file_id: Option<FileId>,
    pub contract_id: Option<ContractId>,
}

impl SystemUndeleteTransaction {
    /// Sets the contract ID to undelete.
    pub fn contract_id(&mut self, id: impl Into<ContractId>) -> &mut Self {
        self.body.data.file_id = None;
        self.body.data.contract_id = Some(id.into());
        self
    }

    /// Sets the file ID to undelete.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        self.body.data.contract_id = None;
        self.body.data.file_id = Some(id.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for SystemUndeleteTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        if self.file_id.is_some() {
            FileServiceClient::new(channel).system_undelete(request).await
        } else {
            SmartContractServiceClient::new(channel).system_undelete(request).await
        }
    }
}

impl ToTransactionDataProtobuf for SystemUndeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let contract_id = self.contract_id.as_ref().map(ContractId::to_protobuf);
        let file_id = self.file_id.as_ref().map(FileId::to_protobuf);

        let id = match (contract_id, file_id) {
            (Some(contract_id), _) => {
                Some(services::system_undelete_transaction_body::Id::ContractId(contract_id))
            }

            (_, Some(file_id)) => {
                Some(services::system_undelete_transaction_body::Id::FileId(file_id))
            }

            _ => None,
        };

        services::transaction_body::Data::SystemUndelete(services::SystemUndeleteTransactionBody {
            id,
        })
    }
}

impl From<SystemUndeleteTransactionData> for AnyTransactionData {
    fn from(transaction: SystemUndeleteTransactionData) -> Self {
        Self::SystemUndelete(transaction)
    }
}
