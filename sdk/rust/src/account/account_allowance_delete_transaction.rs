use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountAddress,
    AccountId,
    NftId,
    TokenId,
    Transaction,
};

/// Deletes one or more non-fungible approved allowances from an owner's account. This operation
/// will remove the allowances granted to one or more specific non-fungible token serial numbers. Each owner account
/// listed as wiping an allowance must sign the transaction. Hbar and fungible token allowances
/// can be removed by setting the amount to zero in [AccountAllowanceApproveTransaction].
///
pub type AccountAllowanceDeleteTransaction = Transaction<AccountAllowanceDeleteTransactionData>;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAllowanceDeleteTransactionData {
    pub nft_allowances: Vec<NftRemoveAllowance>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NftRemoveAllowance {
    /// token that the allowance pertains to
    pub token_id: TokenId,

    /// The account ID that owns token.
    pub owner_account_id: AccountAddress,

    /// The list of serial numbers to remove allowances from.
    pub serial_numbers: Vec<i64>,
}

impl AccountAllowanceDeleteTransaction {
    /// Remove all nft token allowances.
    pub fn delete_all_token_nft_allowances(
        &mut self,
        nft_id: NftId,
        owner_account_id: impl Into<AccountAddress>,
    ) -> &mut Self {
        let owner_account_id = owner_account_id.into();

        if let Some(allowance) = self.body.data.nft_allowances.iter_mut().find(|allowance| {
            allowance.token_id == nft_id.token_id && allowance.owner_account_id == owner_account_id
        }) {
            allowance.serial_numbers.push(nft_id.serial_number as i64);
        } else {
            self.body.data.nft_allowances.push(NftRemoveAllowance {
                token_id: nft_id.token_id,
                serial_numbers: vec![nft_id.serial_number as i64],
                owner_account_id,
            })
        }

        self
    }
}

#[async_trait]
impl TransactionExecute for AccountAllowanceDeleteTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        CryptoServiceClient::new(channel).delete_allowances(request).await
    }
}

impl ToTransactionDataProtobuf for AccountAllowanceDeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let nft_allowances =
            self.nft_allowances.iter().map(|allowance| allowance.to_protobuf()).collect();

        services::transaction_body::Data::CryptoDeleteAllowance(
            services::CryptoDeleteAllowanceTransactionBody { nft_allowances },
        )
    }
}

impl From<AccountAllowanceDeleteTransactionData> for AnyTransactionData {
    fn from(transaction: AccountAllowanceDeleteTransactionData) -> Self {
        Self::AccountAllowanceDelete(transaction)
    }
}

impl ToProtobuf for NftRemoveAllowance {
    type Protobuf = services::NftRemoveAllowance;

    fn to_protobuf(&self) -> Self::Protobuf {
        Self::Protobuf {
            token_id: Some(self.token_id.to_protobuf()),
            owner: Some(self.owner_account_id.to_protobuf()),
            serial_numbers: self.serial_numbers.clone(),
        }
    }
}
