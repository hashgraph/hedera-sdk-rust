use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

use crate::transaction::{ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountIdOrAlias, ToProtobuf, Transaction};

/// Transfers cryptocurrency among two or more accounts by making the desired adjustments to their
/// balances.
///
/// Each transfer_transaction list can specify up to 10 adjustments. Each negative amount is withdrawn
/// from the corresponding account (a sender), and each positive one is added to the corresponding
/// account (a receiver). The amounts list must sum to zero.
///
pub type TransferTransaction = Transaction<TransferTransactionData>;

#[derive(Debug, Default)]
pub struct TransferTransactionData {
    hbar_transfers: Vec<HbarTransfer>,
    // TODO: token_transfers
    // TODO: nft_transfers
}

#[derive(Debug)]
struct HbarTransfer {
    account: AccountIdOrAlias,
    amount: i64,
}

impl TransferTransaction {
    // TODO: [hbar_transfer] or [transfer_hbar]
    pub fn hbar_transfer(
        &mut self,
        account: impl Into<AccountIdOrAlias>,
        amount: i64,
    ) -> &mut Self {
        self.data.hbar_transfers.push(HbarTransfer { account: account.into(), amount });
        self
    }

    // TODO: [hbar_transfer_to] or [transfer_hbar_to]
    pub fn hbar_transfer_to(
        &mut self,
        sender: impl Into<AccountIdOrAlias>,
        receiver: impl Into<AccountIdOrAlias>,
        amount: u64,
    ) -> &mut Self {
        self.hbar_transfer(sender, -(amount as i64));
        self.hbar_transfer(receiver, amount as i64);
        self
    }
}

#[async_trait]
impl TransactionExecute for TransferTransaction {
    async fn execute(
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        CryptoServiceClient::new(channel).crypto_transfer(request).await
    }
}

impl ToProtobuf for HbarTransfer {
    type Protobuf = services::AccountAmount;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::AccountAmount {
            amount: self.amount,
            account_id: Some(self.account.to_protobuf()),
            is_approval: false,
        }
    }
}

impl ToTransactionDataProtobuf for TransferTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: crate::AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let transfers = if !self.hbar_transfers.is_empty() {
            Some(services::TransferList {
                account_amounts: self
                    .hbar_transfers
                    .iter()
                    .map(|amount| amount.to_protobuf())
                    .collect(),
            })
        } else {
            None
        };

        services::transaction_body::Data::CryptoTransfer(services::CryptoTransferTransactionBody {
            transfers,
            token_transfers: Vec::new(),
        })
    }
}
