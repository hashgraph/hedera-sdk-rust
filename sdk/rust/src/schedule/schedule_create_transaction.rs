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

use hedera_proto::services::schedule_service_client::ScheduleServiceClient;
use hedera_proto::services::transaction_body::Data;
use hedera_proto::services::{
    self,
    schedulable_transaction_body,
    transaction_body,
};
use time::OffsetDateTime;
use tonic::transport::Channel;

use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::{
    AccountId,
    BoxGrpcFuture,
    Error,
    Hbar,
    Key,
    LedgerId,
    Transaction,
    TransactionId,
    ValidateChecksums,
};

/// Create a new schedule entity (or simply, schedule) in the network's action queue.
///
/// Upon `SUCCESS`, the receipt contains the `ScheduleId` of the created schedule. A schedule
/// entity includes a `scheduled_transaction_body` to be executed.
///
/// When the schedule has collected enough signing keys to satisfy the schedule's signing
/// requirements, the schedule can be executed.
///
pub type ScheduleCreateTransaction = Transaction<ScheduleCreateTransactionData>;

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", default))]
pub struct ScheduleCreateTransactionData {
    scheduled_transaction: Option<SchedulableTransactionBody>,

    schedule_memo: Option<String>,

    admin_key: Option<Key>,

    payer_account_id: Option<AccountId>,

    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::TimestampNanoSeconds>>")
    )]
    expiration_time: Option<OffsetDateTime>,

    wait_for_expiry: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
struct SchedulableTransactionBody {
    #[cfg_attr(feature = "ffi", serde(flatten))]
    data: Box<AnyTransactionData>,

    #[cfg_attr(feature = "ffi", serde(default))]
    max_transaction_fee: Option<Hbar>,

    #[cfg_attr(feature = "ffi", serde(default, skip_serializing_if = "String::is_empty"))]
    transaction_memo: String,
}

impl ScheduleCreateTransaction {
    // note(sr): not sure what the right way to go about this is?
    // pub fn get_scheduled_transaction(&self) -> Option<&SchedulableTransactionBody> {
    //     self.data().scheduled_transaction.as_ref()
    // }

    /// Sets the scheduled transaction.
    pub fn scheduled_transaction<D>(&mut self, transaction: Transaction<D>) -> &mut Self
    where
        D: TransactionExecute,
    {
        let body = transaction.into_body();

        self.data_mut().scheduled_transaction = Some(SchedulableTransactionBody {
            max_transaction_fee: body.max_transaction_fee,
            transaction_memo: body.transaction_memo,
            data: Box::new(body.data.into()),
        });

        self
    }

    /// Returns the timestamp for when the transaction should be evaluated for execution and then expire.
    #[must_use]
    pub fn get_expiration_time(&self) -> Option<OffsetDateTime> {
        self.data().expiration_time
    }

    /// Sets the timestamp for when the transaction should be evaluated for execution and then expire.
    pub fn expiration_time(&mut self, time: OffsetDateTime) -> &mut Self {
        self.data_mut().expiration_time = Some(time);
        self
    }

    /// Returns `true` if the transaction will be evaluated at `expiration_time` instead
    /// of when all the required signatures are received, `false` otherwise.
    #[must_use]
    pub fn get_wait_for_expiry(&self) -> bool {
        self.data().wait_for_expiry
    }

    /// Sets if the transaction will be evaluated for execution at `expiration_time` instead
    /// of when all required signatures are received.
    pub fn wait_for_expiry(&mut self, wait: bool) -> &mut Self {
        self.data_mut().wait_for_expiry = wait;
        self
    }

    /// Returns the id of the account to be charged the service fee for the scheduled transaction at
    /// the consensus time it executes (if ever).
    #[must_use]
    pub fn get_payer_account_id(&self) -> Option<AccountId> {
        self.data().payer_account_id
    }

    /// Sets the id of the account to be charged the service fee for the scheduled transaction at
    /// the consensus time that it executes (if ever).
    pub fn payer_account_id(&mut self, id: AccountId) -> &mut Self {
        self.data_mut().payer_account_id = Some(id);
        self
    }

    /// Returns the memo for the schedule entity.
    #[must_use]
    pub fn get_schedule_memo(&self) -> Option<&str> {
        self.data().schedule_memo.as_deref()
    }

