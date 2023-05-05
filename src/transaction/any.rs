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

use hedera_proto::services;
use tonic::transport::Channel;

use super::chunked::ChunkInfo;
use super::{
    services_data,
    TransactionData,
    TransactionExecuteChunked,
};
use crate::downcast::DowncastOwned;
use crate::entity_id::ValidateChecksums;
use crate::protobuf::FromProtobuf;
use crate::transaction::{
    ToTransactionDataProtobuf,
    TransactionBody,
    TransactionExecute,
};
use crate::{
    BoxGrpcFuture,
    Error,
    Hbar,
    LedgerId,
    Transaction,
    TransactionId,
};

mod data {
    pub(super) use crate::account::{
        AccountAllowanceApproveTransactionData as AccountAllowanceApprove,
        AccountAllowanceDeleteTransactionData as AccountAllowanceDelete,
        AccountCreateTransactionData as AccountCreate,
        AccountDeleteTransactionData as AccountDelete,
        AccountUpdateTransactionData as AccountUpdate,
    };
    pub(super) use crate::contract::{
        ContractCreateTransactionData as ContractCreate,
        ContractDeleteTransactionData as ContractDelete,
        ContractExecuteTransactionData as ContractExecute,
        ContractUpdateTransactionData as ContractUpdate,
    };
    pub(super) use crate::ethereum::EthereumTransactionData as Ethereum;
    pub(super) use crate::file::{
        FileAppendTransactionData as FileAppend,
        FileCreateTransactionData as FileCreate,
        FileDeleteTransactionData as FileDelete,
        FileUpdateTransactionData as FileUpdate,
    };
    pub(super) use crate::prng_transaction::PrngTransactionData as Prng;
    pub(super) use crate::schedule::{
        ScheduleCreateTransactionData as ScheduleCreate,
        ScheduleDeleteTransactionData as ScheduleDelete,
        ScheduleSignTransactionData as ScheduleSign,
    };
    pub(super) use crate::system::{
        FreezeTransactionData as Freeze,
        SystemDeleteTransactionData as SystemDelete,
        SystemUndeleteTransactionData as SystemUndelete,
    };
    pub(super) use crate::token::{
        TokenAssociateTransactionData as TokenAssociate,
        TokenBurnTransactionData as TokenBurn,
        TokenCreateTransactionData as TokenCreate,
        TokenDeleteTransactionData as TokenDelete,
        TokenDissociateTransactionData as TokenDissociate,
        TokenFeeScheduleUpdateTransactionData as TokenFeeScheduleUpdate,
        TokenFreezeTransactionData as TokenFreeze,
        TokenGrantKycTransactionData as TokenGrantKyc,
        TokenMintTransactionData as TokenMint,
        TokenPauseTransactionData as TokenPause,
        TokenRevokeKycTransactionData as TokenRevokeKyc,
        TokenUnfreezeTransactionData as TokenUnfreeze,
        TokenUnpauseTransactionData as TokenUnpause,
        TokenUpdateTransactionData as TokenUpdate,
        TokenWipeTransactionData as TokenWipe,
    };
    pub(super) use crate::topic::{
        TopicCreateTransactionData as TopicCreate,
        TopicDeleteTransactionData as TopicDelete,
        TopicMessageSubmitTransactionData as TopicMessageSubmit,
        TopicUpdateTransactionData as TopicUpdate,
    };
    pub(super) use crate::transfer_transaction::TransferTransactionData as Transfer;
}

/// Any possible transaction that may be executed on the Hedera network.
pub type AnyTransaction = Transaction<AnyTransactionData>;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum AnyTransactionData {
    AccountCreate(data::AccountCreate),
    AccountUpdate(data::AccountUpdate),
    AccountDelete(data::AccountDelete),
    AccountAllowanceApprove(data::AccountAllowanceApprove),
    AccountAllowanceDelete(data::AccountAllowanceDelete),
    ContractCreate(data::ContractCreate),
    ContractUpdate(data::ContractUpdate),
    ContractDelete(data::ContractDelete),
    ContractExecute(data::ContractExecute),
    Transfer(data::Transfer),
    TopicCreate(data::TopicCreate),
    TopicUpdate(data::TopicUpdate),
    TopicDelete(data::TopicDelete),
    TopicMessageSubmit(data::TopicMessageSubmit),
    FileAppend(data::FileAppend),
    FileCreate(data::FileCreate),
    FileUpdate(data::FileUpdate),
    FileDelete(data::FileDelete),
    Prng(data::Prng),
    ScheduleCreate(data::ScheduleCreate),
    ScheduleSign(data::ScheduleSign),
    ScheduleDelete(data::ScheduleDelete),
    TokenAssociate(data::TokenAssociate),
    TokenBurn(data::TokenBurn),
    TokenCreate(data::TokenCreate),
    TokenDelete(data::TokenDelete),
    TokenDissociate(data::TokenDissociate),
    TokenFeeScheduleUpdate(data::TokenFeeScheduleUpdate),
    TokenFreeze(data::TokenFreeze),
    TokenGrantKyc(data::TokenGrantKyc),
    TokenMint(data::TokenMint),
    TokenPause(data::TokenPause),
    TokenRevokeKyc(data::TokenRevokeKyc),
    TokenUnfreeze(data::TokenUnfreeze),
    TokenUnpause(data::TokenUnpause),
    TokenUpdate(data::TokenUpdate),
    TokenWipe(data::TokenWipe),
    SystemDelete(data::SystemDelete),
    SystemUndelete(data::SystemUndelete),
    Freeze(data::Freeze),
    Ethereum(data::Ethereum),
}

