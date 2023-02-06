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
use time::Duration;
use tonic::transport::Channel;

use super::chunked::ChunkInfo;
use super::{
    TransactionData,
    TransactionExecuteChunked,
};
use crate::client::Operator;
use crate::entity_id::ValidateChecksums;
use crate::protobuf::FromProtobuf;
use crate::transaction::{
    ToTransactionDataProtobuf,
    TransactionBody,
    TransactionExecute,
};
use crate::{
    AccountId,
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
    pub(super) use crate::ethereum_transaction::EthereumTransactionData as Ethereum;
    pub(super) use crate::file::{
        FileAppendTransactionData as FileAppend,
        FileCreateTransactionData as FileCreate,
        FileDeleteTransactionData as FileDelete,
        FileUpdateTransactionData as FileUpdate,
    };
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
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", tag = "$type"))]
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
    ScheduleCreate(data::ScheduleCreate),
    ScheduleSign(data::ScheduleSign),
    ScheduleDelete(data::ScheduleDelete),
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
            AnyTransactionData::AccountCreate(it) => it.maybe_chunk_data(),
            AnyTransactionData::AccountUpdate(it) => it.maybe_chunk_data(),
            AnyTransactionData::AccountDelete(it) => it.maybe_chunk_data(),
            AnyTransactionData::AccountAllowanceApprove(it) => it.maybe_chunk_data(),
            AnyTransactionData::AccountAllowanceDelete(it) => it.maybe_chunk_data(),
            AnyTransactionData::ContractCreate(it) => it.maybe_chunk_data(),
            AnyTransactionData::ContractUpdate(it) => it.maybe_chunk_data(),
            AnyTransactionData::ContractDelete(it) => it.maybe_chunk_data(),
            AnyTransactionData::ContractExecute(it) => it.maybe_chunk_data(),
            AnyTransactionData::Transfer(it) => it.maybe_chunk_data(),
            AnyTransactionData::TopicCreate(it) => it.maybe_chunk_data(),
            AnyTransactionData::TopicUpdate(it) => it.maybe_chunk_data(),
            AnyTransactionData::TopicDelete(it) => it.maybe_chunk_data(),
            AnyTransactionData::TopicMessageSubmit(it) => it.maybe_chunk_data(),
            AnyTransactionData::FileAppend(it) => it.maybe_chunk_data(),
            AnyTransactionData::FileCreate(it) => it.maybe_chunk_data(),
            AnyTransactionData::FileUpdate(it) => it.maybe_chunk_data(),
            AnyTransactionData::FileDelete(it) => it.maybe_chunk_data(),
            AnyTransactionData::TokenAssociate(it) => it.maybe_chunk_data(),
            AnyTransactionData::TokenBurn(it) => it.maybe_chunk_data(),
            AnyTransactionData::TokenCreate(it) => it.maybe_chunk_data(),
            AnyTransactionData::TokenDelete(it) => it.maybe_chunk_data(),
            AnyTransactionData::TokenDissociate(it) => it.maybe_chunk_data(),
            AnyTransactionData::TokenFeeScheduleUpdate(it) => it.maybe_chunk_data(),
            AnyTransactionData::TokenFreeze(it) => it.maybe_chunk_data(),
            AnyTransactionData::TokenGrantKyc(it) => it.maybe_chunk_data(),
            AnyTransactionData::TokenMint(it) => it.maybe_chunk_data(),
            AnyTransactionData::TokenPause(it) => it.maybe_chunk_data(),
            AnyTransactionData::TokenRevokeKyc(it) => it.maybe_chunk_data(),
            AnyTransactionData::TokenUnfreeze(it) => it.maybe_chunk_data(),
            AnyTransactionData::TokenUnpause(it) => it.maybe_chunk_data(),
            AnyTransactionData::TokenUpdate(it) => it.maybe_chunk_data(),
            AnyTransactionData::TokenWipe(it) => it.maybe_chunk_data(),
            AnyTransactionData::SystemDelete(it) => it.maybe_chunk_data(),
            AnyTransactionData::SystemUndelete(it) => it.maybe_chunk_data(),
            AnyTransactionData::Freeze(it) => it.maybe_chunk_data(),
            AnyTransactionData::ScheduleCreate(it) => it.maybe_chunk_data(),
            AnyTransactionData::ScheduleSign(it) => it.maybe_chunk_data(),
            AnyTransactionData::ScheduleDelete(it) => it.maybe_chunk_data(),
            AnyTransactionData::Ethereum(it) => it.maybe_chunk_data(),
        }
    }

    fn wait_for_receipt(&self) -> bool {
        match self {
            AnyTransactionData::AccountCreate(it) => it.wait_for_receipt(),
            AnyTransactionData::AccountUpdate(it) => it.wait_for_receipt(),
            AnyTransactionData::AccountDelete(it) => it.wait_for_receipt(),
            AnyTransactionData::AccountAllowanceApprove(it) => it.wait_for_receipt(),
            AnyTransactionData::AccountAllowanceDelete(it) => it.wait_for_receipt(),
            AnyTransactionData::ContractCreate(it) => it.wait_for_receipt(),
            AnyTransactionData::ContractUpdate(it) => it.wait_for_receipt(),
            AnyTransactionData::ContractDelete(it) => it.wait_for_receipt(),
            AnyTransactionData::ContractExecute(it) => it.wait_for_receipt(),
            AnyTransactionData::Transfer(it) => it.wait_for_receipt(),
            AnyTransactionData::TopicCreate(it) => it.wait_for_receipt(),
            AnyTransactionData::TopicUpdate(it) => it.wait_for_receipt(),
            AnyTransactionData::TopicDelete(it) => it.wait_for_receipt(),
            AnyTransactionData::TopicMessageSubmit(it) => it.wait_for_receipt(),
            AnyTransactionData::FileAppend(it) => it.wait_for_receipt(),
            AnyTransactionData::FileCreate(it) => it.wait_for_receipt(),
            AnyTransactionData::FileUpdate(it) => it.wait_for_receipt(),
            AnyTransactionData::FileDelete(it) => it.wait_for_receipt(),
            AnyTransactionData::TokenAssociate(it) => it.wait_for_receipt(),
            AnyTransactionData::TokenBurn(it) => it.wait_for_receipt(),
            AnyTransactionData::TokenCreate(it) => it.wait_for_receipt(),
            AnyTransactionData::TokenDelete(it) => it.wait_for_receipt(),
            AnyTransactionData::TokenDissociate(it) => it.wait_for_receipt(),
            AnyTransactionData::TokenFeeScheduleUpdate(it) => it.wait_for_receipt(),
            AnyTransactionData::TokenFreeze(it) => it.wait_for_receipt(),
            AnyTransactionData::TokenGrantKyc(it) => it.wait_for_receipt(),
            AnyTransactionData::TokenMint(it) => it.wait_for_receipt(),
            AnyTransactionData::TokenPause(it) => it.wait_for_receipt(),
            AnyTransactionData::TokenRevokeKyc(it) => it.wait_for_receipt(),
            AnyTransactionData::TokenUnfreeze(it) => it.wait_for_receipt(),
            AnyTransactionData::TokenUnpause(it) => it.wait_for_receipt(),
            AnyTransactionData::TokenUpdate(it) => it.wait_for_receipt(),
            AnyTransactionData::TokenWipe(it) => it.wait_for_receipt(),
            AnyTransactionData::SystemDelete(it) => it.wait_for_receipt(),
            AnyTransactionData::SystemUndelete(it) => it.wait_for_receipt(),
            AnyTransactionData::Freeze(it) => it.wait_for_receipt(),
            AnyTransactionData::ScheduleCreate(it) => it.wait_for_receipt(),
            AnyTransactionData::ScheduleSign(it) => it.wait_for_receipt(),
            AnyTransactionData::ScheduleDelete(it) => it.wait_for_receipt(),
            AnyTransactionData::Ethereum(it) => it.wait_for_receipt(),
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
            AnyTransactionData::AccountCreate(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::AccountUpdate(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::AccountDelete(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::AccountAllowanceApprove(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::AccountAllowanceDelete(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::ContractCreate(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::ContractUpdate(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::ContractDelete(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::ContractExecute(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::Transfer(transaction) => transaction.validate_checksums(ledger_id),
            AnyTransactionData::TopicCreate(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TopicUpdate(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TopicDelete(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TopicMessageSubmit(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::FileAppend(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::FileCreate(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::FileUpdate(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::FileDelete(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TokenAssociate(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TokenBurn(transaction) => transaction.validate_checksums(ledger_id),
            AnyTransactionData::TokenCreate(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TokenDelete(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TokenDissociate(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TokenFeeScheduleUpdate(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TokenFreeze(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TokenGrantKyc(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TokenMint(transaction) => transaction.validate_checksums(ledger_id),
            AnyTransactionData::TokenPause(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TokenRevokeKyc(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TokenUnfreeze(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TokenUnpause(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TokenUpdate(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::TokenWipe(transaction) => transaction.validate_checksums(ledger_id),
            AnyTransactionData::SystemDelete(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::SystemUndelete(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::Freeze(transaction) => transaction.validate_checksums(ledger_id),
            AnyTransactionData::ScheduleCreate(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::ScheduleSign(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::ScheduleDelete(transaction) => {
                transaction.validate_checksums(ledger_id)
            }
            AnyTransactionData::Ethereum(transaction) => transaction.validate_checksums(ledger_id),
        }
    }
}

// NOTE: as we cannot derive Deserialize on Query<T> directly as `T` is not Deserialize,
//  we create a proxy type that has the same layout but is only for AnyQueryData and does
//  derive(Deserialize).

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub(crate) struct AnyTransactionBody<D> {
    #[cfg_attr(feature = "ffi", serde(flatten))]
    data: D,

    node_account_ids: Option<Vec<AccountId>>,

    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::DurationSeconds<i64>>>")
    )]
    #[cfg_attr(feature = "ffi", serde(default))]
    transaction_valid_duration: Option<Duration>,

    #[cfg_attr(feature = "ffi", serde(default))]
    max_transaction_fee: Option<Hbar>,

    #[cfg_attr(feature = "ffi", serde(default, skip_serializing_if = "String::is_empty"))]
    transaction_memo: String,

    #[cfg_attr(feature = "ffi", serde(default))]
    transaction_id: Option<TransactionId>,

    #[cfg_attr(feature = "ffi", serde(default))]
    #[cfg_attr(feature = "ffi", serde(skip_serializing_if = "std::ops::Not::not"))]
    is_frozen: bool,

    #[cfg_attr(feature = "ffi", serde(default))]
    operator: Option<Operator>,
}

impl<D> From<AnyTransactionBody<D>> for Transaction<D> {
    fn from(body: AnyTransactionBody<D>) -> Self {
        Self { body: body.into(), signers: Vec::new(), sources: None }
    }
}

impl<D> From<TransactionBody<D>> for AnyTransactionBody<D> {
    fn from(body: TransactionBody<D>) -> Self {
        Self {
            data: body.data,
            node_account_ids: body.node_account_ids,
            transaction_valid_duration: body.transaction_valid_duration,
            max_transaction_fee: body.max_transaction_fee,
            transaction_memo: body.transaction_memo,
            transaction_id: body.transaction_id,
            is_frozen: body.is_frozen,
            operator: body.operator,
        }
    }
}

impl<D> From<AnyTransactionBody<D>> for TransactionBody<D> {
    fn from(body: AnyTransactionBody<D>) -> Self {
        Self {
            data: body.data,
            node_account_ids: body.node_account_ids,
            transaction_valid_duration: body.transaction_valid_duration,
            max_transaction_fee: body.max_transaction_fee,
            transaction_memo: body.transaction_memo,
            transaction_id: body.transaction_id,
            is_frozen: body.is_frozen,
            operator: body.operator,
        }
    }
}

#[cfg(feature = "ffi")]
impl<'de> serde::Deserialize<'de> for AnyTransaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <AnyTransactionBody<AnyTransactionData> as serde::Deserialize>::deserialize(deserializer)
            .map(AnyTransactionBody::into)
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
            Data::UtilPrng(_) => {
                return Err(Error::from_protobuf("unimplemented transaction `PrngTransaction`"))
            }
        };

        Ok(data)
    }
}

impl AnyTransactionData {
    fn from_protobuf(data_chunks: ServicesTransactionDataList) -> crate::Result<Self> {
        fn try_into_only_element<T>(v: Vec<T>) -> crate::Result<T> {
            match <[T; 1]>::try_from(v) {
                Ok([element]) => Ok(element),
                _ => Err(Error::from_protobuf("chunks in non chunkable transaction")),
            }
        }

        let data = match data_chunks {
            ServicesTransactionDataList::AccountCreate(v) => {
                data::AccountCreate::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::AccountUpdate(v) => {
                data::AccountUpdate::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::AccountDelete(v) => {
                data::AccountDelete::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::AccountAllowanceApprove(v) => {
                data::AccountAllowanceApprove::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::AccountAllowanceDelete(v) => {
                data::AccountAllowanceDelete::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::ContractCreate(v) => {
                data::ContractCreate::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::ContractUpdate(v) => {
                data::ContractUpdate::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::ContractDelete(v) => {
                data::ContractDelete::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::ContractExecute(v) => {
                data::ContractExecute::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::Transfer(v) => {
                data::Transfer::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TopicCreate(v) => {
                data::TopicCreate::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TopicUpdate(v) => {
                data::TopicUpdate::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TopicDelete(v) => {
                data::TopicDelete::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TopicMessageSubmit(v) => {
                data::TopicMessageSubmit::from_protobuf(v)?.into()
            }
            ServicesTransactionDataList::FileAppend(v) => {
                data::FileAppend::from_protobuf(v)?.into()
            }
            ServicesTransactionDataList::FileCreate(v) => {
                data::FileCreate::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::FileUpdate(v) => {
                data::FileUpdate::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::FileDelete(v) => {
                data::FileDelete::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TokenAssociate(v) => {
                data::TokenAssociate::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TokenBurn(v) => {
                data::TokenBurn::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TokenCreate(v) => {
                data::TokenCreate::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TokenDelete(v) => {
                data::TokenDelete::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TokenDissociate(v) => {
                data::TokenDissociate::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TokenFeeScheduleUpdate(v) => {
                data::TokenFeeScheduleUpdate::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TokenFreeze(v) => {
                data::TokenFreeze::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TokenGrantKyc(v) => {
                data::TokenGrantKyc::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TokenMint(v) => {
                data::TokenMint::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TokenPause(v) => {
                data::TokenPause::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TokenRevokeKyc(v) => {
                data::TokenRevokeKyc::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TokenUnfreeze(v) => {
                data::TokenUnfreeze::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TokenUnpause(v) => {
                data::TokenUnpause::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TokenUpdate(v) => {
                data::TokenUpdate::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::TokenWipe(v) => {
                data::TokenWipe::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::SystemDelete(v) => {
                data::SystemDelete::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::SystemUndelete(v) => {
                data::SystemUndelete::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::Freeze(v) => {
                data::Freeze::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::ScheduleCreate(v) => {
                data::ScheduleCreate::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::ScheduleSign(v) => {
                data::ScheduleSign::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::ScheduleDelete(v) => {
                data::ScheduleDelete::from_protobuf(try_into_only_element(v)?)?.into()
            }
            ServicesTransactionDataList::Ethereum(v) => {
                data::Ethereum::from_protobuf(try_into_only_element(v)?)?.into()
            }
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
                data: AnyTransactionData::from_protobuf(
                    ServicesTransactionDataList::from_protobuf(data_chunks)?,
                )?,
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
            },
            signers: Vec::new(),
            sources: None,
        })
    }
}

// sigh.
#[derive(Debug)]
enum ServicesTransactionDataList {
    AccountCreate(Vec<services::CryptoCreateTransactionBody>),
    AccountUpdate(Vec<services::CryptoUpdateTransactionBody>),
    AccountDelete(Vec<services::CryptoDeleteTransactionBody>),
    AccountAllowanceApprove(Vec<services::CryptoApproveAllowanceTransactionBody>),
    AccountAllowanceDelete(Vec<services::CryptoDeleteAllowanceTransactionBody>),
    ContractCreate(Vec<services::ContractCreateTransactionBody>),
    ContractUpdate(Vec<services::ContractUpdateTransactionBody>),
    ContractDelete(Vec<services::ContractDeleteTransactionBody>),
    ContractExecute(Vec<services::ContractCallTransactionBody>),
    Transfer(Vec<services::CryptoTransferTransactionBody>),
    TopicCreate(Vec<services::ConsensusCreateTopicTransactionBody>),
    TopicUpdate(Vec<services::ConsensusUpdateTopicTransactionBody>),
    TopicDelete(Vec<services::ConsensusDeleteTopicTransactionBody>),
    TopicMessageSubmit(Vec<services::ConsensusSubmitMessageTransactionBody>),
    FileAppend(Vec<services::FileAppendTransactionBody>),
    FileCreate(Vec<services::FileCreateTransactionBody>),
    FileUpdate(Vec<services::FileUpdateTransactionBody>),
    FileDelete(Vec<services::FileDeleteTransactionBody>),
    TokenAssociate(Vec<services::TokenAssociateTransactionBody>),
    TokenBurn(Vec<services::TokenBurnTransactionBody>),
    TokenCreate(Vec<services::TokenCreateTransactionBody>),
    TokenDelete(Vec<services::TokenDeleteTransactionBody>),
    TokenDissociate(Vec<services::TokenDissociateTransactionBody>),
    TokenFeeScheduleUpdate(Vec<services::TokenFeeScheduleUpdateTransactionBody>),
    TokenFreeze(Vec<services::TokenFreezeAccountTransactionBody>),
    TokenGrantKyc(Vec<services::TokenGrantKycTransactionBody>),
    TokenMint(Vec<services::TokenMintTransactionBody>),
    TokenPause(Vec<services::TokenPauseTransactionBody>),
    TokenRevokeKyc(Vec<services::TokenRevokeKycTransactionBody>),
    TokenUnfreeze(Vec<services::TokenUnfreezeAccountTransactionBody>),
    TokenUnpause(Vec<services::TokenUnpauseTransactionBody>),
    TokenUpdate(Vec<services::TokenUpdateTransactionBody>),
    TokenWipe(Vec<services::TokenWipeAccountTransactionBody>),
    SystemDelete(Vec<services::SystemDeleteTransactionBody>),
    SystemUndelete(Vec<services::SystemUndeleteTransactionBody>),
    Freeze(Vec<services::FreezeTransactionBody>),
    ScheduleCreate(Vec<services::ScheduleCreateTransactionBody>),
    ScheduleSign(Vec<services::ScheduleSignTransactionBody>),
    ScheduleDelete(Vec<services::ScheduleDeleteTransactionBody>),
    Ethereum(Vec<services::EthereumTransactionBody>),
}

impl FromProtobuf<Vec<services::transaction_body::Data>> for ServicesTransactionDataList {
    fn from_protobuf(pb: Vec<services::transaction_body::Data>) -> crate::Result<Self> {
        use services::transaction_body::Data;

        fn make_vec<T>(first: T, cap: usize) -> Vec<T> {
            let mut v = Vec::with_capacity(cap);
            v.push(first);
            v
        }

        let len = pb.len();

        let mut iter = pb.into_iter();

        let first = iter
            .next()
            .expect("empty transaction data list (should be handled earlier up the pipeline)");

        // note: this impl is what I (srr) believe to be the "best" impl
        let mut value = match first {
            Data::ContractCall(it) => Self::ContractExecute(make_vec(it, len)),
            Data::ContractCreateInstance(it) => Self::ContractCreate(make_vec(it, len)),
            Data::ContractUpdateInstance(it) => Self::ContractUpdate(make_vec(it, len)),
            Data::ContractDeleteInstance(it) => Self::ContractDelete(make_vec(it, len)),
            Data::EthereumTransaction(it) => Self::Ethereum(make_vec(it, len)),
            Data::CryptoApproveAllowance(it) => Self::AccountAllowanceApprove(make_vec(it, len)),
            Data::CryptoDeleteAllowance(it) => Self::AccountAllowanceDelete(make_vec(it, len)),
            Data::CryptoCreateAccount(it) => Self::AccountCreate(make_vec(it, len)),
            Data::CryptoDelete(it) => Self::AccountDelete(make_vec(it, len)),
            Data::CryptoTransfer(it) => Self::Transfer(make_vec(it, len)),
            Data::CryptoUpdateAccount(it) => Self::AccountUpdate(make_vec(it, len)),
            Data::FileAppend(it) => Self::FileAppend(make_vec(it, len)),
            Data::FileCreate(it) => Self::FileCreate(make_vec(it, len)),
            Data::FileDelete(it) => Self::FileDelete(make_vec(it, len)),
            Data::FileUpdate(it) => Self::FileUpdate(make_vec(it, len)),
            Data::SystemDelete(it) => Self::SystemDelete(make_vec(it, len)),
            Data::SystemUndelete(it) => Self::SystemUndelete(make_vec(it, len)),
            Data::Freeze(it) => Self::Freeze(make_vec(it, len)),
            Data::ConsensusCreateTopic(it) => Self::TopicCreate(make_vec(it, len)),
            Data::ConsensusUpdateTopic(it) => Self::TopicUpdate(make_vec(it, len)),
            Data::ConsensusDeleteTopic(it) => Self::TopicDelete(make_vec(it, len)),
            Data::ConsensusSubmitMessage(it) => Self::TopicMessageSubmit(make_vec(it, len)),
            Data::TokenCreation(it) => Self::TokenCreate(make_vec(it, len)),
            Data::TokenFreeze(it) => Self::TokenFreeze(make_vec(it, len)),
            Data::TokenUnfreeze(it) => Self::TokenUnfreeze(make_vec(it, len)),
            Data::TokenGrantKyc(it) => Self::TokenGrantKyc(make_vec(it, len)),
            Data::TokenRevokeKyc(it) => Self::TokenRevokeKyc(make_vec(it, len)),
            Data::TokenDeletion(it) => Self::TokenDelete(make_vec(it, len)),
            Data::TokenUpdate(it) => Self::TokenUpdate(make_vec(it, len)),
            Data::TokenMint(it) => Self::TokenMint(make_vec(it, len)),
            Data::TokenBurn(it) => Self::TokenBurn(make_vec(it, len)),
            Data::TokenWipe(it) => Self::TokenWipe(make_vec(it, len)),
            Data::TokenAssociate(it) => Self::TokenAssociate(make_vec(it, len)),
            Data::TokenDissociate(it) => Self::TokenDissociate(make_vec(it, len)),
            Data::TokenFeeScheduleUpdate(it) => Self::TokenFeeScheduleUpdate(make_vec(it, len)),
            Data::TokenPause(it) => Self::TokenPause(make_vec(it, len)),
            Data::TokenUnpause(it) => Self::TokenUnpause(make_vec(it, len)),
            Data::ScheduleCreate(it) => Self::ScheduleCreate(make_vec(it, len)),
            Data::ScheduleDelete(it) => Self::ScheduleDelete(make_vec(it, len)),
            Data::ScheduleSign(it) => Self::ScheduleSign(make_vec(it, len)),

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
            Data::UtilPrng(_) => {
                return Err(Error::from_protobuf("unimplemented transaction `PrngTransaction`"))
            }
        };

        for transaction in iter {
            match (&mut value, transaction) {
                (Self::AccountCreate(v), Data::CryptoCreateAccount(element)) => v.push(element),
                (Self::AccountUpdate(v), Data::CryptoUpdateAccount(element)) => v.push(element),
                (Self::AccountDelete(v), Data::CryptoDelete(element)) => v.push(element),
                (Self::AccountAllowanceApprove(v), Data::CryptoApproveAllowance(element)) => {
                    v.push(element)
                }
                (Self::AccountAllowanceDelete(v), Data::CryptoDeleteAllowance(element)) => {
                    v.push(element)
                }
                (Self::ContractCreate(v), Data::ContractCreateInstance(element)) => v.push(element),
                (Self::ContractUpdate(v), Data::ContractUpdateInstance(element)) => v.push(element),
                (Self::ContractDelete(v), Data::ContractDeleteInstance(element)) => v.push(element),
                (Self::ContractExecute(v), Data::ContractCall(element)) => v.push(element),
                (Self::Transfer(v), Data::CryptoTransfer(element)) => v.push(element),
                (Self::TopicCreate(v), Data::ConsensusCreateTopic(element)) => v.push(element),
                (Self::TopicUpdate(v), Data::ConsensusUpdateTopic(element)) => v.push(element),
                (Self::TopicDelete(v), Data::ConsensusDeleteTopic(element)) => v.push(element),
                (Self::TopicMessageSubmit(v), Data::ConsensusSubmitMessage(element)) => {
                    v.push(element)
                }
                (Self::FileAppend(v), Data::FileAppend(element)) => v.push(element),
                (Self::FileCreate(v), Data::FileCreate(element)) => v.push(element),
                (Self::FileUpdate(v), Data::FileUpdate(element)) => v.push(element),
                (Self::FileDelete(v), Data::FileDelete(element)) => v.push(element),
                (Self::TokenAssociate(v), Data::TokenAssociate(element)) => v.push(element),
                (Self::TokenBurn(v), Data::TokenBurn(element)) => v.push(element),
                (Self::TokenCreate(v), Data::TokenCreation(element)) => v.push(element),
                (Self::TokenDelete(v), Data::TokenDeletion(element)) => v.push(element),
                (Self::TokenDissociate(v), Data::TokenDissociate(element)) => v.push(element),
                (Self::TokenFeeScheduleUpdate(v), Data::TokenFeeScheduleUpdate(element)) => {
                    v.push(element)
                }
                (Self::TokenFreeze(v), Data::TokenFreeze(element)) => v.push(element),
                (Self::TokenGrantKyc(v), Data::TokenGrantKyc(element)) => v.push(element),
                (Self::TokenMint(v), Data::TokenMint(element)) => v.push(element),
                (Self::TokenPause(v), Data::TokenPause(element)) => v.push(element),
                (Self::TokenRevokeKyc(v), Data::TokenRevokeKyc(element)) => v.push(element),
                (Self::TokenUnfreeze(v), Data::TokenUnfreeze(element)) => v.push(element),
                (Self::TokenUnpause(v), Data::TokenUnpause(element)) => v.push(element),
                (Self::TokenUpdate(v), Data::TokenUpdate(element)) => v.push(element),
                (Self::TokenWipe(v), Data::TokenWipe(element)) => v.push(element),
                (Self::SystemDelete(v), Data::SystemDelete(element)) => v.push(element),
                (Self::SystemUndelete(v), Data::SystemUndelete(element)) => v.push(element),
                (Self::Freeze(v), Data::Freeze(element)) => v.push(element),
                (Self::ScheduleCreate(v), Data::ScheduleCreate(element)) => v.push(element),
                (Self::ScheduleSign(v), Data::ScheduleSign(element)) => v.push(element),
                (Self::ScheduleDelete(v), Data::ScheduleDelete(element)) => v.push(element),
                (Self::Ethereum(v), Data::EthereumTransaction(element)) => v.push(element),
                _ => return Err(Error::from_protobuf("mismatched transaction types")),
            }
        }

        Ok(value)
    }
}
