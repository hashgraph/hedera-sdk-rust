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

use std::collections::HashMap;

use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use tonic::transport::Channel;

use super::{
    NftId,
    TokenId,
    TokenNftTransfer,
};
use crate::ledger_id::RefLedgerId;
use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::transaction::{
    AnyTransactionData,
    ChunkInfo,
    ToSchedulableTransactionDataProtobuf,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::transfer_transaction::{
    TokenTransfer,
    Transfer,
};
use crate::{
    AccountId,
    BoxGrpcFuture,
    Error,
    Transaction,
    ValidateChecksums,
};

///
/// * Airdrop one or more tokens to one or more accounts.
//  *
//  * ### Effects
//  * This distributes tokens from the balance of one or more sending account(s) to the balance
//  * of one or more recipient accounts. Accounts MAY receive the tokens in one of four ways.
//  *
//  *  - An account already associated to the token to be distributed SHALL receive the
//  *    airdropped tokens immediately to the recipient account balance.<br/>
//  *    The fee for this transfer SHALL include the transfer, the airdrop fee, and any custom fees.
//  *  - An account with available automatic association slots SHALL be automatically
//  *    associated to the token, and SHALL immediately receive the airdropped tokens to the
//  *    recipient account balance.<br/>
//  *    The fee for this transfer SHALL include the transfer, the association, the cost to renew
//  *    that association once, the airdrop fee, and any custom fees.
//  *  - An account with "receiver signature required" set SHALL have a "Pending Airdrop" created
//  *    and must claim that airdrop with a `claimAirdrop` transaction.<br/>
//  *    The fee for this transfer SHALL include the transfer, the association, the cost to renew
//  *    that association once, the airdrop fee, and any custom fees. If the pending airdrop is not
//  *    claimed immediately, the `sender` SHALL pay the cost to renew the token association, and
//  *    the cost to maintain the pending airdrop, until the pending airdrop is claimed or cancelled.
//  *  - An account with no available automatic association slots SHALL have a "Pending Airdrop"
//  *    created and must claim that airdrop with a `claimAirdrop` transaction.<br/>
//  *    The fee for this transfer SHALL include the transfer, the association, the cost to renew
//  *    that association once, the airdrop fee, and any custom fees. If the pending airdrop is not
//  *    claimed immediately, the `sender` SHALL pay the cost to renew the token association, and
//  *    the cost to maintain the pending airdrop, until the pending airdrop is claimed or cancelled.
//  *
//  * If an airdrop would create a pending airdrop for a fungible/common token, and a pending airdrop
//  * for the same sender, receiver, and token already exists, the existing pending airdrop
//  * SHALL be updated to add the new amount to the existing airdrop, rather than creating a new
//  * pending airdrop.
//  *
//  * Any airdrop that completes immediately SHALL be irreversible. Any airdrop that results in a
//  * "Pending Airdrop" MAY be canceled via a `cancelAirdrop` transaction.
//  *
//  * All transfer fees (including custom fees and royalties), as well as the rent cost for the
//  * first auto-renewal period for any automatic-association slot occupied by the airdropped
//  * tokens, SHALL be charged to the account paying for this transaction.
//  *
//  * ### Record Stream Effects
//  * - Each successful transfer SHALL be recorded in `token_transfer_list` for the transaction record.
//  * - Each successful transfer that consumes an automatic association slot SHALL populate the
//  *   `automatic_association` field for the record.
//  * - Each pending transfer _created_ SHALL be added to the `pending_airdrops` field for the record.
//  * - Each pending transfer _updated_ SHALL be added to the `pending_airdrops` field for the record.
//  */
pub type TokenAirdropTransaction = Transaction<TokenAirdropTransactionData>;

#[derive(Debug, Clone, Default)]
pub struct TokenAirdropTransactionData {
    /// A list of token transfers representing one or more airdrops.
    token_transfers: Vec<TokenTransfer>,
}

impl TokenAirdropTransactionData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_token_transfer(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        amount: i64,
    ) -> &mut Self {
        self.do_add_token_transfer(token_id, account_id, amount, false, None)
    }

    pub fn get_token_transfers(&self) -> HashMap<TokenId, HashMap<AccountId, i64>> {
        self.token_transfers
            .iter()
            .map(|t| (t.token_id, t.transfers.iter().map(|t| (t.account_id, t.amount)).collect()))
            .collect()
    }

    /// Add a non-approved nft transfer.
    pub fn add_nft_transfer(
        &mut self,
        nft_id: NftId,
        sender: AccountId,
        receiver: AccountId,
    ) -> &mut Self {
        self.do_add_nft_transfer(nft_id, sender, receiver, false);
        self
    }

    /// Extract the of token nft transfers.
    pub fn get_nft_transfers(&self) -> HashMap<TokenId, Vec<TokenNftTransfer>> {
        self.token_transfers.iter().map(|t| (t.token_id, t.nft_transfers.clone())).collect()
    }

    /// Add a non-approved token transfer with decimals.
    pub fn add_token_transfer_with_decimals(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        amount: i64,
        decimals: u32,
    ) -> &mut Self {
        self.do_add_token_transfer(token_id, account_id, amount, false, Some(decimals));
        self
    }

    /// Extract the list of token id decimals.
    pub fn get_token_ids_with_decimals(&self) -> HashMap<TokenId, Option<u32>> {
        self.token_transfers
            .iter()
            .map(|t| (t.token_id, t.expected_decimals))
            .collect()
    }

    /// Add an approved token transfer to the transaction.
    pub fn add_approved_token_transfer(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        amount: i64,
    ) -> &mut Self {
        self.do_add_token_transfer(token_id, account_id, amount, true, None);
        self
    }

    /// Add an approved nft transfer.
    pub fn add_approved_nft_transfer(
        &mut self,
        nft_id: NftId,
        sender: AccountId,
        receiver: AccountId,
    ) -> &mut Self {
        self.do_add_nft_transfer(nft_id, sender, receiver, true);
        self
    }

    /// Add an approved token transfer with decimals.
    pub fn add_approved_token_transfer_with_decimals(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        amount: i64,
        decimals: u32,
    ) -> &mut Self {
        self.do_add_token_transfer(token_id, account_id, amount, true, Some(decimals));
        self
    }

    fn do_add_token_transfer(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        amount: i64,
        is_approved: bool,
        decimals: Option<u32>,
    ) -> &mut Self {
        self.token_transfers.push(TokenTransfer {
            token_id: token_id,
            transfers: vec![Transfer { account_id: account_id, amount, is_approval: is_approved }],
            nft_transfers: vec![],
            expected_decimals: decimals,
        });
        self
    }

    fn do_add_nft_transfer(
        &mut self,
        nft_id: NftId,
        sender: AccountId,
        receiver: AccountId,
        is_approved: bool,
    ) -> &mut Self {
        self.token_transfers.push(TokenTransfer {
            token_id: nft_id.token_id,
            transfers: vec![],
            nft_transfers: vec![TokenNftTransfer {
                token_id: TokenId {
                    shard: nft_id.token_id.shard,
                    realm: nft_id.token_id.realm,
                    num: nft_id.token_id.num,
                    checksum: nft_id.token_id.checksum,
                },
                sender,
                receiver,
                serial: nft_id.serial,
                is_approved,
            }],
            expected_decimals: None,
        });
        self
    }
}

