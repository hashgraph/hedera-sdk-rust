use std::ops::Not;

use async_trait::async_trait;
use itertools::Itertools;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountAddress, NftId, TokenId, ToProtobuf, Transaction};

/// Transfers cryptocurrency among two or more accounts by making the desired adjustments to their
/// balances.
///
/// Each transfer list can specify up to 10 adjustments. Each negative amount is withdrawn
/// from the corresponding account (a sender), and each positive one is added to the corresponding
/// account (a receiver). The amounts list must sum to zero.
///
pub type TransferTransaction = Transaction<TransferTransactionData>;

type TokenTransferList = Vec<TokenTransfer>;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferTransactionData {
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "tinybarTransfers")]
    hbar_transfers: Vec<HbarTransfer>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    token_transfers: TokenTransferList,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct HbarTransfer {
    account: AccountAddress,
    #[serde(default)]
    amount: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct TokenTransfer {
    token_id: TokenId,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    transfers: Vec<HbarTransfer>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    nft_transfers: Vec<NftTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_decimals: Option<u32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct NftTransfer {
    sender: AccountAddress,
    receiver: AccountAddress,
    serial_number: i64,
    is_approval: bool,
}

impl TransferTransaction {
    pub fn hbar_transfer(&mut self, account: impl Into<AccountAddress>, amount: i64) -> &mut Self {
        self.body.data.hbar_transfers.push(HbarTransfer { account: account.into(), amount });
        self
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn hbar_transfer_to(
        &mut self,
        sender: impl Into<AccountAddress>,
        receiver: impl Into<AccountAddress>,
        amount: u64,
    ) -> &mut Self {
        self.hbar_transfer(sender, -(amount as i64));
        self.hbar_transfer(receiver, amount as i64);
        self
    }

    pub fn token_transfer(&mut self, token_id: impl Into<TokenId>, account: impl Into<AccountAddress>, amount: i64) -> &mut Self {
        let transfer = HbarTransfer { account: account.into(), amount };
        let token_id = token_id.into();

        if let Some(token_transfer) = self.body.data.token_transfers.iter_mut().find(|tx| tx.token_id == token_id) {
            token_transfer.transfers.push(transfer);
        } else {
            let token_transfer = TokenTransfer { token_id, transfers: [transfer].into(), nft_transfers: Vec::new(), expected_decimals: None };
            self.body.data.token_transfers.push(token_transfer);
        }

        self
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn token_transfer_to(&mut self, token_id: impl Into<TokenId>, sender: impl Into<AccountAddress>, receiver: impl Into<AccountAddress>, amount: u64) -> &mut Self {
        let token_id = token_id.into();
        self.token_transfer(token_id, sender, -(amount as i64));
        self.token_transfer(token_id, receiver, amount as i64);
        self
    }

    pub fn nft_transfer_to(&mut self, nft_id: impl Into<NftId>, sender: impl Into<AccountAddress>, receiver: impl Into<AccountAddress>) -> &mut Self {
        let nft_id = nft_id.into();
        let transfer = NftTransfer { sender: sender.into(), receiver: receiver.into(), serial_number: nft_id.serial_number, is_approval: false };

        if let Some(token_transfer) = self.body.data.token_transfers.iter_mut().find(|tx| tx.token_id == nft_id.token_id) {
            token_transfer.nft_transfers.push(transfer);
        } else {
            let token_transfer = TokenTransfer { token_id: nft_id.token_id, transfers: Vec::new(), nft_transfers: [transfer].into(), expected_decimals: None };
            self.body.data.token_transfers.push(token_transfer);
        }

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

impl ToProtobuf for TokenTransfer {
    type Protobuf = services::TokenTransferList;

    fn to_protobuf(&self) -> Self::Protobuf {
        Self::Protobuf {
            token: Some(self.token_id.to_protobuf()),
            transfers: self.transfers.iter().map(HbarTransfer::to_protobuf).collect_vec(),
            nft_transfers: self.nft_transfers.iter().map(NftTransfer::to_protobuf).collect_vec(),
            expected_decimals: None,
        }
    }
}

impl ToProtobuf for NftTransfer {
    type Protobuf = services::NftTransfer;

    fn to_protobuf(&self) -> Self::Protobuf {
        Self::Protobuf {
            sender_account_id: Some(self.sender.to_protobuf()),
            receiver_account_id: Some(self.receiver.to_protobuf()),
            serial_number: self.serial_number as i64,
            is_approval: self.is_approval,
        }
    }
}

impl ToProtobuf for TokenTransferList {
    type Protobuf = Vec<services::TokenTransferList>;

    fn to_protobuf(&self) -> Self::Protobuf {
        self.iter().map(TokenTransfer::to_protobuf).collect_vec()
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

        let token_transfers = self.token_transfers.to_protobuf();

        services::transaction_body::Data::CryptoTransfer(services::CryptoTransferTransactionBody {
            transfers,
            token_transfers,
        })
    }
}

impl From<TransferTransactionData> for AnyTransactionData {
    fn from(transaction: TransferTransactionData) -> Self {
        Self::Transfer(transaction)
    }
}

// TODO update tests
#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use crate::transaction::{AnyTransaction, AnyTransactionData};
    use crate::{AccountId, TokenId, TransferTransaction};

    // language=JSON
    const TRANSFER_HBAR: &str = r#"{
  "$type": "transfer",
  "tinybarTransfers": [
    {
      "account": "0.0.1001",
      "amount": 20
    },
    {
      "account": "0.0.1002",
      "amount": -20
    }
  ],
  "payerAccountId": "0.0.6189"
}"#;

    // language=JSON
    const TRANSFER_TOKEN: &str = r#"{
  "$type": "transfer",
  "tokenTransfers": [
    {
      "token_id": "0.0.1001",
      "transfers": [
        {
          "account": "0.0.1002",
          "amount": 20
        },
        {
          "account": "0.0.1003",
          "amount": -20
        }
      ]
    }
  ],
  "payerAccountId": "0.0.6189"
}"#;

    // language=JSON
    const TRANSFER_NFT: &str = r#"{
  "$type": "transfer",
  "tokenTransfers": [
    {
      "token_id": "0.0.1001",
      "nft_transfers": [
        {
          "sender": "0.0.1002",
          "receiver": "0.0.1003",
          "serial_number": 12345,
          "is_approval": false
        }
      ]
    }
  ],
  "payerAccountId": "0.0.6189"
}"#;

    #[test]
    fn it_should_serialize_hbar_transfer() -> anyhow::Result<()> {
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
    fn it_should_serialize_token_transfer() -> anyhow::Result<()> {
        let mut transaction = TransferTransaction::new();
        transaction
            .token_transfer(TokenId::from(1001), AccountId::from(1002), 20)
            .token_transfer(TokenId::from(1001), AccountId::from(1003), -20)
            .payer_account_id(AccountId::from(6189));

        let transaction_json = serde_json::to_string_pretty(&transaction)?;
        assert_eq!(transaction_json, TRANSFER_TOKEN);

        Ok(())
    }

    #[test]
    fn it_should_serialize_nft_transfer() -> anyhow::Result<()> {
        let mut transaction = TransferTransaction::new();
        transaction
            .nft_transfer_to((TokenId::from(1001), 12345), AccountId::from(1002), AccountId::from(1003))
            .payer_account_id(AccountId::from(6189));

        let transaction_json = serde_json::to_string_pretty(&transaction)?;
        assert_eq!(transaction_json, TRANSFER_NFT);

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