impl ToTransactionDataProtobuf for AnyTransactionData {
    // not really anything I can do about this
    #[allow(clippy::too_many_lines)]
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        match self {
            Self::Transfer(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::AccountCreate(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::AccountUpdate(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::AccountDelete(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::AccountAllowanceApprove(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::AccountAllowanceDelete(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::ContractCreate(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::ContractUpdate(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::ContractDelete(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::ContractExecute(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::FileAppend(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::FileCreate(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::FileUpdate(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::FileDelete(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::Prng(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::TokenAssociate(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::TokenBurn(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::TokenCreate(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::TokenDelete(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::TokenDissociate(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::TokenFeeScheduleUpdate(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::TokenFreeze(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::TokenGrantKyc(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::TokenMint(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::TokenPause(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::TokenRevokeKyc(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::TokenUnfreeze(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::TokenUnpause(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::TokenUpdate(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::TokenWipe(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::TopicCreate(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::TopicUpdate(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::TopicDelete(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::TopicMessageSubmit(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::SystemDelete(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::SystemUndelete(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::Freeze(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::ScheduleCreate(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::ScheduleSign(transaction) => transaction.to_transaction_data_protobuf(chunk_info),

            Self::ScheduleDelete(transaction) => {
                transaction.to_transaction_data_protobuf(chunk_info)
            }

            Self::Ethereum(transaction) => transaction.to_transaction_data_protobuf(chunk_info),
        }
    }
}

impl TransactionData for AnyTransactionData {
    fn default_max_transaction_fee(&self) -> Hbar {
        match self {
            Self::Transfer(transaction) => transaction.default_max_transaction_fee(),
            Self::AccountCreate(transaction) => transaction.default_max_transaction_fee(),
            Self::AccountUpdate(transaction) => transaction.default_max_transaction_fee(),
            Self::AccountDelete(transaction) => transaction.default_max_transaction_fee(),
            Self::AccountAllowanceApprove(transaction) => transaction.default_max_transaction_fee(),
            Self::AccountAllowanceDelete(transaction) => transaction.default_max_transaction_fee(),
            Self::ContractCreate(transaction) => transaction.default_max_transaction_fee(),
            Self::ContractUpdate(transaction) => transaction.default_max_transaction_fee(),
            Self::ContractDelete(transaction) => transaction.default_max_transaction_fee(),
            Self::ContractExecute(transaction) => transaction.default_max_transaction_fee(),
            Self::FileAppend(transaction) => transaction.default_max_transaction_fee(),
            Self::FileCreate(transaction) => transaction.default_max_transaction_fee(),
            Self::FileUpdate(transaction) => transaction.default_max_transaction_fee(),
            Self::FileDelete(transaction) => transaction.default_max_transaction_fee(),
            Self::Prng(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenAssociate(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenBurn(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenCreate(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenDelete(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenDissociate(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenFeeScheduleUpdate(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenFreeze(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenGrantKyc(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenMint(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenPause(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenRevokeKyc(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenUnfreeze(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenUnpause(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenUpdate(transaction) => transaction.default_max_transaction_fee(),
            Self::TokenWipe(transaction) => transaction.default_max_transaction_fee(),
            Self::TopicCreate(transaction) => transaction.default_max_transaction_fee(),
            Self::TopicUpdate(transaction) => transaction.default_max_transaction_fee(),
            Self::TopicDelete(transaction) => transaction.default_max_transaction_fee(),
            Self::TopicMessageSubmit(transaction) => transaction.default_max_transaction_fee(),
            Self::SystemDelete(transaction) => transaction.default_max_transaction_fee(),
            Self::SystemUndelete(transaction) => transaction.default_max_transaction_fee(),
            Self::Freeze(transaction) => transaction.default_max_transaction_fee(),
            Self::ScheduleCreate(transaction) => transaction.default_max_transaction_fee(),
            Self::ScheduleSign(transaction) => transaction.default_max_transaction_fee(),
            Self::ScheduleDelete(transaction) => transaction.default_max_transaction_fee(),
            Self::Ethereum(transaction) => transaction.default_max_transaction_fee(),
        }
    }

    fn maybe_chunk_data(&self) -> Option<&super::ChunkData> {
        match self {
            Self::AccountCreate(it) => it.maybe_chunk_data(),
            Self::AccountUpdate(it) => it.maybe_chunk_data(),
            Self::AccountDelete(it) => it.maybe_chunk_data(),
            Self::AccountAllowanceApprove(it) => it.maybe_chunk_data(),
            Self::AccountAllowanceDelete(it) => it.maybe_chunk_data(),
            Self::ContractCreate(it) => it.maybe_chunk_data(),
            Self::ContractUpdate(it) => it.maybe_chunk_data(),
            Self::ContractDelete(it) => it.maybe_chunk_data(),
            Self::ContractExecute(it) => it.maybe_chunk_data(),
            Self::Transfer(it) => it.maybe_chunk_data(),
            Self::TopicCreate(it) => it.maybe_chunk_data(),
            Self::TopicUpdate(it) => it.maybe_chunk_data(),
            Self::TopicDelete(it) => it.maybe_chunk_data(),
            Self::TopicMessageSubmit(it) => it.maybe_chunk_data(),
            Self::FileAppend(it) => it.maybe_chunk_data(),
            Self::FileCreate(it) => it.maybe_chunk_data(),
            Self::FileUpdate(it) => it.maybe_chunk_data(),
            Self::FileDelete(it) => it.maybe_chunk_data(),
            Self::Prng(it) => it.maybe_chunk_data(),
            Self::TokenAssociate(it) => it.maybe_chunk_data(),
            Self::TokenBurn(it) => it.maybe_chunk_data(),
            Self::TokenCreate(it) => it.maybe_chunk_data(),
            Self::TokenDelete(it) => it.maybe_chunk_data(),
            Self::TokenDissociate(it) => it.maybe_chunk_data(),
            Self::TokenFeeScheduleUpdate(it) => it.maybe_chunk_data(),
            Self::TokenFreeze(it) => it.maybe_chunk_data(),
            Self::TokenGrantKyc(it) => it.maybe_chunk_data(),
            Self::TokenMint(it) => it.maybe_chunk_data(),
            Self::TokenPause(it) => it.maybe_chunk_data(),
            Self::TokenRevokeKyc(it) => it.maybe_chunk_data(),
            Self::TokenUnfreeze(it) => it.maybe_chunk_data(),
            Self::TokenUnpause(it) => it.maybe_chunk_data(),
            Self::TokenUpdate(it) => it.maybe_chunk_data(),
            Self::TokenWipe(it) => it.maybe_chunk_data(),
            Self::SystemDelete(it) => it.maybe_chunk_data(),
            Self::SystemUndelete(it) => it.maybe_chunk_data(),
            Self::Freeze(it) => it.maybe_chunk_data(),
            Self::ScheduleCreate(it) => it.maybe_chunk_data(),
            Self::ScheduleSign(it) => it.maybe_chunk_data(),
            Self::ScheduleDelete(it) => it.maybe_chunk_data(),
            Self::Ethereum(it) => it.maybe_chunk_data(),
        }
    }

    fn wait_for_receipt(&self) -> bool {
        match self {
            Self::AccountCreate(it) => it.wait_for_receipt(),
            Self::AccountUpdate(it) => it.wait_for_receipt(),
            Self::AccountDelete(it) => it.wait_for_receipt(),
            Self::AccountAllowanceApprove(it) => it.wait_for_receipt(),
            Self::AccountAllowanceDelete(it) => it.wait_for_receipt(),
            Self::ContractCreate(it) => it.wait_for_receipt(),
            Self::ContractUpdate(it) => it.wait_for_receipt(),
            Self::ContractDelete(it) => it.wait_for_receipt(),
            Self::ContractExecute(it) => it.wait_for_receipt(),
            Self::Transfer(it) => it.wait_for_receipt(),
            Self::TopicCreate(it) => it.wait_for_receipt(),
            Self::TopicUpdate(it) => it.wait_for_receipt(),
            Self::TopicDelete(it) => it.wait_for_receipt(),
            Self::TopicMessageSubmit(it) => it.wait_for_receipt(),
            Self::FileAppend(it) => it.wait_for_receipt(),
            Self::FileCreate(it) => it.wait_for_receipt(),
            Self::FileUpdate(it) => it.wait_for_receipt(),
            Self::FileDelete(it) => it.wait_for_receipt(),
            Self::Prng(it) => it.wait_for_receipt(),
            Self::TokenAssociate(it) => it.wait_for_receipt(),
            Self::TokenBurn(it) => it.wait_for_receipt(),
            Self::TokenCreate(it) => it.wait_for_receipt(),
            Self::TokenDelete(it) => it.wait_for_receipt(),
            Self::TokenDissociate(it) => it.wait_for_receipt(),
            Self::TokenFeeScheduleUpdate(it) => it.wait_for_receipt(),
            Self::TokenFreeze(it) => it.wait_for_receipt(),
            Self::TokenGrantKyc(it) => it.wait_for_receipt(),
            Self::TokenMint(it) => it.wait_for_receipt(),
            Self::TokenPause(it) => it.wait_for_receipt(),
            Self::TokenRevokeKyc(it) => it.wait_for_receipt(),
            Self::TokenUnfreeze(it) => it.wait_for_receipt(),
            Self::TokenUnpause(it) => it.wait_for_receipt(),
            Self::TokenUpdate(it) => it.wait_for_receipt(),
            Self::TokenWipe(it) => it.wait_for_receipt(),
            Self::SystemDelete(it) => it.wait_for_receipt(),
            Self::SystemUndelete(it) => it.wait_for_receipt(),
            Self::Freeze(it) => it.wait_for_receipt(),
            Self::ScheduleCreate(it) => it.wait_for_receipt(),
            Self::ScheduleSign(it) => it.wait_for_receipt(),
            Self::ScheduleDelete(it) => it.wait_for_receipt(),
            Self::Ethereum(it) => it.wait_for_receipt(),
        }
    }
}

impl TransactionExecute for AnyTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        match self {
            Self::Transfer(transaction) => transaction.execute(channel, request),
            Self::AccountCreate(transaction) => transaction.execute(channel, request),
            Self::AccountUpdate(transaction) => transaction.execute(channel, request),
            Self::AccountDelete(transaction) => transaction.execute(channel, request),
            Self::AccountAllowanceApprove(transaction) => transaction.execute(channel, request),
            Self::AccountAllowanceDelete(transaction) => transaction.execute(channel, request),
            Self::ContractCreate(transaction) => transaction.execute(channel, request),
            Self::ContractUpdate(transaction) => transaction.execute(channel, request),
            Self::ContractDelete(transaction) => transaction.execute(channel, request),
            Self::ContractExecute(transaction) => transaction.execute(channel, request),
            Self::FileAppend(transaction) => transaction.execute(channel, request),
            Self::FileCreate(transaction) => transaction.execute(channel, request),
            Self::FileUpdate(transaction) => transaction.execute(channel, request),
            Self::FileDelete(transaction) => transaction.execute(channel, request),
            Self::Prng(transaction) => transaction.execute(channel, request),
            Self::TokenAssociate(transaction) => transaction.execute(channel, request),
            Self::TokenBurn(transaction) => transaction.execute(channel, request),
            Self::TokenCreate(transaction) => transaction.execute(channel, request),
            Self::TokenDelete(transaction) => transaction.execute(channel, request),
            Self::TokenDissociate(transaction) => transaction.execute(channel, request),
            Self::TokenFeeScheduleUpdate(transaction) => transaction.execute(channel, request),
            Self::TokenFreeze(transaction) => transaction.execute(channel, request),
            Self::TokenGrantKyc(transaction) => transaction.execute(channel, request),
            Self::TokenMint(transaction) => transaction.execute(channel, request),
            Self::TokenPause(transaction) => transaction.execute(channel, request),
            Self::TokenRevokeKyc(transaction) => transaction.execute(channel, request),
            Self::TokenUnfreeze(transaction) => transaction.execute(channel, request),
            Self::TokenUnpause(transaction) => transaction.execute(channel, request),
            Self::TokenUpdate(transaction) => transaction.execute(channel, request),
            Self::TokenWipe(transaction) => transaction.execute(channel, request),
            Self::TopicCreate(transaction) => transaction.execute(channel, request),
            Self::TopicUpdate(transaction) => transaction.execute(channel, request),
            Self::TopicDelete(transaction) => transaction.execute(channel, request),
            Self::TopicMessageSubmit(transaction) => transaction.execute(channel, request),
            Self::SystemDelete(transaction) => transaction.execute(channel, request),
            Self::SystemUndelete(transaction) => transaction.execute(channel, request),
            Self::Freeze(transaction) => transaction.execute(channel, request),
            Self::ScheduleCreate(transaction) => transaction.execute(channel, request),
            Self::ScheduleSign(transaction) => transaction.execute(channel, request),
            Self::ScheduleDelete(transaction) => transaction.execute(channel, request),
            Self::Ethereum(transaction) => transaction.execute(channel, request),
        }
    }
}

impl TransactionExecuteChunked for AnyTransactionData {}

impl ValidateChecksums for AnyTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        match self {
            Self::AccountCreate(transaction) => transaction.validate_checksums(ledger_id),
            Self::AccountUpdate(transaction) => transaction.validate_checksums(ledger_id),
            Self::AccountDelete(transaction) => transaction.validate_checksums(ledger_id),
            Self::AccountAllowanceApprove(transaction) => transaction.validate_checksums(ledger_id),
            Self::AccountAllowanceDelete(transaction) => transaction.validate_checksums(ledger_id),
            Self::ContractCreate(transaction) => transaction.validate_checksums(ledger_id),
            Self::ContractUpdate(transaction) => transaction.validate_checksums(ledger_id),
            Self::ContractDelete(transaction) => transaction.validate_checksums(ledger_id),
            Self::ContractExecute(transaction) => transaction.validate_checksums(ledger_id),
            Self::Transfer(transaction) => transaction.validate_checksums(ledger_id),
            Self::TopicCreate(transaction) => transaction.validate_checksums(ledger_id),
            Self::TopicUpdate(transaction) => transaction.validate_checksums(ledger_id),
            Self::TopicDelete(transaction) => transaction.validate_checksums(ledger_id),
            Self::TopicMessageSubmit(transaction) => transaction.validate_checksums(ledger_id),
            Self::FileAppend(transaction) => transaction.validate_checksums(ledger_id),
            Self::FileCreate(transaction) => transaction.validate_checksums(ledger_id),
            Self::FileUpdate(transaction) => transaction.validate_checksums(ledger_id),
            Self::FileDelete(transaction) => transaction.validate_checksums(ledger_id),
            Self::Prng(transaction) => transaction.validate_checksums(ledger_id),
            Self::ScheduleCreate(transaction) => transaction.validate_checksums(ledger_id),
            Self::ScheduleSign(transaction) => transaction.validate_checksums(ledger_id),
            Self::ScheduleDelete(transaction) => transaction.validate_checksums(ledger_id),
            Self::TokenAssociate(transaction) => transaction.validate_checksums(ledger_id),
            Self::TokenBurn(transaction) => transaction.validate_checksums(ledger_id),
            Self::TokenCreate(transaction) => transaction.validate_checksums(ledger_id),
            Self::TokenDelete(transaction) => transaction.validate_checksums(ledger_id),
            Self::TokenDissociate(transaction) => transaction.validate_checksums(ledger_id),
            Self::TokenFeeScheduleUpdate(transaction) => transaction.validate_checksums(ledger_id),
            Self::TokenFreeze(transaction) => transaction.validate_checksums(ledger_id),
            Self::TokenGrantKyc(transaction) => transaction.validate_checksums(ledger_id),
            Self::TokenMint(transaction) => transaction.validate_checksums(ledger_id),
            Self::TokenPause(transaction) => transaction.validate_checksums(ledger_id),
            Self::TokenRevokeKyc(transaction) => transaction.validate_checksums(ledger_id),
            Self::TokenUnfreeze(transaction) => transaction.validate_checksums(ledger_id),
            Self::TokenUnpause(transaction) => transaction.validate_checksums(ledger_id),
            Self::TokenUpdate(transaction) => transaction.validate_checksums(ledger_id),
            Self::TokenWipe(transaction) => transaction.validate_checksums(ledger_id),
            Self::SystemDelete(transaction) => transaction.validate_checksums(ledger_id),
            Self::SystemUndelete(transaction) => transaction.validate_checksums(ledger_id),
            Self::Freeze(transaction) => transaction.validate_checksums(ledger_id),
            Self::Ethereum(transaction) => transaction.validate_checksums(ledger_id),
        }
    }
}

impl FromProtobuf<services::transaction_body::Data> for AnyTransactionData {
    fn from_protobuf(pb: services::transaction_body::Data) -> crate::Result<Self>
    where
        Self: Sized,
    {
        use services::transaction_body::Data;
        let data = match pb {
            Data::ContractCall(pb) => data::ContractExecute::from_protobuf(pb)?.into(),
            Data::ContractCreateInstance(pb) => data::ContractCreate::from_protobuf(pb)?.into(),
            Data::ContractUpdateInstance(pb) => data::ContractUpdate::from_protobuf(pb)?.into(),
            Data::ContractDeleteInstance(pb) => data::ContractDelete::from_protobuf(pb)?.into(),
            Data::EthereumTransaction(pb) => data::Ethereum::from_protobuf(pb)?.into(),
            Data::CryptoApproveAllowance(pb) => {
                data::AccountAllowanceApprove::from_protobuf(pb)?.into()
            }
            Data::CryptoDeleteAllowance(pb) => {
                data::AccountAllowanceDelete::from_protobuf(pb)?.into()
            }
            Data::CryptoCreateAccount(pb) => data::AccountCreate::from_protobuf(pb)?.into(),
            Data::CryptoDelete(pb) => data::AccountDelete::from_protobuf(pb)?.into(),
            Data::CryptoTransfer(pb) => data::Transfer::from_protobuf(pb)?.into(),
            Data::CryptoUpdateAccount(pb) => data::AccountUpdate::from_protobuf(pb)?.into(),
            Data::FileAppend(pb) => data::FileAppend::from_protobuf(pb)?.into(),
            Data::FileCreate(pb) => data::FileCreate::from_protobuf(pb)?.into(),
            Data::FileDelete(pb) => data::FileDelete::from_protobuf(pb)?.into(),
            Data::FileUpdate(pb) => data::FileUpdate::from_protobuf(pb)?.into(),
            Data::UtilPrng(pb) => data::Prng::from_protobuf(pb)?.into(),
            Data::SystemDelete(pb) => data::SystemDelete::from_protobuf(pb)?.into(),
            Data::SystemUndelete(pb) => data::SystemUndelete::from_protobuf(pb)?.into(),
            Data::Freeze(pb) => data::Freeze::from_protobuf(pb)?.into(),
            Data::ConsensusCreateTopic(pb) => data::TopicCreate::from_protobuf(pb)?.into(),
            Data::ConsensusUpdateTopic(pb) => data::TopicUpdate::from_protobuf(pb)?.into(),
            Data::ConsensusDeleteTopic(pb) => data::TopicDelete::from_protobuf(pb)?.into(),
            Data::ConsensusSubmitMessage(pb) => data::TopicMessageSubmit::from_protobuf(pb)?.into(),
            Data::TokenCreation(pb) => data::TokenCreate::from_protobuf(pb)?.into(),
            Data::TokenFreeze(pb) => data::TokenFreeze::from_protobuf(pb)?.into(),
            Data::TokenUnfreeze(pb) => data::TokenUnfreeze::from_protobuf(pb)?.into(),
            Data::TokenGrantKyc(pb) => data::TokenGrantKyc::from_protobuf(pb)?.into(),
            Data::TokenRevokeKyc(pb) => data::TokenRevokeKyc::from_protobuf(pb)?.into(),
            Data::TokenDeletion(pb) => data::TokenDelete::from_protobuf(pb)?.into(),
            Data::TokenUpdate(pb) => data::TokenUpdate::from_protobuf(pb)?.into(),
            Data::TokenMint(pb) => data::TokenMint::from_protobuf(pb)?.into(),
            Data::TokenBurn(pb) => data::TokenBurn::from_protobuf(pb)?.into(),
            Data::TokenWipe(pb) => data::TokenWipe::from_protobuf(pb)?.into(),
            Data::TokenAssociate(pb) => data::TokenAssociate::from_protobuf(pb)?.into(),
            Data::TokenDissociate(pb) => data::TokenDissociate::from_protobuf(pb)?.into(),
            Data::TokenFeeScheduleUpdate(pb) => {
                data::TokenFeeScheduleUpdate::from_protobuf(pb)?.into()
            }
            Data::TokenPause(pb) => data::TokenPause::from_protobuf(pb)?.into(),
            Data::TokenUnpause(pb) => data::TokenUnpause::from_protobuf(pb)?.into(),
            Data::ScheduleCreate(pb) => data::ScheduleCreate::from_protobuf(pb)?.into(),
            Data::ScheduleDelete(pb) => data::ScheduleDelete::from_protobuf(pb)?.into(),
            Data::ScheduleSign(pb) => data::ScheduleSign::from_protobuf(pb)?.into(),
            Data::CryptoAddLiveHash(_) => {
                return Err(Error::from_protobuf(
                    "unsupported transaction `AddLiveHashTransaction`",
                ))
            }
            Data::CryptoDeleteLiveHash(_) => {
                return Err(Error::from_protobuf(
                    "unsupported transaction `DeleteLiveHashTransaction`",
                ))
            }
            Data::UncheckedSubmit(_) => {
                return Err(Error::from_protobuf(
                    "unsupported transaction `UncheckedSubmitTransaction`",
                ))
            }
            Data::NodeStakeUpdate(_) => {
                return Err(Error::from_protobuf(
                    "unsupported transaction `NodeStakeUpdateTransaction`",
                ))
            }
        };

        Ok(data)
    }
}

impl AnyTransactionData {
    // can't do anything about the # of lines, since this function just delegates to `data::_::from_protobuf`.
    #[allow(clippy::too_many_lines)]
    fn from_protobuf(data_chunks: services_data::TransactionData) -> crate::Result<Self> {
        use services_data::{
            Chunkable,
            TransactionData,
            Unchunkable,
        };

        let data = match data_chunks {
            TransactionData::Chunkable(Chunkable::TopicMessageSubmit(v)) => {
                data::TopicMessageSubmit::from_protobuf(v)?.into()
            }
            TransactionData::Chunkable(Chunkable::FileAppend(v)) => {
                data::FileAppend::from_protobuf(v)?.into()
            }

            TransactionData::Unchunkable(it) => match *it {
                Unchunkable::AccountCreate(v) => data::AccountCreate::from_protobuf(v)?.into(),
                Unchunkable::AccountUpdate(v) => data::AccountUpdate::from_protobuf(v)?.into(),
                Unchunkable::AccountDelete(v) => data::AccountDelete::from_protobuf(v)?.into(),
                Unchunkable::AccountAllowanceApprove(v) => {
                    data::AccountAllowanceApprove::from_protobuf(v)?.into()
                }
                Unchunkable::AccountAllowanceDelete(v) => {
                    data::AccountAllowanceDelete::from_protobuf(v)?.into()
                }
                Unchunkable::ContractCreate(v) => data::ContractCreate::from_protobuf(v)?.into(),
                Unchunkable::ContractUpdate(v) => data::ContractUpdate::from_protobuf(v)?.into(),
                Unchunkable::ContractDelete(v) => data::ContractDelete::from_protobuf(v)?.into(),
                Unchunkable::ContractExecute(v) => data::ContractExecute::from_protobuf(v)?.into(),
                Unchunkable::Transfer(v) => data::Transfer::from_protobuf(v)?.into(),
                Unchunkable::TopicCreate(v) => data::TopicCreate::from_protobuf(v)?.into(),
                Unchunkable::TopicUpdate(v) => data::TopicUpdate::from_protobuf(v)?.into(),
                Unchunkable::TopicDelete(v) => data::TopicDelete::from_protobuf(v)?.into(),
                Unchunkable::FileCreate(v) => data::FileCreate::from_protobuf(v)?.into(),
                Unchunkable::FileUpdate(v) => data::FileUpdate::from_protobuf(v)?.into(),
                Unchunkable::FileDelete(v) => data::FileDelete::from_protobuf(v)?.into(),
                Unchunkable::TokenAssociate(v) => data::TokenAssociate::from_protobuf(v)?.into(),
                Unchunkable::TokenBurn(v) => data::TokenBurn::from_protobuf(v)?.into(),
                Unchunkable::TokenCreate(v) => data::TokenCreate::from_protobuf(v)?.into(),
                Unchunkable::TokenDelete(v) => data::TokenDelete::from_protobuf(v)?.into(),
                Unchunkable::TokenDissociate(v) => data::TokenDissociate::from_protobuf(v)?.into(),
                Unchunkable::TokenFeeScheduleUpdate(v) => {
                    data::TokenFeeScheduleUpdate::from_protobuf(v)?.into()
                }
                Unchunkable::TokenFreeze(v) => data::TokenFreeze::from_protobuf(v)?.into(),
                Unchunkable::TokenGrantKyc(v) => data::TokenGrantKyc::from_protobuf(v)?.into(),
                Unchunkable::TokenMint(v) => data::TokenMint::from_protobuf(v)?.into(),
                Unchunkable::TokenPause(v) => data::TokenPause::from_protobuf(v)?.into(),
                Unchunkable::TokenRevokeKyc(v) => data::TokenRevokeKyc::from_protobuf(v)?.into(),
                Unchunkable::TokenUnfreeze(v) => data::TokenUnfreeze::from_protobuf(v)?.into(),
                Unchunkable::TokenUnpause(v) => data::TokenUnpause::from_protobuf(v)?.into(),
                Unchunkable::TokenUpdate(v) => data::TokenUpdate::from_protobuf(v)?.into(),
                Unchunkable::TokenWipe(v) => data::TokenWipe::from_protobuf(v)?.into(),
                Unchunkable::SystemDelete(v) => data::SystemDelete::from_protobuf(v)?.into(),
                Unchunkable::SystemUndelete(v) => data::SystemUndelete::from_protobuf(v)?.into(),
                Unchunkable::Freeze(v) => data::Freeze::from_protobuf(v)?.into(),
                Unchunkable::ScheduleCreate(v) => data::ScheduleCreate::from_protobuf(v)?.into(),
                Unchunkable::ScheduleSign(v) => data::ScheduleSign::from_protobuf(v)?.into(),
                Unchunkable::ScheduleDelete(v) => data::ScheduleDelete::from_protobuf(v)?.into(),
                Unchunkable::Ethereum(v) => data::Ethereum::from_protobuf(v)?.into(),
                Unchunkable::UtilPrng(v) => data::Prng::from_protobuf(v)?.into(),
            },
        };

        Ok(data)
    }
}

impl AnyTransaction {
    pub(super) fn from_protobuf(
        first_body: services::TransactionBody,
        data_chunks: Vec<services::transaction_body::Data>,
    ) -> crate::Result<Self> {
        Ok(Transaction {
            body: TransactionBody {
                node_account_ids: None,
                transaction_valid_duration: first_body.transaction_valid_duration.map(Into::into),
                max_transaction_fee: Some(Hbar::from_tinybars(first_body.transaction_fee as i64)),
                transaction_memo: first_body.memo,
                transaction_id: Some(TransactionId::from_protobuf(pb_getf!(
                    first_body,
                    transaction_id
                )?)?),
                operator: None,
                is_frozen: true,
                regenerate_transaction_id: Some(false),
            },
            data: AnyTransactionData::from_protobuf(
                services_data::TransactionData::from_protobuf(data_chunks)?,
            )?,
            signers: Vec::new(),
            sources: None,
        })
    }
}

impl AnyTransaction {
    /// Attempt to downcast from any transaction to the given transaction kind.
    ///
    /// # Errors
    /// - If self doesn't match the given transaction type, the transaction is returned as-is.
    pub fn downcast<D>(self) -> Result<D, Self>
    where
        Self: DowncastOwned<D>,
    {
        self.downcast_owned()
    }
}

// this is macro worthy (there's like 40 transactions that all do this the exact same way)
/// Impl `DowncastOwned` for `AnyTransactionData`.
///
/// This macro will ensure you get all variants via a pattern match, if something changes (say, another transaction type is added), you'll get a `Missing match arm` compiler error.
macro_rules! impl_downcast_any {
    ($($id:ident),+$(,)?) => {
        $(
            impl $crate::downcast::DowncastOwned<data::$id> for AnyTransactionData {
                fn downcast_owned(self) -> Result<data::$id, Self> {
                    let Self::$id(data) = self else {
                        return Err(self)
                    };

                    Ok(data)
                }
            }
        )*

        #[allow(non_snake_case)]
        mod ___private_impl_downcast_any {
            use super::AnyTransactionData;
            // ensure the what we were given is actually everything.
            fn _assert_exhaustive(d: AnyTransactionData)
            {
                match d {
                    $(AnyTransactionData::$id(_) => {},)+
                }
            }
        }
    };
}

impl_downcast_any! {
    AccountCreate,
    AccountUpdate,
    AccountDelete,
    AccountAllowanceApprove,
    AccountAllowanceDelete,
    ContractCreate,
    ContractUpdate,
    ContractDelete,
    ContractExecute,
    Transfer,
    TopicCreate,
    TopicUpdate,
    TopicDelete,
    TopicMessageSubmit,
    FileAppend,
    FileCreate,
    FileUpdate,
    FileDelete,
    Prng,
    ScheduleCreate,
    ScheduleSign,
    ScheduleDelete,
    TokenAssociate,
    TokenBurn,
    TokenCreate,
    TokenDelete,
    TokenDissociate,
    TokenFeeScheduleUpdate,
    TokenFreeze,
    TokenGrantKyc,
    TokenMint,
    TokenPause,
    TokenRevokeKyc,
    TokenUnfreeze,
    TokenUnpause,
    TokenUpdate,
    TokenWipe,
    SystemDelete,
    SystemUndelete,
    Freeze,
    Ethereum,
}
