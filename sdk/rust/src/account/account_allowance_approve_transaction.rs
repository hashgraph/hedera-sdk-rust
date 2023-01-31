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

use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

use crate::protobuf::FromProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    Error,
    Hbar,
    LedgerId,
    NftId,
    ToProtobuf,
    TokenId,
    Transaction,
    ValidateChecksums,
};

/// Creates one or more hbar/token approved allowances **relative to the owner account specified in the allowances of
/// this transaction**.
///
/// Each allowance grants a spender the right to transfer a pre-determined amount of the owner's
/// hbar/token to any other account of the spender's choice. If the owner is not specified in any
/// allowance, the payer of transaction is considered to be the owner for that particular allowance.
///
/// Setting the amount to zero will remove the respective allowance for the spender.
///
pub type AccountAllowanceApproveTransaction = Transaction<AccountAllowanceApproveTransactionData>;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", default))]
pub struct AccountAllowanceApproveTransactionData {
    /// List of hbar allowances approved by the account owner.
    hbar_allowances: Vec<HbarAllowance>,

    /// List of fungible token allowances approved by the account owner.
    token_allowances: Vec<TokenAllowance>,

    /// List of non-fungible token allowances approved by the account owner.
    nft_allowances: Vec<NftAllowance>,
}

impl AccountAllowanceApproveTransaction {
    /// Approves the hbar allowance.
    pub fn approve_hbar_allowance(
        &mut self,
        owner_account_id: AccountId,
        spender_account_id: AccountId,
        amount: Hbar,
    ) -> &mut Self {
        self.data_mut().hbar_allowances.push(HbarAllowance {
            owner_account_id,
            spender_account_id,
            amount,
        });

        self
    }

    /// Approves the token allowance.
    pub fn approve_token_allowance(
        &mut self,
        token_id: TokenId,
        owner_account_id: AccountId,
        spender_account_id: AccountId,
        amount: u64,
    ) -> &mut Self {
        self.data_mut().token_allowances.push(TokenAllowance {
            token_id,
            owner_account_id,
            spender_account_id,
            amount,
        });
        self
    }

    /// Approve the NFT allowance.
    pub fn approve_token_nft_allowance(
        &mut self,
        nft_id: impl Into<NftId>,
        owner_account_id: AccountId,
        spender_account_id: AccountId,
    ) -> &mut Self {
        let nft_id = nft_id.into();
        let data = self.data_mut();

        if let Some(allowance) = data.nft_allowances.iter_mut().find(|allowance| {
            allowance.token_id == nft_id.token_id
                && allowance.owner_account_id == owner_account_id
                && allowance.spender_account_id == spender_account_id
                && allowance.approved_for_all.is_none()
        }) {
            allowance.serials.push(nft_id.serial as i64);
        } else {
            data.nft_allowances.push(NftAllowance {
                serials: vec![nft_id.serial as i64],
                token_id: nft_id.token_id,
                spender_account_id,
                owner_account_id,
                delegating_spender_account_id: None,
                approved_for_all: None,
            });
        };

        self
    }

