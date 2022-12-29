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

use crate::account::{
    AccountAllowanceApproveTransactionData,
    AccountAllowanceDeleteTransactionData,
    AccountCreateTransactionData,
    AccountDeleteTransactionData,
    AccountUpdateTransactionData,
};
use crate::contract::{
    ContractCreateTransactionData,
    ContractDeleteTransactionData,
    ContractExecuteTransactionData,
    ContractUpdateTransactionData,
};
use crate::ethereum_transaction::EthereumTransactionData;
use crate::file::{
    FileAppendTransactionData,
    FileCreateTransactionData,
    FileDeleteTransactionData,
    FileUpdateTransactionData,
};
use crate::schedule::{
    ScheduleCreateTransactionData,
    ScheduleDeleteTransactionData,
    ScheduleSignTransactionData,
};
use crate::system::{
    FreezeTransactionData,
    SystemDeleteTransactionData,
    SystemUndeleteTransactionData,
};
use crate::token::{
    TokenAssociateTransactionData,
    TokenBurnTransactionData,
    TokenCreateTransactionData,
    TokenDeleteTransactionData,
    TokenDissociateTransactionData,
    TokenFeeScheduleUpdateTransactionData,
    TokenFreezeTransactionData,
    TokenGrantKycTransactionData,
    TokenMintTransactionData,
    TokenPauseTransactionData,
    TokenRevokeKycTransactionData,
    TokenUnfreezeTransactionData,
    TokenUnpauseTransactionData,
    TokenUpdateTransactionData,
    TokenWipeTransactionData,
};
use crate::topic::{
    TopicCreateTransactionData,
    TopicDeleteTransactionData,
    TopicMessageSubmitTransactionData,
    TopicUpdateTransactionData,
};
use crate::transaction::{
    ToTransactionDataProtobuf,
    TransactionBody,
    TransactionExecute,
};
use crate::transfer_transaction::TransferTransactionData;
use crate::{
    AccountId,
    Error,
    Hbar,
    LedgerId,
    Transaction,
    TransactionId,
};

#[cfg(feature = "ffi")]
/// Any possible transaction that may be executed on the Hedera network.
pub type AnyTransaction = Transaction<AnyTransactionData>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", tag = "$type"))]
pub enum AnyTransactionData {
    AccountCreate(AccountCreateTransactionData),
    AccountUpdate(AccountUpdateTransactionData),
    AccountDelete(AccountDeleteTransactionData),
    AccountAllowanceApprove(AccountAllowanceApproveTransactionData),
    AccountAllowanceDelete(AccountAllowanceDeleteTransactionData),
    ContractCreate(ContractCreateTransactionData),
    ContractUpdate(ContractUpdateTransactionData),
    ContractDelete(ContractDeleteTransactionData),
    ContractExecute(ContractExecuteTransactionData),
    Transfer(TransferTransactionData),
    TopicCreate(TopicCreateTransactionData),
    TopicUpdate(TopicUpdateTransactionData),
    TopicDelete(TopicDeleteTransactionData),
    TopicMessageSubmit(TopicMessageSubmitTransactionData),
    FileAppend(FileAppendTransactionData),
    FileCreate(FileCreateTransactionData),
    FileUpdate(FileUpdateTransactionData),
    FileDelete(FileDeleteTransactionData),
    TokenAssociate(TokenAssociateTransactionData),
    TokenBurn(TokenBurnTransactionData),
    TokenCreate(TokenCreateTransactionData),
    TokenDelete(TokenDeleteTransactionData),
    TokenDissociate(TokenDissociateTransactionData),
    TokenFeeScheduleUpdate(TokenFeeScheduleUpdateTransactionData),
    TokenFreeze(TokenFreezeTransactionData),
    TokenGrantKyc(TokenGrantKycTransactionData),
    TokenMint(TokenMintTransactionData),
    TokenPause(TokenPauseTransactionData),
    TokenRevokeKyc(TokenRevokeKycTransactionData),
    TokenUnfreeze(TokenUnfreezeTransactionData),
    TokenUnpause(TokenUnpauseTransactionData),
    TokenUpdate(TokenUpdateTransactionData),
    TokenWipe(TokenWipeTransactionData),
    SystemDelete(SystemDeleteTransactionData),
    SystemUndelete(SystemUndeleteTransactionData),
    Freeze(FreezeTransactionData),
    ScheduleCreate(ScheduleCreateTransactionData),
    ScheduleSign(ScheduleSignTransactionData),
    ScheduleDelete(ScheduleDeleteTransactionData),
    Ethereum(EthereumTransactionData),
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

    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        match self {
            AnyTransactionData::AccountCreate(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::AccountUpdate(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::AccountDelete(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::AccountAllowanceApprove(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::AccountAllowanceDelete(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::ContractCreate(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::ContractUpdate(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::ContractDelete(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::ContractExecute(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::Transfer(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TopicCreate(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TopicUpdate(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TopicDelete(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TopicMessageSubmit(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::FileAppend(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::FileCreate(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::FileUpdate(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::FileDelete(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TokenAssociate(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TokenBurn(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TokenCreate(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TokenDelete(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TokenDissociate(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TokenFeeScheduleUpdate(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TokenFreeze(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TokenGrantKyc(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TokenMint(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TokenPause(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TokenRevokeKyc(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TokenUnfreeze(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TokenUnpause(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TokenUpdate(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::TokenWipe(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::SystemDelete(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::SystemUndelete(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::Freeze(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::ScheduleCreate(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::ScheduleSign(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::ScheduleDelete(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
            AnyTransactionData::Ethereum(transaction) => {
                transaction.validate_checksums_for_ledger_id(ledger_id)
            }
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

    #[cfg_attr(feature = "ffi", serde(default))]
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
    payer_account_id: Option<AccountId>,

    #[cfg_attr(feature = "ffi", serde(default))]
    transaction_id: Option<TransactionId>,
}

impl<D> From<AnyTransactionBody<D>> for Transaction<D>
where
    D: TransactionExecute,
{
    fn from(body: AnyTransactionBody<D>) -> Self {
        Self { body: body.into(), signers: Vec::new(), is_frozen: true }
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
            payer_account_id: body.payer_account_id,
            transaction_id: body.transaction_id,
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
            payer_account_id: body.payer_account_id,
            transaction_id: body.transaction_id,
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
