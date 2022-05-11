use async_trait::async_trait;
use hedera_proto::services::{self, crypto_service_client::CryptoServiceClient};
use tonic::transport::Channel;

use crate::{
    transaction::{ToTransactionDataProtobuf, TransactionExecute},
    AccountId, ToProtobuf, Transaction, TransactionId,
};

pub(super) type PaymentTransaction = Transaction<PaymentTransactionData>;

#[derive(Default)]
pub(super) struct PaymentTransactionData {
    // TODO: Use Hbar
    pub(super) amount: Option<u64>,
    pub(super) max_amount: Option<Option<u64>>,
}

#[async_trait]
impl TransactionExecute for PaymentTransaction {
    async fn execute(
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        CryptoServiceClient::new(channel).crypto_transfer(request).await
    }
}

impl ToTransactionDataProtobuf for PaymentTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        node_account_id: AccountId,
        transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let amount = self.amount.unwrap_or_default();

        services::transaction_body::Data::CryptoTransfer(services::CryptoTransferTransactionBody {
            token_transfers: Vec::new(),
            transfers: Some(services::TransferList {
                account_amounts: vec![
                    services::AccountAmount {
                        account_id: Some(node_account_id.to_protobuf()),
                        amount: amount as i64,
                        is_approval: false,
                    },
                    services::AccountAmount {
                        account_id: Some(transaction_id.account_id.to_protobuf()),
                        amount: -(amount as i64),
                        is_approval: false,
                    },
                ],
            }),
        })
    }
}
