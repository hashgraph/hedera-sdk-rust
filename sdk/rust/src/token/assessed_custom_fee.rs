use hedera_proto::services;

use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::{
    AccountId,
    TokenId,
};

/// A custom transfer fee that was assessed during the handling of a CryptoTransfer.
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct AssessedCustomFee {
    /// The amount of currency charged to each payer.
    pub amount: i64,

    /// The currency `amount` is charged in, if `None` the fee is in HBar.
    pub token_id: Option<TokenId>,

    /// The account that receives the fees that were charged.
    pub fee_collector_account_id: Option<AccountId>,

    /// A list of all accounts that were charged this fee.
    pub payer_account_id_list: Vec<AccountId>,
}

impl AssessedCustomFee {
    /// Create a new `AssessedCustomFee` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        ToProtobuf::to_bytes(self)
    }
}

impl FromProtobuf<services::AssessedCustomFee> for AssessedCustomFee {
    fn from_protobuf(pb: services::AssessedCustomFee) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            amount: pb.amount,
            token_id: pb.token_id.map(TokenId::from_protobuf).transpose()?,
            fee_collector_account_id: FromProtobuf::from_protobuf(pb.fee_collector_account_id)?,
            payer_account_id_list: FromProtobuf::from_protobuf(pb.effective_payer_account_id)?,
        })
    }
}

impl ToProtobuf for AssessedCustomFee {
    type Protobuf = services::AssessedCustomFee;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::AssessedCustomFee {
            amount: self.amount,
            token_id: self.token_id.to_protobuf(),
            fee_collector_account_id: self.fee_collector_account_id.to_protobuf(),
            effective_payer_account_id: self.payer_account_id_list.to_protobuf(),
        }
    }
}
