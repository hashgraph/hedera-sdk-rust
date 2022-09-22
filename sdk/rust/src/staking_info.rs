use hedera_proto::services;
use serde::Serialize;
use serde_with::{
    serde_as,
    TimestampNanoSeconds,
};
use time::OffsetDateTime;

use crate::{
    AccountId,
    FromProtobuf,
    Hbar,
};

#[serde_as]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StakingInfo {
    /// If true, the contract declines receiving a staking reward. The default value is false.
    pub decline_staking_reward: bool,

    /// The staking period during which either the staking settings for this account or contract changed (such as starting
    /// staking or changing staked_node_id) or the most recent reward was earned, whichever is later. If this account or contract
    /// is not currently staked to a node, then this field is not set.
    #[serde_as(as = "Option<TimestampNanoSeconds>")]
    pub stake_period_start: Option<OffsetDateTime>,

    /// The amount in Hbar that will be received in the next reward situation.
    pub pending_reward: Hbar,

    /// The total of balance of all accounts staked to this account or contract.
    pub staked_to_me: Hbar,

    /// The account to which this account or contract is staking.
    pub staked_account_id: Option<AccountId>,

    /// The ID of the node this account or contract is staked to.
    pub staked_node_id: Option<u64>,
}

impl FromProtobuf<services::StakingInfo> for StakingInfo {
    fn from_protobuf(pb: services::StakingInfo) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut staked_account_id = None;
        let mut staked_node_id = None;

        match pb.staked_id {
            Some(services::staking_info::StakedId::StakedAccountId(id)) => {
                staked_account_id = Some(AccountId::from_protobuf(id)?);
            }

            Some(services::staking_info::StakedId::StakedNodeId(id)) => {
                staked_node_id = Some(id as u64);
            }

            None => {}
        }

        Ok(Self {
            decline_staking_reward: pb.decline_reward,
            stake_period_start: pb.stake_period_start.map(Into::into),
            pending_reward: Hbar::from_tinybars(pb.pending_reward),
            staked_to_me: Hbar::from_tinybars(pb.staked_to_me),
            staked_account_id,
            staked_node_id,
        })
    }
}
