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
use serde::{
    Deserialize,
    Serialize,
};
use serde_with::serde_as;
use time::{
    Duration,
    OffsetDateTime,
};

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

impl FromProtobuf<services::response::Response> for ContractInfo {
    #[allow(deprecated)]
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, ContractGetInfo, services::response::Response);
        let info = pb_getf!(response, contract_info)?;
        let contract_id = pb_getf!(info, contract_id)?;
        let account_id = pb_getf!(info, account_id)?;
        let expiration_time = info.expiration_time.map(Into::into);
        let auto_renew_period = info.auto_renew_period.map(Into::into);
        let auto_renew_account_id =
            info.auto_renew_account_id.map(AccountId::from_protobuf).transpose()?;
        let admin_key = info.admin_key.map(Key::from_protobuf).transpose()?;
        let ledger_id = LedgerId::from_bytes(info.ledger_id);

        Ok(Self {
            contract_id: ContractId::from_protobuf(contract_id)?,
            account_id: AccountId::from_protobuf(account_id)?,
            contract_account_id: info.contract_account_id,
            is_deleted: info.deleted,
            balance: info.balance as u64,
            expiration_time,
            auto_renew_period,
            auto_renew_account_id,
            contract_memo: info.memo,
            max_automatic_token_associations: info.max_automatic_token_associations as u32,
            admin_key,
            storage: info.storage as u64,
            ledger_id,
        })
    }
}
