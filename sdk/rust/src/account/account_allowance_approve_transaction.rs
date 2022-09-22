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

use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    Hbar,
    NftId,
    ToProtobuf,
    TokenId,
    Transaction,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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
        self.body.data.hbar_allowances.push(HbarAllowance {
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
        self.body.data.token_allowances.push(TokenAllowance {
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
        let owner_account_id = owner_account_id;
        let spender_account_id = spender_account_id;

        if let Some(allowance) = self.body.data.nft_allowances.iter_mut().find(|allowance| {
            allowance.token_id == nft_id.token_id
                && allowance.owner_account_id == owner_account_id
                && allowance.spender_account_id == spender_account_id
                && allowance.approved_for_all.is_none()
        }) {
            allowance.serial_numbers.push(nft_id.serial_number as i64);
        } else {
            self.body.data.nft_allowances.push(NftAllowance {
                serial_numbers: vec![nft_id.serial_number as i64],
                token_id: nft_id.token_id,
                spender_account_id,
                owner_account_id,
                delegating_spender_account_id: None,
                approved_for_all: None,
            });
        }

        self
    }

    /// Approve the NFT allowance on all serial numbers (present and future).
    pub fn approve_token_nft_allowance_all_serials(
        &mut self,
        token_id: TokenId,
        owner_account_id: AccountId,
        spender_account_id: AccountId,
    ) -> &mut Self {
        let owner_account_id = owner_account_id;
        let spender_account_id = spender_account_id;

        self.body.data.nft_allowances.push(NftAllowance {
            approved_for_all: Some(true),
            delegating_spender_account_id: None,
            spender_account_id,
            owner_account_id,
            token_id,
            serial_numbers: Vec::new(),
        });

        self
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct HbarAllowance {
    /// The account ID of the hbar owner (ie. the grantor of the allowance).
    owner_account_id: AccountId,

    /// The account ID of the spender of the hbar allowance.
    spender_account_id: AccountId,

    /// The amount of the spender's allowance.
    amount: Hbar,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct NftAllowance {
    /// The token that the allowance pertains to.
    token_id: TokenId,

    /// The account ID of the token owner (ie. the grantor of the allowance).
    owner_account_id: AccountId,

    /// The account ID of the spender of the token allowance.
    spender_account_id: AccountId,

    /// The list of serial numbers that the spender is permitted to transfer.
    serial_numbers: Vec<i64>,

    /// If true, the spender has access to all of the owner's NFT units of type tokenId (currently
    /// owned and any in the future).
    approved_for_all: Option<bool>,

    /// The account ID of the spender who is granted approvedForAll allowance and granting
    /// approval on an NFT serial to another spender.
    delegating_spender_account_id: Option<AccountId>,
}

#[async_trait]
impl TransactionExecute for AccountAllowanceApproveTransactionData {
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
        let crypto_allowances =
            self.hbar_allowances.iter().map(|allowance| allowance.to_protobuf()).collect();

        let token_allowances =
            self.token_allowances.iter().map(|allowance| allowance.to_protobuf()).collect();

        let nft_allowances =
            self.nft_allowances.iter().map(|allowance| allowance.to_protobuf()).collect();

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

impl ToProtobuf for NftAllowance {
    type Protobuf = services::NftAllowance;

    fn to_protobuf(&self) -> Self::Protobuf {
        Self::Protobuf {
            token_id: Some(self.token_id.to_protobuf()),
            owner: Some(self.owner_account_id.to_protobuf()),
            spender: Some(self.spender_account_id.to_protobuf()),
            serial_numbers: self.serial_numbers.clone(),
            approved_for_all: self.approved_for_all,
            delegating_spender: self
                .delegating_spender_account_id
                .as_ref()
                .map(|id| id.to_protobuf()),
        }
    }
}
