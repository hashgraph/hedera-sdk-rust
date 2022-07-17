use hedera_proto::services;
use serde::{
    Deserialize,
    Serialize,
};
use time::OffsetDateTime;

use crate::{
    AccountId,
    FromProtobuf,
    Key,
    ScheduleId,
    TransactionId,
};

// TODO: ledger_id
// TODO: scheduled_transaction
/// Response from [`ScheduleInfoQuery`][crate::ScheduleInfoQuery].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleInfo {
    /// The ID of the schedule for which information is requested.
    pub schedule_id: ScheduleId,

    /// The account that created the scheduled transaction.
    pub creator_account_id: AccountId,

    /// The account paying for the execution of the scheduled transaction.
    pub payer_account_id: Option<AccountId>,

    /// The signatories that have provided signatures so far for the schedule
    /// transaction.
    pub signatories: Vec<Key>,

    /// The key which is able to delete the schedule transaction if set.
    pub admin_key: Option<Key>,

    /// The transaction id that will be used in the record of the scheduled transaction (if
    /// it executes).
    pub scheduled_transaction_id: TransactionId,

    /// When set to true, the transaction will be evaluated for execution at `expiration_time`
    /// instead of when all required signatures are received.
    pub wait_for_expiry: bool,

    /// Publicly visible information about the Schedule entity.
    pub schedule_memo: String,

    /// The date and time the schedule transaction will expire
    pub expiration_time: Option<OffsetDateTime>,

    /// The time the schedule transaction was executed.
    pub executed_at: Option<OffsetDateTime>,

    /// The time the schedule transaction was deleted.
    pub deleted_at: Option<OffsetDateTime>,
}

impl FromProtobuf for ScheduleInfo {
    type Protobuf = services::response::Response;

    #[allow(deprecated)]
    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, ScheduleGetInfo, services::response::Response);
        let info = pb_getf!(response, schedule_info)?;
        let schedule_id = pb_getf!(info, schedule_id)?;
        let creator_account_id = pb_getf!(info, creator_account_id)?;
        let payer_account_id = info.payer_account_id.map(AccountId::from_protobuf).transpose()?;
        let admin_key = info.admin_key.map(Key::from_protobuf).transpose()?;

        let scheduled_transaction_id =
            TransactionId::from_protobuf(pb_getf!(info, scheduled_transaction_id)?)?;

        let signatories = info
            .signers
            .map(|kl| {
                kl.keys.into_iter().map(Key::from_protobuf).collect::<crate::Result<Vec<_>>>()
            })
            .transpose()?
            .unwrap_or_default();

        let (executed_at, deleted_at) = match info.data {
            Some(services::schedule_info::Data::DeletionTime(deleted)) => {
                (None, Some(deleted.into()))
            }

            Some(services::schedule_info::Data::ExecutionTime(executed)) => {
                (Some(executed.into()), None)
            }

            None => (None, None),
        };

        Ok(Self {
            schedule_id: ScheduleId::from_protobuf(schedule_id)?,
            executed_at,
            deleted_at,
            schedule_memo: info.memo,
            creator_account_id: AccountId::from_protobuf(creator_account_id)?,
            payer_account_id,
            expiration_time: info.expiration_time.map(Into::into),
            admin_key,
            scheduled_transaction_id,
            signatories,
            wait_for_expiry: info.wait_for_expiry,
        })
    }
}
