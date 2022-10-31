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
use prost::Message;
use serde::{
    Deserialize,
    Serialize,
};
use serde_with::serde_as;
use time::{
    Duration,
    OffsetDateTime,
};

use crate::protobuf::ToProtobuf;
use crate::{
    AccountId,
    ContractId,
    FromProtobuf,
    Key,
    LedgerId,
};

// TODO: staking_info
/// Current information on a smart contract instance.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfo {
    /// ID of the contract instance, in the format used by transactions.
    pub contract_id: ContractId,

    /// ID of the cryptocurrency account owned by the contract instance,
    /// in the format used in transactions.
    pub account_id: AccountId,

    /// ID of both the contract instance and the cryptocurrency account owned by the contract
    /// instance, in the format used by Solidity.
    pub contract_account_id: String,

    /// The admin key of the contract instance.
    pub admin_key: Option<Key>,

    /// The current time at which this contract instance (and its account) is set to expire.
    pub expiration_time: Option<OffsetDateTime>,

    /// The auto renew period for this contract instance.
    pub auto_renew_period: Option<Duration>,

    /// Number of bytes of storage being used by this instance.
    pub storage: u64,

    /// The memo associated with the contract.
    pub contract_memo: String,

    /// The current balance, in tinybars.
    pub balance: u64,

    /// Whether the contract has been deleted.
    pub is_deleted: bool,

    /// ID of the an account to charge for auto-renewal of this contract.
    pub auto_renew_account_id: Option<AccountId>,

    /// The maximum number of tokens that a contract can be implicitly associated with.
    pub max_automatic_token_associations: u32,

    /// The ledger ID the response was returned from
    pub ledger_id: LedgerId,
}

impl ContractInfo {
    /// Create a new `StakingInfo` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::<services::contract_get_info_response::ContractInfo>::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        services::contract_get_info_response::ContractInfo {
            contract_id: Some(self.contract_id.to_protobuf()),
            account_id: Some(self.account_id.to_protobuf()),
            contract_account_id: self.contract_account_id.clone(),
            admin_key: self.admin_key.as_ref().map(ToProtobuf::to_protobuf),
            expiration_time: self.expiration_time.as_ref().map(ToProtobuf::to_protobuf),
            auto_renew_period: self.auto_renew_period.as_ref().map(ToProtobuf::to_protobuf),
            storage: self.storage as i64,
            memo: self.contract_memo.clone(),
            balance: self.balance,
            deleted: self.is_deleted,
            ledger_id: self.ledger_id.to_bytes(),
            auto_renew_account_id: self.auto_renew_account_id.as_ref().map(ToProtobuf::to_protobuf),
            max_automatic_token_associations: self.max_automatic_token_associations as i32,

            // unimplemented fields
            token_relationships: Vec::new(),
            staking_info: None,
        }
        .encode_to_vec()
    }
}

impl FromProtobuf<services::response::Response> for ContractInfo {
    #[allow(deprecated)]
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, ContractGetInfo, services::response::Response);
        let info = pb_getf!(response, contract_info)?;
        Self::from_protobuf(info)
    }
}

impl FromProtobuf<services::contract_get_info_response::ContractInfo> for ContractInfo {
    #[allow(deprecated)]
    fn from_protobuf(pb: services::contract_get_info_response::ContractInfo) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let contract_id = pb_getf!(pb, contract_id)?;
        let account_id = pb_getf!(pb, account_id)?;
        let expiration_time = pb.expiration_time.map(Into::into);
        let auto_renew_period = pb.auto_renew_period.map(Into::into);
        let auto_renew_account_id =
            pb.auto_renew_account_id.map(AccountId::from_protobuf).transpose()?;
        let admin_key = pb.admin_key.map(Key::from_protobuf).transpose()?;
        let ledger_id = LedgerId::from_bytes(pb.ledger_id);

        Ok(Self {
            contract_id: ContractId::from_protobuf(contract_id)?,
            account_id: AccountId::from_protobuf(account_id)?,
            contract_account_id: pb.contract_account_id,
            is_deleted: pb.deleted,
            balance: pb.balance as u64,
            expiration_time,
            auto_renew_period,
            auto_renew_account_id,
            contract_memo: pb.memo,
            max_automatic_token_associations: pb.max_automatic_token_associations as u32,
            admin_key,
            storage: pb.storage as u64,
            ledger_id,
        })
    }
}
