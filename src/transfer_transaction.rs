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
use std::ops::Not;

use hedera_proto::services;
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

use crate::ledger_id::RefLedgerId;
use crate::protobuf::FromProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ChunkInfo,
    ToSchedulableTransactionDataProtobuf,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::{
    AccountId,
    BoxGrpcFuture,
    Error,
    Hbar,
    NftId,
    ToProtobuf,
    TokenId,
    TokenNftTransfer,
    Transaction,
    ValidateChecksums,
};

/// Transfers cryptocurrency among two or more accounts by making the desired adjustments to their
/// balances.
///
/// Each transfer list can specify up to 10 adjustments. Each negative amount is withdrawn
/// from the corresponding account (a sender), and each positive one is added to the corresponding
/// account (a receiver). The amounts list must sum to zero.
///
/// All transfers are in the lowest denomination, for `Hbar` that is tinybars (although `Hbar` handles this itself).
///
/// As an example: for a fungible token with `3` decimals (and let's say the symbol is `ƒ`), transferring `1` _always_ transfers `0.001 ƒ`.
pub type TransferTransaction = Transaction<TransferTransactionData>;

#[derive(Debug, Clone, Default)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct TransferTransactionData {
    transfers: Vec<Transfer>,
    token_transfers: Vec<TokenTransfer>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct Transfer {
    /// The account involved in the transfer.
    account_id: AccountId,

    /// The value of the transfer.
    amount: i64,

    /// If this is an approved transfer.
    is_approval: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct TokenTransfer {
    token_id: TokenId,

    transfers: Vec<Transfer>,

    nft_transfers: Vec<TokenNftTransfer>,

    expected_decimals: Option<u32>,
}

impl TransferTransaction {
    fn _hbar_transfer(&mut self, account_id: AccountId, amount: Hbar, approved: bool) -> &mut Self {
        self.data_mut().transfers.push(Transfer {
            account_id,
            amount: amount.to_tinybars(),
            is_approval: approved,
        });

        self
    }

    /// Add a non-approved hbar transfer to the transaction.
    pub fn hbar_transfer(&mut self, account_id: AccountId, amount: Hbar) -> &mut Self {
        self._hbar_transfer(account_id, amount, false)
    }

    /// Add an approved hbar transfer to the transaction.
    pub fn approved_hbar_transfer(&mut self, account_id: AccountId, amount: Hbar) -> &mut Self {
        self._hbar_transfer(account_id, amount, true)
    }

    /// Returns all transfers associated with this transaction.
    pub fn get_hbar_transfers(&self) -> HashMap<AccountId, Hbar> {
        self.data()
            .transfers
            .iter()
            .map(|it| (it.account_id, Hbar::from_tinybars(it.amount)))
            .collect()
    }

    fn _token_transfer(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        amount: i64,
        approved: bool,
        expected_decimals: Option<u32>,
    ) -> &mut Self {
        let transfer = Transfer { account_id, amount, is_approval: approved };
        let data = self.data_mut();

        if let Some(tt) = data.token_transfers.iter_mut().find(|tt| tt.token_id == token_id) {
            tt.expected_decimals = expected_decimals;
            tt.transfers.push(transfer);
        } else {
            data.token_transfers.push(TokenTransfer {
                token_id,
                expected_decimals,
                nft_transfers: Vec::new(),
                transfers: vec![transfer],
            });
        }

        self
    }

    /// Add a non-approved token transfer to the transaction.
    ///
    /// `amount` is in the lowest denomination for the token (if the token has `2` decimals this would be `0.01` tokens).
    pub fn token_transfer(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        amount: i64,
    ) -> &mut Self {
        self._token_transfer(token_id, account_id, amount, false, None)
    }

    /// Add an approved token transfer to the transaction.
    ///
    /// `amount` is in the lowest denomination for the token (if the token has `2` decimals this would be `0.01` tokens).
    pub fn approved_token_transfer(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        amount: i64,
    ) -> &mut Self {
        self._token_transfer(token_id, account_id, amount, true, None)
    }

    // todo: make the examples into code, just not sure how to do that.
    /// Add a non-approved token transfer to the transaction, ensuring that the token has `expected_decimals` decimals.
    ///
    /// `amount` is _still_ in the lowest denomination, however,
    /// you will get an error if the token has a different amount of decimals than `expected_decimals`.
    pub fn token_transfer_with_decimals(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        amount: i64,
        expected_decimals: u32,
    ) -> &mut Self {
        self._token_transfer(token_id, account_id, amount, false, Some(expected_decimals))
    }

    /// Add an approved token transfer, ensuring that the token has `expected_decimals` decimals.
    ///
    /// `amount` is _still_ in the lowest denomination, however,
    /// you will get an error if the token has a different amount of decimals than `expected_decimals`.
    pub fn approved_token_transfer_with_decimals(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        amount: i64,
        expected_decimals: u32,
    ) -> &mut Self {
        self._token_transfer(token_id, account_id, amount, true, Some(expected_decimals))
    }

    /// Returns all the token transfers associated associated with this transaction.
    pub fn get_token_transfers(&self) -> HashMap<TokenId, HashMap<AccountId, i64>> {
        use std::collections::hash_map::Entry;

        // note: using fold instead of nested collects on the off chance a token is in here twice.
        self.data().token_transfers.iter().fold(
            HashMap::with_capacity(self.data().token_transfers.len()),
            |mut map, transfer| {
                let iter = transfer.transfers.iter().map(|it| (it.account_id, it.amount));
                match map.entry(transfer.token_id) {
                    Entry::Occupied(mut it) => it.get_mut().extend(iter),
                    Entry::Vacant(it) => {
                        it.insert(iter.collect());
                    }
                }

                map
            },
        )
    }

    fn _nft_transfer(
        &mut self,
        nft_id: NftId,
        sender_account_id: AccountId,
        receiver_account_id: AccountId,
        approved: bool,
    ) -> &mut Self {
        let NftId { token_id, serial } = nft_id;
        let transfer = TokenNftTransfer {
            token_id,
            serial,
            sender: sender_account_id,
            receiver: receiver_account_id,
            is_approved: approved,
        };

        let data = self.data_mut();

        if let Some(tt) = data.token_transfers.iter_mut().find(|tt| tt.token_id == token_id) {
            tt.nft_transfers.push(transfer);
        } else {
            data.token_transfers.push(TokenTransfer {
                token_id,
                expected_decimals: None,
                transfers: Vec::new(),
                nft_transfers: vec![transfer],
            });
        }

        self
    }

    /// Add an approved nft transfer to the transaction.
    pub fn approved_nft_transfer(
        &mut self,
        nft_id: impl Into<NftId>,
        sender_account_id: AccountId,
        receiver_account_id: AccountId,
    ) -> &mut Self {
        self._nft_transfer(nft_id.into(), sender_account_id, receiver_account_id, true)
    }

    /// Add a non-approved nft transfer to the transaction.
    pub fn nft_transfer(
        &mut self,
        nft_id: impl Into<NftId>,
        sender_account_id: AccountId,
        receiver_account_id: AccountId,
    ) -> &mut Self {
        self._nft_transfer(nft_id.into(), sender_account_id, receiver_account_id, false)
    }

    /// Returns all the NFT transfers associated with this transaction.
    pub fn get_nft_transfers(&self) -> HashMap<TokenId, Vec<TokenNftTransfer>> {
        self.data()
            .token_transfers
            .iter()
            .map(|it| (it.token_id, it.nft_transfers.clone()))
            .collect()
    }
}

impl TransactionExecute for TransferTransactionData {
    // noinspection DuplicatedCode
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { CryptoServiceClient::new(channel).crypto_transfer(request).await })
    }
}

