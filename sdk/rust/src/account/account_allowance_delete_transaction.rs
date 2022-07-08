use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use serde_with::skip_serializing_none;
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
    TokenId,
    Transaction,
};

/// Deletes one or more non-fungible approved allowances from an owner's account. This operation
/// will remove the allowances granted to one or more specific non-fungible token serial numbers. Each owner account
/// listed as wiping an allowance must sign the transaction. Hbar and fungible token allowances
/// can be removed by setting the amount to zero in CryptoApproveAllowance.
///
pub type AccountDeleteAllowanceTransaction = Transaction<AccountDeleteAllowanceTransactionData>;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDeleteAllowanceTransactionData {
    pub nft_allowances: Vec<NftRemoveAllowanceData>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NftRemoveAllowanceData {
    /// token that the allowance pertains to
    pub token_id: Option<TokenId>,

    /// The account ID that owns token.
    pub owner: Option<AccountAddress>,

    /// The list of serial numbers to remove allowances from.
    pub serial_numbers: Vec<i64>,
}

impl AccountDeleteAllowanceTransaction {
    /// Sets the account ID which should be deleted.
    pub fn nft_allowances(&mut self, id: impl Into<Vec<NftRemoveAllowanceData>>) -> &mut Self {
        self.body.data.nft_allowances = id.into();
        self
    }
}

#[async_trait]
impl TransactionExecute for AccountDeleteAllowanceTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        CryptoServiceClient::new(channel).delete_allowances(request).await
    }
}

impl ToTransactionDataProtobuf for AccountDeleteAllowanceTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let nft_allowances =
            self.nft_allowances.iter().map(|allowance| allowance.to_protobuf()).collect::<Vec<_>>();
        services::transaction_body::Data::CryptoDeleteAllowance(
            services::CryptoDeleteAllowanceTransactionBody { nft_allowances },
        )
    }
}

impl From<AccountDeleteAllowanceTransactionData> for AnyTransactionData {
    fn from(transaction: AccountDeleteAllowanceTransactionData) -> Self {
        Self::AccountDeleteAllowance(transaction)
    }
}

impl ToProtobuf for NftRemoveAllowanceData {
    type Protobuf = services::NftRemoveAllowance;

    fn to_protobuf(&self) -> Self::Protobuf {
        Self::Protobuf {
            token_id: self.token_id.as_ref().map(|id| id.to_protobuf()),
            owner: self.owner.as_ref().map(|id| id.to_protobuf()),
            serial_numbers: self.serial_numbers.clone(),
        }
    }
}
