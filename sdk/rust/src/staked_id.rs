use crate::entity_id::AutoValidateChecksum;
use crate::AccountId;

// no rename all, because each field is renamed
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
pub(crate) enum StakedId {
    #[cfg_attr(feature = "ffi", serde(rename = "stakedAccountId"))]
    AccountId(AccountId),
    #[cfg_attr(feature = "ffi", serde(rename = "stakedNodeId"))]
    NodeId(u64),
}

impl StakedId {
    pub(crate) fn to_account_id(&self) -> Option<AccountId> {
        match self {
            StakedId::AccountId(it) => Some(*it),
            StakedId::NodeId(_) => None,
        }
    }

    pub(crate) fn to_node_id(&self) -> Option<u64> {
        match self {
            StakedId::NodeId(it) => Some(*it),
            StakedId::AccountId(_) => None,
        }
    }
}

impl AutoValidateChecksum for StakedId {
    fn validate_checksum_for_ledger_id(
        &self,
        ledger_id: &crate::LedgerId,
    ) -> Result<(), crate::Error> {
        self.to_account_id().validate_checksum_for_ledger_id(ledger_id)
    }
}

impl From<AccountId> for StakedId {
    fn from(v: AccountId) -> Self {
        Self::AccountId(v)
    }
}

impl From<u64> for StakedId {
    fn from(v: u64) -> Self {
        Self::NodeId(v)
    }
}