    /// Sets the memo for the schedule entity.
    pub fn schedule_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.data_mut().schedule_memo = Some(memo.into());
        self
    }

    /// Returns the Hedera key which can be used to sign a `ScheduleDelete` and remove the schedule.
    #[must_use]
    pub fn get_admin_key(&self) -> Option<&Key> {
        self.data().admin_key.as_ref()
    }

    /// Sets the Hedera key which can be used to sign a `ScheduleDelete` and remove the schedule.
    pub fn admin_key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.data_mut().admin_key = Some(key.into());
        self
    }
}

impl TransactionData for ScheduleCreateTransactionData {}

impl TransactionExecute for ScheduleCreateTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { ScheduleServiceClient::new(channel).create_schedule(request).await })
    }
}

impl ValidateChecksums for ScheduleCreateTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.payer_account_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for ScheduleCreateTransactionData {
    // not really anything I can do about this
    #[allow(clippy::too_many_lines)]
    fn to_transaction_data_protobuf(
        &self,
        node_account_id: AccountId,
        transaction_id: &TransactionId,
    ) -> transaction_body::Data {
        let body = self.scheduled_transaction.as_ref().map(|scheduled| {
            let data = scheduled.data.to_transaction_data_protobuf(node_account_id, transaction_id);

            #[allow(clippy::match_same_arms)]
            let data = match data {
                transaction_body::Data::ConsensusCreateTopic(data) => {
                    Some(schedulable_transaction_body::Data::ConsensusCreateTopic(data))
                }
                transaction_body::Data::ContractCreateInstance(data) => {
                    Some(schedulable_transaction_body::Data::ContractCreateInstance(data))
                }
                transaction_body::Data::ContractUpdateInstance(data) => {
                    Some(schedulable_transaction_body::Data::ContractUpdateInstance(data))
                }
                transaction_body::Data::ContractDeleteInstance(data) => {
                    Some(schedulable_transaction_body::Data::ContractDeleteInstance(data))
                }
                transaction_body::Data::EthereumTransaction(_) => {
                    // NOTE: cannot schedule a EthereumTransaction transaction
                    None
                }
                transaction_body::Data::CryptoAddLiveHash(_) => {
                    // NOTE: cannot schedule a CryptoAddLiveHash transaction
                    None
                }
                transaction_body::Data::CryptoApproveAllowance(data) => {
                    Some(schedulable_transaction_body::Data::CryptoApproveAllowance(data))
                }
                transaction_body::Data::CryptoDeleteAllowance(data) => {
                    Some(schedulable_transaction_body::Data::CryptoDeleteAllowance(data))
                }
                transaction_body::Data::CryptoCreateAccount(data) => {
                    Some(schedulable_transaction_body::Data::CryptoCreateAccount(data))
                }
                transaction_body::Data::CryptoDelete(data) => {
                    Some(schedulable_transaction_body::Data::CryptoDelete(data))
                }
                transaction_body::Data::CryptoDeleteLiveHash(_) => {
                    // NOTE: cannot schedule a CryptoDeleteLiveHash transaction
                    None
                }
                transaction_body::Data::CryptoTransfer(data) => {
                    Some(schedulable_transaction_body::Data::CryptoTransfer(data))
                }
                transaction_body::Data::CryptoUpdateAccount(data) => {
                    Some(schedulable_transaction_body::Data::CryptoUpdateAccount(data))
                }
                transaction_body::Data::FileAppend(data) => {
                    Some(schedulable_transaction_body::Data::FileAppend(data))
                }
                transaction_body::Data::FileCreate(data) => {
                    Some(schedulable_transaction_body::Data::FileCreate(data))
                }
                transaction_body::Data::FileDelete(data) => {
                    Some(schedulable_transaction_body::Data::FileDelete(data))
                }
                transaction_body::Data::FileUpdate(data) => {
                    Some(schedulable_transaction_body::Data::FileUpdate(data))
                }
                transaction_body::Data::SystemDelete(data) => {
                    Some(schedulable_transaction_body::Data::SystemDelete(data))
                }
                transaction_body::Data::SystemUndelete(data) => {
                    Some(schedulable_transaction_body::Data::SystemUndelete(data))
                }
                transaction_body::Data::Freeze(data) => {
                    Some(schedulable_transaction_body::Data::Freeze(data))
                }
                transaction_body::Data::ConsensusUpdateTopic(data) => {
                    Some(schedulable_transaction_body::Data::ConsensusUpdateTopic(data))
                }
                transaction_body::Data::ConsensusDeleteTopic(data) => {
                    Some(schedulable_transaction_body::Data::ConsensusDeleteTopic(data))
                }
                transaction_body::Data::ConsensusSubmitMessage(data) => {
                    Some(schedulable_transaction_body::Data::ConsensusSubmitMessage(data))
                }
                transaction_body::Data::UncheckedSubmit(_) => {
                    // NOTE: cannot schedule a UncheckedSubmit transaction
                    None
                }
                transaction_body::Data::TokenCreation(data) => {
                    Some(schedulable_transaction_body::Data::TokenCreation(data))
                }
                transaction_body::Data::TokenFreeze(data) => {
                    Some(schedulable_transaction_body::Data::TokenFreeze(data))
                }
                transaction_body::Data::TokenUnfreeze(data) => {
                    Some(schedulable_transaction_body::Data::TokenUnfreeze(data))
                }
                transaction_body::Data::TokenGrantKyc(data) => {
                    Some(schedulable_transaction_body::Data::TokenGrantKyc(data))
                }
                transaction_body::Data::TokenRevokeKyc(data) => {
                    Some(schedulable_transaction_body::Data::TokenRevokeKyc(data))
                }
                transaction_body::Data::TokenDeletion(data) => {
                    Some(schedulable_transaction_body::Data::TokenDeletion(data))
                }
                transaction_body::Data::TokenUpdate(data) => {
                    Some(schedulable_transaction_body::Data::TokenUpdate(data))
                }
                transaction_body::Data::TokenMint(data) => {
                    Some(schedulable_transaction_body::Data::TokenMint(data))
                }
                transaction_body::Data::TokenBurn(data) => {
                    Some(schedulable_transaction_body::Data::TokenBurn(data))
                }
                transaction_body::Data::TokenWipe(data) => {
                    Some(schedulable_transaction_body::Data::TokenWipe(data))
                }
                transaction_body::Data::TokenAssociate(data) => {
                    Some(schedulable_transaction_body::Data::TokenAssociate(data))
                }
                transaction_body::Data::TokenDissociate(data) => {
                    Some(schedulable_transaction_body::Data::TokenDissociate(data))
                }
                transaction_body::Data::TokenFeeScheduleUpdate(data) => {
                    Some(schedulable_transaction_body::Data::TokenFeeScheduleUpdate(data))
                }
                transaction_body::Data::TokenPause(data) => {
                    Some(schedulable_transaction_body::Data::TokenPause(data))
                }
                transaction_body::Data::TokenUnpause(data) => {
                    Some(schedulable_transaction_body::Data::TokenUnpause(data))
                }
                transaction_body::Data::ScheduleCreate(_) => {
                    // NOTE: cannot schedule a ScheduleCreate transaction
                    None
                }
                transaction_body::Data::ScheduleDelete(data) => {
                    Some(schedulable_transaction_body::Data::ScheduleDelete(data))
                }
                transaction_body::Data::ScheduleSign(_) => {
                    // NOTE: cannot schedule a ScheduleSign transaction
                    None
                }
                transaction_body::Data::ContractCall(data) => {
                    Some(schedulable_transaction_body::Data::ContractCall(data))
                }
                // TODO: implement these
                Data::NodeStakeUpdate(_) => {
                    unimplemented!("NodeStakeUpdate has not been implemented")
                }
                Data::UtilPrng(_) => {
                    unimplemented!("UtilPrng has not been implemented")
                }
            };

            services::SchedulableTransactionBody {
                data,
                memo: scheduled.transaction_memo.clone(),
                // FIXME: does not use the client to default the max transaction fee
                transaction_fee: scheduled
                    .max_transaction_fee
                    .unwrap_or_else(|| scheduled.data.default_max_transaction_fee())
                    .to_tinybars() as u64,
            }
        });

        let payer_account_id = self.payer_account_id.to_protobuf();
        let admin_key = self.admin_key.to_protobuf();
        let expiration_time = self.expiration_time.map(Into::into);

        transaction_body::Data::ScheduleCreate(services::ScheduleCreateTransactionBody {
            scheduled_transaction_body: body,
            memo: self.schedule_memo.clone().unwrap_or_default(),
            admin_key,
            payer_account_id,
            expiration_time,
            wait_for_expiry: self.wait_for_expiry,
        })
    }
}

