use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use serde_with::skip_serializing_none;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountAddress, AccountId, Transaction};

/// Mark an account as deleted, moving all its current hbars to another account.
///
/// It will remain in the ledger, marked as deleted, until it expires.
/// Transfers into it a deleted account will fail.
///
pub type AccountDeleteTransaction = Transaction<AccountDeleteTransactionData>;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDeleteTransactionData {
    /// The account ID which will receive all remaining hbars.
    pub transfer_account_id: Option<AccountAddress>,

    /// The account ID which should be deleted.
    pub delete_account_id: Option<AccountAddress>,
}

impl AccountDeleteTransaction {
    /// Sets the account ID which should be deleted.
    pub fn delete_account_id(&mut self, id: impl Into<AccountAddress>) -> &mut Self {
        self.body.data.delete_account_id = Some(id.into());
        self
    }

    /// Sets the account ID which will receive all remaining hbars.
    pub fn transfer_account_id(&mut self, id: impl Into<AccountAddress>) -> &mut Self {
        self.body.data.transfer_account_id = Some(id.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for AccountDeleteTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        CryptoServiceClient::new(channel).crypto_delete(request).await
    }
}

impl ToTransactionDataProtobuf for AccountDeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let delete_account_id = self.delete_account_id.as_ref().map(AccountAddress::to_protobuf);
        let transfer_account_id =
            self.transfer_account_id.as_ref().map(AccountAddress::to_protobuf);

        services::transaction_body::Data::CryptoDelete(services::CryptoDeleteTransactionBody {
            delete_account_id,
            transfer_account_id,
        })
    }
}

impl From<AccountDeleteTransactionData> for AnyTransactionData {
    fn from(transaction: AccountDeleteTransactionData) -> Self {
        Self::AccountDelete(transaction)
    }
}
