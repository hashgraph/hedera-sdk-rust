use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use serde_with::skip_serializing_none;
use tonic::transport::Channel;

use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountId, ToProtobuf, Transaction, TransactionId};

pub type PaymentTransaction = Transaction<PaymentTransactionData>;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentTransactionData {
    pub(crate) amount: Option<u64>,
    pub(crate) max_amount: Option<u64>,
}

#[async_trait]
impl TransactionExecute for PaymentTransactionData {
    // noinspection DuplicatedCode
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        CryptoServiceClient::new(channel).crypto_transfer(request).await
    }
}

impl ToTransactionDataProtobuf for PaymentTransactionData {
    #[allow(clippy::cast_possible_wrap)]
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

impl From<PaymentTransactionData> for AnyTransactionData {
    fn from(_transaction: PaymentTransactionData) -> Self {
        // NOTE: this should only be reached if we try to serialize a PaymentTransaction
        //  as this is a private type that we have no intention of serializing, we should be good
        unreachable!()
    }
}