impl TransactionData for TransferTransactionData {}

impl ValidateChecksums for TransferTransactionData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        for transfer in &self.transfers {
            transfer.account_id.validate_checksums(ledger_id)?;
        }
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

impl FromProtobuf<services::AccountAmount> for Transfer {
    fn from_protobuf(pb: services::AccountAmount) -> crate::Result<Self> {
        Ok(Self {
            amount: pb.amount,
            account_id: AccountId::from_protobuf(pb_getf!(pb, account_id)?)?,
            is_approval: pb.is_approval,
        })
    }
}

impl ToProtobuf for Transfer {
    type Protobuf = services::AccountAmount;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::AccountAmount {
            amount: self.amount,
            account_id: Some(self.account_id.to_protobuf()),
            is_approval: self.is_approval,
        }
    }
}

impl FromProtobuf<services::TokenTransferList> for TokenTransfer {
    fn from_protobuf(pb: services::TokenTransferList) -> crate::Result<Self> {
        let token_id = TokenId::from_protobuf(pb_getf!(pb, token)?)?;

        Ok(Self {
            token_id,
            transfers: Vec::from_protobuf(pb.transfers)?,
            nft_transfers: pb
                .nft_transfers
                .into_iter()
                .map(|pb| TokenNftTransfer::from_protobuf(pb, token_id))
                .collect::<Result<Vec<_>, _>>()?,
            expected_decimals: pb.expected_decimals,
        })
    }
}

impl ToProtobuf for TokenTransfer {
    type Protobuf = services::TokenTransferList;

    fn to_protobuf(&self) -> Self::Protobuf {
        let transfers = self.transfers.to_protobuf();
        let nft_transfers = self.nft_transfers.to_protobuf();

        services::TokenTransferList {
            token: Some(self.token_id.to_protobuf()),
            transfers,
            nft_transfers,
            expected_decimals: self.expected_decimals,
        }
    }
}

impl ToProtobuf for TokenNftTransfer {
    type Protobuf = services::NftTransfer;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::NftTransfer {
            sender_account_id: Some(self.sender.to_protobuf()),
            receiver_account_id: Some(self.receiver.to_protobuf()),
            serial_number: self.serial as i64,
            is_approval: self.is_approved,
        }
    }
}

impl ToTransactionDataProtobuf for TransferTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::CryptoTransfer(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for TransferTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::CryptoTransfer(self.to_protobuf())
    }
}

impl From<TransferTransactionData> for AnyTransactionData {
    fn from(transaction: TransferTransactionData) -> Self {
        Self::Transfer(transaction)
    }
}

impl FromProtobuf<services::CryptoTransferTransactionBody> for TransferTransactionData {
    fn from_protobuf(pb: services::CryptoTransferTransactionBody) -> crate::Result<Self> {
        let transfers = pb.transfers.map(|it| it.account_amounts);
        let transfers = Option::from_protobuf(transfers)?.unwrap_or_default();

        Ok(Self { transfers, token_transfers: Vec::from_protobuf(pb.token_transfers)? })
    }
}

impl ToProtobuf for TransferTransactionData {
    type Protobuf = services::CryptoTransferTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        let transfers = self
            .transfers
            .is_empty()
            .not()
            .then(|| services::TransferList { account_amounts: self.transfers.to_protobuf() });

        let token_transfers = self.token_transfers.to_protobuf();

        services::CryptoTransferTransactionBody { transfers, token_transfers }
    }
}
