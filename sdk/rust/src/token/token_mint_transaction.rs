use async_trait::async_trait;
use tonic::transport::Channel;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use serde_with::base64::Base64;
use serde_with::{serde_as, skip_serializing_none};

use crate::protobuf::ToProtobuf;
use crate::{AccountId, TokenId, Transaction, TransactionId};
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};

/// Mints tokens to the Token's treasury Account.
///
/// The operation increases the Total Supply of the Token. The maximum total supply a token can have
/// is 2^63-1.
///
/// The amount provided must be in the lowest denomination possible. Example: Token A has 2 decimals.
/// In order to mint 100 tokens, one must provide amount of 10000. In order to mint 100.55 tokens,
/// one must provide amount of 10055.
///
/// - If no Supply Key is defined, the transaction will resolve to `TokenHasNoSupplyKey`.
/// - If both amount and metadata list get filled, a `InvalidTransactionBody` response code will be
/// returned.
/// - If the metadata list contains metadata which is too large, a `MetadataTooLong` response code will
/// be returned.
/// - If neither the amount nor the metadata list get filled, a `InvalidTokenMintAmount` response code
/// will be returned.
/// - If the metadata list count is greater than the batch size limit global dynamic property, a
/// `BatchSizeLimitExceeded` response code will be returned.
pub type TokenMintTransaction = Transaction<TokenMintTransactionData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenMintTransactionData {
    /// The token for which to mint tokens. If token does not exist, transaction results in
    /// `InvalidTokenId`
    token_id: Option<TokenId>,

    /// Applicable to tokens of type [`FungibleCommon`][TokenType::FungibleCommon].
    ///
    /// The amount to mint to the Treasury Account.
    /// 
    /// Amount must be a positive non-zero number represented in the lowest denomination of the
    /// token. The new supply must be lower than 2^63.
    amount: u64,

    /// Applicable to tokens of type [`NonFungibleUnique`][TokenType::NonFungibleUnique].
    ///
    /// A list of metadata that are being created.
    ///
    /// Maximum allowed size of each metadata is 100 bytes
    #[serde_as(as = "Vec<Base64>")]
    metadata: Vec<Vec<u8>>,
}

impl TokenMintTransaction {
    /// Sets the token for which to mint tokens. If token does not exist, transaction results in
    /// `InvalidTokenId`
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.body.data.token_id = Some(token_id.into());
        self
    }

    /// Applicable to tokens of type [`FungibleCommon`][TokenType::FungibleCommon].
    ///
    /// Sets the amount to mint to the Treasury Account.
    ///
    /// Amount must be a positive non-zero number represented in the lowest denomination of the
    /// token. The new supply must be lower than 2^63.
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.body.data.amount = amount;
        self
    }

    /// Applicable to tokens of type [`NonFungibleUnique`][TokenType::NonFungibleUnique].
    ///
    /// Sets the metadata to be added to the created token.
    ///
    /// Maximum allowed size of each metadata is 100 bytes
    pub fn metadata(&mut self, metadata: impl Into<Vec<Vec<u8>>>) -> &mut Self {
        self.body.data.metadata = metadata.into();
        self
    }
}

#[async_trait]
impl TransactionExecute for TokenMintTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        TokenServiceClient::new(channel).mint_token(request).await
    }
}

impl ToTransactionDataProtobuf for TokenMintTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let token = self.token_id.as_ref().map(TokenId::to_protobuf);
        let amount = self.amount;
        let metadata = self.metadata.clone();

        services::transaction_body::Data::TokenMint(services::TokenMintTransactionBody {
            token,
            amount,
            metadata,
        })
    }
}

impl From<TokenMintTransactionData> for AnyTransactionData {
    fn from(transaction: TokenMintTransactionData) -> Self {
        Self::TokenMint(transaction)
    }
}
