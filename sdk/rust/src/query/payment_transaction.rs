use async_trait::async_trait;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use hedera_proto::services::{self};
use serde::{Deserialize, Deserializer};
use serde_with::skip_serializing_none;
use time::Duration;
use tonic::transport::Channel;

use crate::transaction::{ToTransactionDataProtobuf, TransactionBody, TransactionExecute};
use crate::{AccountId, ToProtobuf, Transaction, TransactionId};

pub(super) type PaymentTransaction = Transaction<PaymentTransactionData>;

#[skip_serializing_none]
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PaymentTransactionData {
    // TODO: Use Hbar
    pub(super) amount: Option<u64>,
    pub(super) max_amount: Option<u64>,
}

#[async_trait]
impl TransactionExecute for PaymentTransactionData {
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

// TODO: this is identical to AnyTransaction

#[derive(serde::Deserialize, Debug)]
struct PaymentTransactionBodyProxy {
    data: PaymentTransactionData,
    node_account_ids: Option<Vec<AccountId>>,
    #[serde(with = "crate::serde::duration_opt")]
    transaction_valid_duration: Option<Duration>,
    max_transaction_fee: Option<u64>,
    #[serde(skip_serializing_if = "crate::serde::skip_if_string_empty")]
    transaction_memo: String,
    payer_account_id: Option<AccountId>,
    transaction_id: Option<TransactionId>,
}

impl<'de> Deserialize<'de> for PaymentTransaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        <PaymentTransactionBodyProxy as Deserialize>::deserialize(deserializer).map(|body| Self {
            body: TransactionBody {
                data: body.data,
                node_account_ids: body.node_account_ids,
                transaction_valid_duration: body.transaction_valid_duration,
                max_transaction_fee: body.max_transaction_fee,
                transaction_memo: body.transaction_memo,
                payer_account_id: body.payer_account_id,
                transaction_id: body.transaction_id,
            },
            signers: Vec::new(),
        })
    }
}
