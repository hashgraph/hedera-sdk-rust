use hedera_proto::services;

use crate::{
    AccountId,
    FromProtobuf,
};

/// Response from [`AccountStakersQuery`][crate::AccountStakersQuery].
pub type AllProxyStakers = Vec<ProxyStaker>;

/// Information about a single account that is proxy staking.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyStaker {
    /// The Account ID that is proxy staking.
    pub account_id: AccountId,

    /// The number of hbars that are currently proxy staked.
    pub amount: u64,
}

impl FromProtobuf<services::response::Response> for AllProxyStakers {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, CryptoGetProxyStakers, services::response::Response);
        let stakers = pb_getf!(response, stakers)?;

        stakers
            .proxy_staker
            .into_iter()
            .map(ProxyStaker::from_protobuf)
            .collect::<crate::Result<Vec<_>>>()
    }
}

impl FromProtobuf<services::ProxyStaker> for ProxyStaker {
    fn from_protobuf(pb: services::ProxyStaker) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let account_id = pb_getf!(pb, account_id)?;

        Ok(Self { account_id: AccountId::from_protobuf(account_id)?, amount: pb.amount as u64 })
    }
}
