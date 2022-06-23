use hedera_proto::services;
use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::serde_as;
use time::OffsetDateTime;

use crate::{AccountId, FromProtobuf, NftId};

/// Response from [`TokenNftInfoQuery`][crate::TokenNftInfoQuery].
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenNftInfoResponse {
    /// The ID of the NFT.
    pub nft_id: NftId,

    /// The current owner of the NFT.
    pub account_id: AccountId,

    /// Effective consensus timestamp at which the NFT was minted.
    pub creation_time: OffsetDateTime,

    /// The unique metadata of the NFT
    #[serde_as(as = "Base64")]
    pub metadata: Vec<u8>,

    // /// The ledger ID the response was returned from.
    // TODO pub ledger_id: LedgerId, --- also shows as todo in account_info.rs

    /// If an allowance is granted for the NFT, its corresponding spender account.
    pub spender_id: Option<AccountId>,
}

impl FromProtobuf for TokenNftInfoResponse {
    type Protobuf = services::response::Response;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
        where
            Self: Sized,
    {
        let pb = pb_getv!(pb, TokenGetNftInfo, services::response::Response);
        let nft = pb_getf!(pb, nft)?;

        let nft_id = pb_getf!(nft, nft_id)?;
        let account_id = pb_getf!(nft, account_id)?;
        let creation_time = nft.creation_time.unwrap();
        let metadata = nft.metadata;
        // TODO let ledger_id = nft.ledger_id;

        let spender_id = nft.spender_id
            .map(AccountId::from_protobuf)
            .map(Result::ok)
            .flatten();

        Ok(Self {
            nft_id: NftId::from_protobuf(nft_id)?,
            account_id: AccountId::from_protobuf(account_id)?,
            creation_time: OffsetDateTime::from(creation_time),
            metadata,
            // TODO ledger_id: Ledger_Id::from_protobuf(ledger_id),
            spender_id
        })
    }
}