impl TransactionData for TokenAirdropTransactionData {}

impl TransactionExecute for TokenAirdropTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { TokenServiceClient::new(channel).airdrop_tokens(request).await })
    }
}

impl ValidateChecksums for TokenAirdropTransactionData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        for token_transfer in &self.token_transfers {
            token_transfer.token_id.validate_checksums(ledger_id)?;
            for transfer in &token_transfer.transfers {
                transfer.account_id.validate_checksums(ledger_id)?;
            }
            for nft_transfer in &token_transfer.nft_transfers {
                nft_transfer.sender.validate_checksums(ledger_id)?;
                nft_transfer.receiver.validate_checksums(ledger_id)?;
            }
        }
        Ok(())
    }
}

impl ToTransactionDataProtobuf for TokenAirdropTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::TokenAirdrop(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for TokenAirdropTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::TokenAirdrop(self.to_protobuf())
    }
}

impl From<TokenAirdropTransactionData> for AnyTransactionData {
    fn from(transaction: TokenAirdropTransactionData) -> Self {
        Self::TokenAirdrop(transaction)
    }
}

impl ToProtobuf for TokenAirdropTransactionData {
    type Protobuf = services::TokenAirdropTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::TokenAirdropTransactionBody {
            token_transfers: self.token_transfers.iter().map(|t| t.to_protobuf()).collect(),
        }
    }
}

impl FromProtobuf<services::TokenAirdropTransactionBody> for TokenAirdropTransactionData {
    fn from_protobuf(pb: services::TokenAirdropTransactionBody) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            token_transfers: pb
                .token_transfers
                .into_iter()
                .map(|t| TokenTransfer::from_protobuf(t))
                .collect::<crate::Result<_>>()?,
        })
    }
}
