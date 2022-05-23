use std::ops::Not;

use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountIdOrAlias, ToProtobuf, Transaction};

/// Transfers cryptocurrency among two or more accounts by making the desired adjustments to their
/// balances.
///
/// Each transfer list can specify up to 10 adjustments. Each negative amount is withdrawn
/// from the corresponding account (a sender), and each positive one is added to the corresponding
/// account (a receiver). The amounts list must sum to zero.
///
pub type TransferTransaction = Transaction<TransferTransactionData>;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferTransactionData {
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "tinybarTransfers")]
    hbar_transfers: Vec<HbarTransfer>,
    // TODO: token_transfers
    // TODO: nft_transfers
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct HbarTransfer {
    account: AccountIdOrAlias,
    #[serde(default)]
    amount: i64,
}

impl TransferTransaction {
    // TODO: [hbar_transfer] or [transfer_hbar]
    pub fn hbar_transfer(
        &mut self,
        account: impl Into<AccountIdOrAlias>,
        amount: i64,
    ) -> &mut Self {
        self.body.data.hbar_transfers.push(HbarTransfer { account: account.into(), amount });
        self
    }

    // TODO: [hbar_transfer_to] or [transfer_hbar_to]
    #[allow(clippy::cast_possible_wrap)]
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
impl TransactionExecute for TransferTransactionData {
    // noinspection DuplicatedCode
    async fn execute(
        &self,
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
        let transfers = self.hbar_transfers.is_empty().not().then(|| services::TransferList {
            account_amounts: self.hbar_transfers.iter().map(HbarTransfer::to_protobuf).collect(),
        });

        services::transaction_body::Data::CryptoTransfer(services::CryptoTransferTransactionBody {
            transfers,
            token_transfers: Vec::new(),
        })
    }
}

impl From<TransferTransactionData> for AnyTransactionData {
    fn from(transaction: TransferTransactionData) -> Self {
        Self::Transfer(transaction)
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use crate::transaction::{AnyTransaction, AnyTransactionData};
    use crate::{AccountId, TransferTransaction};

    // language=JSON
    const TRANSFER_HBAR: &str = r#"{
  "transfer": {
    "tinybarTransfers": [
      {
        "account": "0.0.1001",
        "amount": 20
      },
      {
        "account": "0.0.1002",
        "amount": -20
      }
    ]
  },
  "payerAccountId": "0.0.6189"
}"#;

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut transaction = TransferTransaction::new();
        transaction
            .hbar_transfer(AccountId::from(1001), 20)
            .hbar_transfer(AccountId::from(1002), -20)
            .payer_account_id(AccountId::from(6189));

        let s = serde_json::to_string_pretty(&transaction)?;
        assert_eq!(s, TRANSFER_HBAR);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let transaction: AnyTransaction = serde_json::from_str(TRANSFER_HBAR)?;

        let data = assert_matches!(transaction.body.data, AnyTransactionData::Transfer(transaction) => transaction);

        assert_eq!(data.hbar_transfers.len(), 2);

        assert_eq!(data.hbar_transfers[0].amount, 20);
        assert_eq!(data.hbar_transfers[1].amount, -20);

        Ok(())
    }
}