    /// Approve the NFT allowance on all serial numbers (present and future).
    pub fn approve_token_nft_allowance_all_serials(
        &mut self,
        token_id: TokenId,
        owner_account_id: AccountId,
        spender_account_id: AccountId,
    ) -> &mut Self {
        self.data_mut().nft_allowances.push(NftAllowance {
            approved_for_all: Some(true),
            delegating_spender_account_id: None,
            spender_account_id,
            owner_account_id,
            token_id,
            serials: Vec::new(),
        });

        self
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
struct HbarAllowance {
    /// The account ID of the hbar owner (ie. the grantor of the allowance).
    owner_account_id: AccountId,

    /// The account ID of the spender of the hbar allowance.
    spender_account_id: AccountId,

    /// The amount of the spender's allowance.
    amount: Hbar,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
struct TokenAllowance {
    /// The token that the allowance pertains to.
    token_id: TokenId,

    /// The account ID of the token owner (ie. the grantor of the allowance).
    owner_account_id: AccountId,

    /// The account ID of the spender of the token allowance.
    spender_account_id: AccountId,

    /// The amount of the spender's token allowance.
    amount: u64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
struct NftAllowance {
    /// The token that the allowance pertains to.
    token_id: TokenId,

    /// The account ID of the token owner (ie. the grantor of the allowance).
    owner_account_id: AccountId,

    /// The account ID of the spender of the token allowance.
    spender_account_id: AccountId,

    /// The list of serial numbers that the spender is permitted to transfer.
    serials: Vec<i64>,

    /// If true, the spender has access to all of the owner's NFT units of type tokenId (currently
    /// owned and any in the future).
    approved_for_all: Option<bool>,

    /// The account ID of the spender who is granted approvedForAll allowance and granting
    /// approval on an NFT serial to another spender.
    delegating_spender_account_id: Option<AccountId>,
}

#[async_trait]
impl TransactionExecute for AccountAllowanceApproveTransactionData {
    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        for hbar_allowance in &self.hbar_allowances {
            hbar_allowance.owner_account_id.validate_checksums_for_ledger_id(ledger_id)?;
            hbar_allowance.spender_account_id.validate_checksums_for_ledger_id(ledger_id)?;
        }
        for token_allowance in &self.token_allowances {
            token_allowance.token_id.validate_checksums_for_ledger_id(ledger_id)?;
            token_allowance.owner_account_id.validate_checksums_for_ledger_id(ledger_id)?;
            token_allowance.spender_account_id.validate_checksums_for_ledger_id(ledger_id)?;
        }
        for nft_allowance in &self.nft_allowances {
            nft_allowance.token_id.validate_checksums_for_ledger_id(ledger_id)?;
            nft_allowance.spender_account_id.validate_checksums_for_ledger_id(ledger_id)?;
            nft_allowance.owner_account_id.validate_checksums_for_ledger_id(ledger_id)?;
            if let Some(delegating_spender) = nft_allowance.delegating_spender_account_id {
                delegating_spender.validate_checksums_for_ledger_id(ledger_id)?;
            }
        }
        Ok(())
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        CryptoServiceClient::new(channel).approve_allowances(request).await
    }
}

impl ToTransactionDataProtobuf for AccountAllowanceApproveTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let crypto_allowances = self.hbar_allowances.to_protobuf();

        let token_allowances = self.token_allowances.to_protobuf();

        let nft_allowances = self.nft_allowances.to_protobuf();

        services::transaction_body::Data::CryptoApproveAllowance(
            services::CryptoApproveAllowanceTransactionBody {
                crypto_allowances,
                nft_allowances,
                token_allowances,
            },
        )
    }
}

impl From<AccountAllowanceApproveTransactionData> for AnyTransactionData {
    fn from(transaction: AccountAllowanceApproveTransactionData) -> Self {
        Self::AccountAllowanceApprove(transaction)
    }
}

impl FromProtobuf<services::CryptoApproveAllowanceTransactionBody>
    for AccountAllowanceApproveTransactionData
{
    fn from_protobuf(pb: services::CryptoApproveAllowanceTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            hbar_allowances: Vec::from_protobuf(pb.crypto_allowances)?,
            token_allowances: Vec::from_protobuf(pb.token_allowances)?,
            nft_allowances: Vec::from_protobuf(pb.nft_allowances)?,
        })
    }
}

impl FromProtobuf<services::CryptoAllowance> for HbarAllowance {
    fn from_protobuf(pb: services::CryptoAllowance) -> crate::Result<Self> {
        Ok(Self {
            owner_account_id: AccountId::from_protobuf(pb_getf!(pb, owner)?)?,
            spender_account_id: AccountId::from_protobuf(pb_getf!(pb, spender)?)?,
            amount: Hbar::from_tinybars(pb.amount),
        })
    }
}

impl ToProtobuf for HbarAllowance {
    type Protobuf = services::CryptoAllowance;

    fn to_protobuf(&self) -> Self::Protobuf {
        Self::Protobuf {
            owner: Some(self.owner_account_id.to_protobuf()),
            spender: Some(self.spender_account_id.to_protobuf()),
            amount: self.amount.to_tinybars(),
        }
    }
}

impl FromProtobuf<services::TokenAllowance> for TokenAllowance {
    fn from_protobuf(pb: services::TokenAllowance) -> crate::Result<Self> {
        Ok(Self {
            token_id: TokenId::from_protobuf(pb_getf!(pb, token_id)?)?,
            owner_account_id: AccountId::from_protobuf(pb_getf!(pb, owner)?)?,
            spender_account_id: AccountId::from_protobuf(pb_getf!(pb, spender)?)?,
            amount: pb.amount as u64,
        })
    }
}

impl ToProtobuf for TokenAllowance {
    type Protobuf = services::TokenAllowance;

    fn to_protobuf(&self) -> Self::Protobuf {
        Self::Protobuf {
            token_id: Some(self.token_id.to_protobuf()),
            owner: Some(self.owner_account_id.to_protobuf()),
            spender: Some(self.spender_account_id.to_protobuf()),
            amount: self.amount as i64,
        }
    }
}

impl FromProtobuf<services::NftAllowance> for NftAllowance {
    fn from_protobuf(pb: services::NftAllowance) -> crate::Result<Self> {
        Ok(Self {
            token_id: TokenId::from_protobuf(pb_getf!(pb, token_id)?)?,
            owner_account_id: AccountId::from_protobuf(pb_getf!(pb, owner)?)?,
            spender_account_id: AccountId::from_protobuf(pb_getf!(pb, spender)?)?,
            serials: pb.serial_numbers,
            approved_for_all: pb.approved_for_all,
            delegating_spender_account_id: Option::from_protobuf(pb.delegating_spender)?,
        })
    }
}

impl ToProtobuf for NftAllowance {
    type Protobuf = services::NftAllowance;

    fn to_protobuf(&self) -> Self::Protobuf {
        Self::Protobuf {
            token_id: Some(self.token_id.to_protobuf()),
            owner: Some(self.owner_account_id.to_protobuf()),
            spender: Some(self.spender_account_id.to_protobuf()),
            serial_numbers: self.serials.clone(),
            approved_for_all: self.approved_for_all,
            delegating_spender: self.delegating_spender_account_id.to_protobuf(),
        }
    }
}
