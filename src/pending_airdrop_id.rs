/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use std::fmt::{
    self,
    Debug,
    Display,
    Formatter,
};

use hedera_proto::services;

use crate::ledger_id::RefLedgerId;
use crate::{
    AccountId,
    Error,
    FromProtobuf,
    NftId,
    ToProtobuf,
    TokenId,
    ValidateChecksums,
};

/// A unique, composite, identifier for a pending airdrop.
///
/// Each pending airdrop SHALL be uniquely identified by a PendingAirdropId.
/// A PendingAirdropId SHALL be recorded when created and MUST be provided in any transaction
/// that would modify that pending airdrop (such as a `claimAirdrop` or `cancelAirdrop`).
///
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct PendingAirdropId {
    /// A sending account.
    ///
    /// This is the account that initiated, and SHALL fund, this pending airdrop.
    /// This field is REQUIRED.
    pub sender_id: AccountId,

    /// A receiving account.
    ///
    /// This is the ID of the account that SHALL receive the airdrop.
    /// This field is REQUIRED.
    pub receiver_id: AccountId,

    /// A token reference.
    pub token_reference: Option<TokenReference>,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum TokenReference {
    /// A token ID.
    FungibleTokenType(TokenId),

    /// The id of a single NFT, consisting of a Token ID and serial number.
    NonFungibleToken(NftId),
}

impl PendingAirdropId {
    pub const fn new(
        sender_id: AccountId,
        receiver_id: AccountId,
        token_reference: Option<TokenReference>,
    ) -> Self {
        Self { sender_id, receiver_id, token_reference }
    }

    /// Create a new `PendingAirdropId` from protobuf-encoded `bytes`.
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

impl Debug for TokenReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenReference::FungibleTokenType(token_id) => {
                f.debug_tuple("FungibleTokenType").field(token_id).finish()
            }
            TokenReference::NonFungibleToken(nft_id) => {
                f.debug_tuple("NonFungibleToken").field(nft_id).finish()
            }
        }
    }
}

impl Display for TokenReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenReference::FungibleTokenType(token_id) => write!(f, "FT:{}", token_id),
            TokenReference::NonFungibleToken(nft_id) => write!(f, "NFT:{}", nft_id),
        }
    }
}

impl Debug for PendingAirdropId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("PendingAirdropId")
            .field("sender_id", &self.sender_id)
            .field("receiver_id", &self.receiver_id)
            .field("token_reference", &self.token_reference)
            .finish()
    }
}

impl Display for PendingAirdropId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}->{}:{}",
            self.sender_id,
            self.receiver_id,
            match &self.token_reference {
                Some(TokenReference::FungibleTokenType(token_id)) => token_id.to_string(),
                Some(TokenReference::NonFungibleToken(nft_id)) => nft_id.to_string(),
                None => "No token reference".to_string(),
            }
        )
    }
}

impl ValidateChecksums for PendingAirdropId {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        let _ = self.sender_id.validate_checksums(ledger_id);
        let _ = self.receiver_id.validate_checksums(ledger_id);

        if let Some(token_reference) = &self.token_reference {
            match token_reference {
                TokenReference::FungibleTokenType(token_id) => {
                    token_id.validate_checksums(ledger_id)?
                }
                TokenReference::NonFungibleToken(nft_id) => nft_id.validate_checksums(ledger_id)?,
            }
        }

        Ok(())
    }
}

impl FromProtobuf<services::PendingAirdropId> for PendingAirdropId {
    fn from_protobuf(pb: services::PendingAirdropId) -> crate::Result<Self> {
        let sender_id = AccountId::from_protobuf(pb_getf!(pb, sender_id)?)?;
        let receiver_id = AccountId::from_protobuf(pb_getf!(pb, receiver_id)?)?;

        let token_reference = match pb.token_reference {
            Some(token_reference) => match token_reference {
                services::pending_airdrop_id::TokenReference::FungibleTokenType(token_id) => {
                    Some(TokenReference::FungibleTokenType(TokenId::from_protobuf(token_id)?))
                }
                services::pending_airdrop_id::TokenReference::NonFungibleToken(nft_id) => {
                    Some(TokenReference::NonFungibleToken(NftId::from_protobuf(nft_id)?))
                }
            },
            None => None,
        };

        Ok(Self { sender_id, receiver_id, token_reference })
    }
}

impl ToProtobuf for PendingAirdropId {
    type Protobuf = services::PendingAirdropId;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::PendingAirdropId {
            sender_id: Some(self.sender_id.to_protobuf()),
            receiver_id: Some(self.receiver_id.to_protobuf()),
            token_reference: match &self.token_reference {
                Some(TokenReference::FungibleTokenType(token_id)) => {
                    Some(services::pending_airdrop_id::TokenReference::FungibleTokenType(
                        token_id.to_protobuf(),
                    ))
                }
                Some(TokenReference::NonFungibleToken(nft_id)) => {
                    Some(services::pending_airdrop_id::TokenReference::NonFungibleToken(
                        nft_id.to_protobuf(),
                    ))
                }
                None => None,
            },
        }
    }
}
