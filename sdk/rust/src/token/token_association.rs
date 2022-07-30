use hedera_proto::services;

use crate::{
    AccountId,
    FromProtobuf,
    TokenId,
};

/// A token <-> account association.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAssociation {
    /// The token involved in the association.
    pub token_id: TokenId,

    /// The account involved in the association.
    pub account_id: AccountId,
}

impl FromProtobuf<services::TokenAssociation> for TokenAssociation {
    fn from_protobuf(pb: services::TokenAssociation) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let token_id = pb_getf!(pb, token_id)?;
        let account_id = pb_getf!(pb, account_id)?;

        Ok(Self {
            token_id: TokenId::from_protobuf(token_id)?,
            account_id: AccountId::from_protobuf(account_id)?,
        })
    }
}
