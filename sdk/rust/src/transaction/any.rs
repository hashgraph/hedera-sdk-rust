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
use time::Duration;
use tonic::transport::Channel;
use tonic::{
    Response,
    Status,
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
        node_account_id: AccountId,
        transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        match self {
            Self::Transfer(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::AccountCreate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::AccountUpdate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::AccountDelete(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::AccountAllowanceApprove(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::AccountAllowanceDelete(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::ContractCreate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::ContractUpdate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::ContractDelete(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::ContractExecute(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::FileAppend(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::FileCreate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::FileUpdate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::FileDelete(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenAssociate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenBurn(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenCreate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenDelete(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenDissociate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenFeeScheduleUpdate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenFreeze(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenGrantKyc(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenMint(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenPause(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenRevokeKyc(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenUnfreeze(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenUnpause(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenUpdate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TokenWipe(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TopicCreate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TopicUpdate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TopicDelete(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::TopicMessageSubmit(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::SystemDelete(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::SystemUndelete(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::Freeze(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::ScheduleCreate(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::ScheduleSign(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::ScheduleDelete(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }

            Self::Ethereum(transaction) => {
                transaction.to_transaction_data_protobuf(node_account_id, transaction_id)
            }
        }
    }
}

#[async_trait]
impl TransactionExecute for AnyTransactionData {
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

    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<Response<services::TransactionResponse>, Status> {
        match self {
            Self::Transfer(transaction) => transaction.execute(channel, request).await,
            Self::AccountCreate(transaction) => transaction.execute(channel, request).await,
            Self::AccountUpdate(transaction) => transaction.execute(channel, request).await,
            Self::AccountDelete(transaction) => transaction.execute(channel, request).await,
            Self::AccountAllowanceApprove(transaction) => {
                transaction.execute(channel, request).await
            }
            Self::AccountAllowanceDelete(transaction) => {
                transaction.execute(channel, request).await
            }
            Self::ContractCreate(transaction) => transaction.execute(channel, request).await,
            Self::ContractUpdate(transaction) => transaction.execute(channel, request).await,
            Self::ContractDelete(transaction) => transaction.execute(channel, request).await,
            Self::ContractExecute(transaction) => transaction.execute(channel, request).await,
            Self::FileAppend(transaction) => transaction.execute(channel, request).await,
            Self::FileCreate(transaction) => transaction.execute(channel, request).await,
            Self::FileUpdate(transaction) => transaction.execute(channel, request).await,
            Self::FileDelete(transaction) => transaction.execute(channel, request).await,
            Self::TokenAssociate(transaction) => transaction.execute(channel, request).await,
            Self::TokenBurn(transaction) => transaction.execute(channel, request).await,
            Self::TokenCreate(transaction) => transaction.execute(channel, request).await,
            Self::TokenDelete(transaction) => transaction.execute(channel, request).await,
            Self::TokenDissociate(transaction) => transaction.execute(channel, request).await,
            Self::TokenFeeScheduleUpdate(transaction) => {
                transaction.execute(channel, request).await
            }
            Self::TokenFreeze(transaction) => transaction.execute(channel, request).await,
            Self::TokenGrantKyc(transaction) => transaction.execute(channel, request).await,
            Self::TokenMint(transaction) => transaction.execute(channel, request).await,
            Self::TokenPause(transaction) => transaction.execute(channel, request).await,
            Self::TokenRevokeKyc(transaction) => transaction.execute(channel, request).await,
            Self::TokenUnfreeze(transaction) => transaction.execute(channel, request).await,
            Self::TokenUnpause(transaction) => transaction.execute(channel, request).await,
            Self::TokenUpdate(transaction) => transaction.execute(channel, request).await,
            Self::TokenWipe(transaction) => transaction.execute(channel, request).await,
            Self::TopicCreate(transaction) => transaction.execute(channel, request).await,
            Self::TopicUpdate(transaction) => transaction.execute(channel, request).await,
            Self::TopicDelete(transaction) => transaction.execute(channel, request).await,
            Self::TopicMessageSubmit(transaction) => transaction.execute(channel, request).await,
            Self::SystemDelete(transaction) => transaction.execute(channel, request).await,
            Self::SystemUndelete(transaction) => transaction.execute(channel, request).await,
            Self::Freeze(transaction) => transaction.execute(channel, request).await,
            Self::ScheduleCreate(transaction) => transaction.execute(channel, request).await,
            Self::ScheduleSign(transaction) => transaction.execute(channel, request).await,
            Self::ScheduleDelete(transaction) => transaction.execute(channel, request).await,
            Self::Ethereum(transaction) => transaction.execute(channel, request).await,
        }
    }
}

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

impl<D> From<AnyTransactionBody<D>> for Transaction<D>
where
    D: TransactionExecute,
{
    fn from(body: AnyTransactionBody<D>) -> Self {
        Self { body: body.into(), signers: Vec::new(), sources: None }
    }
}

impl<D> From<TransactionBody<D>> for AnyTransactionBody<D>
where
    D: TransactionExecute,
{
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

impl<D> From<AnyTransactionBody<D>> for TransactionBody<D>
where
    D: TransactionExecute,
{
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
