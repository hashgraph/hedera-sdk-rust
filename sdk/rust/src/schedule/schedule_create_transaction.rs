use async_trait::async_trait;
use hedera_proto::services::schedule_service_client::ScheduleServiceClient;
use hedera_proto::services::{
    self,
    schedulable_transaction_body,
    transaction_body,
};
use serde::{
    Deserialize,
    Serialize,
};
use time::OffsetDateTime;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    Key,
    Transaction,
    TransactionId,
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

#[derive(Serialize, Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleCreateTransactionData {
    scheduled_transaction: Option<SchedulableTransactionBody>,

    schedule_memo: Option<String>,

    admin_key: Option<Key>,

    payer_account_id: Option<AccountId>,

    expiration_time: Option<OffsetDateTime>,

    wait_for_expiry: bool,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SchedulableTransactionBody {
    #[serde(flatten)]
    data: Box<AnyTransactionData>,

    #[serde(default)]
    max_transaction_fee: Option<u64>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    transaction_memo: String,
}

impl ScheduleCreateTransaction {
    /// Sets the scheduled transaction.
    pub fn scheduled_transaction<D>(&mut self, transaction: Transaction<D>) -> &mut Self
    where
        D: TransactionExecute,
    {
        self.body.data.scheduled_transaction = Some(SchedulableTransactionBody {
            max_transaction_fee: transaction.body.max_transaction_fee,
            transaction_memo: transaction.body.transaction_memo,
            data: Box::new(transaction.body.data.into()),
        });

        self
    }

    /// Sets the timestamp for when the transaction should be evaluated for execution and then expire.
    pub fn expiration_time(&mut self, time: OffsetDateTime) -> &mut Self {
        self.body.data.expiration_time = Some(time);
        self
    }

    /// Sets if the transaction will be evaluated for execution at expiration_time instead
    /// of when all required signatures are received.
    pub fn wait_for_expiry(&mut self, wait: bool) -> &mut Self {
        self.body.data.wait_for_expiry = wait;
        self
    }

    // /// Sets the id of the account to be charged the service fee for the scheduled transaction at
    // /// the consensus time that it executes (if ever).
    // TODO: pub fn payer_account_id(&mut self, id: AccountId) -> &mut Self {
    //     self.body.data.payer_account_id = Some(id);
    //     self
    // }

    /// Sets the memo for the schedule entity.
    pub fn schedule_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.body.data.schedule_memo = Some(memo.into());
        self
    }

    /// Sets the Hedera key which can be used to sign a ScheduleDelete and remove the schedule.
    pub fn admin_key(&mut self, key: impl Into<Key>) -> &mut Self {
        self.body.data.admin_key = Some(key.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for ScheduleCreateTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        ScheduleServiceClient::new(channel).create_schedule(request).await
    }
}

impl ToTransactionDataProtobuf for ScheduleCreateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        node_account_id: AccountId,
        transaction_id: &TransactionId,
    ) -> transaction_body::Data {
        let body = self.scheduled_transaction.as_ref().map(|scheduled| {
            let data = scheduled.data.to_transaction_data_protobuf(node_account_id, transaction_id);
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
            };

            services::SchedulableTransactionBody {
                data,
                memo: scheduled.transaction_memo.clone(),
                // FIXME: does not use the client to default the max transaction fee
                transaction_fee: scheduled
                    .max_transaction_fee
                    .unwrap_or_else(|| scheduled.data.default_max_transaction_fee()),
            }
        });

        let payer_account_id = self.payer_account_id.as_ref().map(AccountId::to_protobuf);
        let admin_key = self.admin_key.as_ref().map(Key::to_protobuf);
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
