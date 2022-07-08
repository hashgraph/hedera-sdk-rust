use hedera_proto::services;
use time::{Duration, OffsetDateTime}; // timestamp

use crate::token::custom_fees::CustomFee;
use crate::{AccountId, FromProtobuf, Key, TokenId};

/// Response from [`TokenInfoQuery`][crate::TokenInfoQuery].
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub token_id: TokenId,
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub total_supply: u64,
    pub treasury: Option<AccountId>,
    pub admin_key: Option<Key>,
    pub kyc_key: Option<Key>,
    pub freeze_key: Option<Key>,
    pub wipe_key: Option<Key>,
    pub supply_key: Option<Key>,
    pub default_freeze_status: i32,
    pub default_kyc_status: i32,
    pub deleted: bool,
    pub auto_renew_account: Option<AccountId>,
    pub auto_renew_period: Option<Duration>,
    pub expiry: Option<OffsetDateTime>,
    pub memo: String,
    pub token_type: i32,
    pub supply_type: i32,
    pub max_supply: i64,
    pub fee_schedule_key: Option<Key>,
    pub custom_fees: Vec<CustomFee>,
    pub pause_key: Option<Key>,
    pub pause_status: i32, //TODO: Option<PauseStatus>
    pub ledger_id: Vec<u8>, //TODO: Option<LedgerId>
}

impl FromProtobuf for TokenInfo {
    type Protobuf = services::response::Response;

    #[allow(deprecated)]
    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, TokenGetInfo, services::response::Response);
        let info = pb_getf!(response, token_info)?;
        let token_id = pb_getf!(info, token_id)?;

        Ok(Self {
            token_id: TokenId::from_protobuf(token_id)?,
            name: info.name,
            symbol: info.symbol,
            decimals: info.decimals as u32,
            total_supply: info.total_supply as u64,
            treasury: info.treasury.map(AccountId::from_protobuf).transpose()?,
            admin_key: info.admin_key.map(Key::from_protobuf).transpose()?,
            kyc_key: info.kyc_key.map(Key::from_protobuf).transpose()?,
            freeze_key: info.freeze_key.map(Key::from_protobuf).transpose()?,
            wipe_key: info.wipe_key.map(Key::from_protobuf).transpose()?,
            supply_key: info.supply_key.map(Key::from_protobuf).transpose()?,
            default_freeze_status: info.default_freeze_status as i32,
            default_kyc_status: info.default_kyc_status as i32,
            deleted: info.deleted,
            // FIXME
            auto_renew_account: None,
            auto_renew_period: None,
            expiry: None,
            memo: info.memo,
            token_type: info.token_type as i32,
            supply_type: info.supply_type as i32,
            max_supply: info.max_supply as i64,
            fee_schedule_key: info.fee_schedule_key.map(Key::from_protobuf).transpose()?,
            custom_fees: info
                .custom_fees
                .into_iter()
                .map(CustomFee::from_protobuf)
                .collect::<Result<Vec<_>, _>>()?, //test this
            pause_key: info.pause_key.map(Key::from_protobuf).transpose()?,
            pause_status: info.pause_status as i32,
            ledger_id: info.ledger_id,
        })
    }
}