impl From<ScheduleCreateTransactionData> for AnyTransactionData {
    fn from(transaction: ScheduleCreateTransactionData) -> Self {
        Self::ScheduleCreate(transaction)
    }
}

impl FromProtobuf<services::SchedulableTransactionBody> for SchedulableTransactionBody {
    fn from_protobuf(pb: services::SchedulableTransactionBody) -> crate::Result<Self> {
        use schedulable_transaction_body::Data;
        let data = pb_getf!(pb, data)?;
        let data = match data {
            Data::ContractCall(it) => transaction_body::Data::ContractCall(it),
            Data::ContractCreateInstance(it) => transaction_body::Data::ContractCreateInstance(it),
            Data::ContractUpdateInstance(it) => transaction_body::Data::ContractUpdateInstance(it),
            Data::ContractDeleteInstance(it) => transaction_body::Data::ContractDeleteInstance(it),
            Data::CryptoApproveAllowance(it) => transaction_body::Data::CryptoApproveAllowance(it),
            Data::CryptoDeleteAllowance(it) => transaction_body::Data::CryptoDeleteAllowance(it),
            Data::CryptoCreateAccount(it) => transaction_body::Data::CryptoCreateAccount(it),
            Data::CryptoDelete(it) => transaction_body::Data::CryptoDelete(it),
            Data::CryptoTransfer(it) => transaction_body::Data::CryptoTransfer(it),
            Data::CryptoUpdateAccount(it) => transaction_body::Data::CryptoUpdateAccount(it),
            Data::FileAppend(it) => transaction_body::Data::FileAppend(it),
            Data::FileCreate(it) => transaction_body::Data::FileCreate(it),
            Data::FileDelete(it) => transaction_body::Data::FileDelete(it),
            Data::FileUpdate(it) => transaction_body::Data::FileUpdate(it),
            Data::SystemDelete(it) => transaction_body::Data::SystemDelete(it),
            Data::SystemUndelete(it) => transaction_body::Data::SystemUndelete(it),
            Data::Freeze(it) => transaction_body::Data::Freeze(it),
            Data::ConsensusCreateTopic(it) => transaction_body::Data::ConsensusCreateTopic(it),
            Data::ConsensusUpdateTopic(it) => transaction_body::Data::ConsensusUpdateTopic(it),
            Data::ConsensusDeleteTopic(it) => transaction_body::Data::ConsensusDeleteTopic(it),
            Data::ConsensusSubmitMessage(it) => transaction_body::Data::ConsensusSubmitMessage(it),
            Data::TokenCreation(it) => transaction_body::Data::TokenCreation(it),
            Data::TokenFreeze(it) => transaction_body::Data::TokenFreeze(it),
            Data::TokenUnfreeze(it) => transaction_body::Data::TokenUnfreeze(it),
            Data::TokenGrantKyc(it) => transaction_body::Data::TokenGrantKyc(it),
            Data::TokenRevokeKyc(it) => transaction_body::Data::TokenRevokeKyc(it),
            Data::TokenDeletion(it) => transaction_body::Data::TokenDeletion(it),
            Data::TokenUpdate(it) => transaction_body::Data::TokenUpdate(it),
            Data::TokenMint(it) => transaction_body::Data::TokenMint(it),
            Data::TokenBurn(it) => transaction_body::Data::TokenBurn(it),
            Data::TokenWipe(it) => transaction_body::Data::TokenWipe(it),
            Data::TokenAssociate(it) => transaction_body::Data::TokenAssociate(it),
            Data::TokenDissociate(it) => transaction_body::Data::TokenDissociate(it),
            Data::TokenFeeScheduleUpdate(it) => transaction_body::Data::TokenFeeScheduleUpdate(it),
            Data::TokenPause(it) => transaction_body::Data::TokenPause(it),
            Data::TokenUnpause(it) => transaction_body::Data::TokenUnpause(it),
            Data::ScheduleDelete(it) => transaction_body::Data::ScheduleDelete(it),
            Data::UtilPrng(it) => transaction_body::Data::UtilPrng(it),
        };

        Ok(Self {
            data: Box::new(AnyTransactionData::from_protobuf(data)?),
            max_transaction_fee: Some(Hbar::from_tinybars(pb.transaction_fee as i64)),
            transaction_memo: pb.memo,
        })
    }
}

impl FromProtobuf<services::ScheduleCreateTransactionBody> for ScheduleCreateTransactionData {
    fn from_protobuf(pb: services::ScheduleCreateTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            scheduled_transaction: Option::from_protobuf(pb.scheduled_transaction_body)?,
            schedule_memo: Some(pb.memo),
            admin_key: Option::from_protobuf(pb.admin_key)?,
            payer_account_id: Option::from_protobuf(pb.payer_account_id)?,
            expiration_time: pb.expiration_time.map(Into::into),
            wait_for_expiry: pb.wait_for_expiry,
        })
    }
}
