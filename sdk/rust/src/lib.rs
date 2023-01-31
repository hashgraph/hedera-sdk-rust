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

// todo: more indepth documentation
//! Hedera Rust SDK.

#![forbid(unsafe_op_in_unsafe_fn)]
#![warn(
    absolute_paths_not_starting_with_crate,
    deprecated_in_future,
    future_incompatible,
    missing_docs,
    clippy::cargo_common_metadata,
    clippy::future_not_send,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions,
    clippy::pedantic
)]
#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::enum_glob_use,
    clippy::enum_variant_names,
    clippy::inline_always,
    clippy::match_bool,
    clippy::module_name_repetitions
)]

#[macro_use]
mod protobuf;

mod account;
mod client;
mod contract;
mod entity_id;
mod error;
mod ethereum_transaction;
mod evm_address;
mod execute;
mod file;
mod key;
mod ledger_id;
mod mirror_query;
mod mnemonic;
mod network_version_info;
mod network_version_info_query;
mod node_address;
mod node_address_book;
mod node_address_book_query;
mod query;
mod schedule;
mod semantic_version;
mod signer;
mod staked_id;
mod staking_info;
mod system;
mod token;
mod topic;
mod transaction;
mod transaction_hash;
mod transaction_id;
mod transaction_receipt;
mod transaction_receipt_query;
mod transaction_record;
mod transaction_record_query;
mod transaction_response;
mod transfer_transaction;

#[cfg(feature = "ffi")]
mod ffi;
mod hbar;
mod transfer;

pub use account::{
    AccountAllowanceApproveTransaction,
    AccountAllowanceDeleteTransaction,
    AccountBalance,
    AccountBalanceQuery,
    AccountCreateTransaction,
    AccountDeleteTransaction,
    AccountId,
    AccountInfo,
    AccountInfoQuery,
    AccountRecordsQuery,
    AccountStakersQuery,
    AccountUpdateTransaction,
    AllProxyStakers,
    ProxyStaker,
};
pub use client::Client;
pub(crate) use client::Operator;
pub use contract::{
    ContractBytecodeQuery,
    ContractCallQuery,
    ContractCreateTransaction,
    ContractExecuteTransaction,
    ContractFunctionParameters,
    ContractFunctionResult,
    ContractId,
    ContractInfo,
    ContractInfoQuery,
    ContractLogInfo,
    ContractUpdateTransaction,
};
pub use entity_id::EntityId;
pub(crate) use entity_id::ValidateChecksums;
pub use error::{
    Error,
    MnemonicEntropyError,
    MnemonicParseError,
    Result,
};
pub use ethereum_transaction::EthereumTransaction;
pub use evm_address::EvmAddress;
pub use file::{
    FileAppendTransaction,
    FileContentsQuery,
    FileContentsResponse,
    FileCreateTransaction,
    FileDeleteTransaction,
    FileId,
    FileInfo,
    FileInfoQuery,
    FileUpdateTransaction,
};
pub use hbar::{
    Hbar,
    HbarUnit,
    Tinybar,
};
pub use hedera_proto::services::ResponseCodeEnum as Status;
pub use key::{
    Key,
    KeyList,
    PrivateKey,
    PublicKey,
};
pub use ledger_id::LedgerId;
pub use mirror_query::{
    AnyMirrorQuery,
    AnyMirrorQueryResponse,
    MirrorQuery,
};
pub use mnemonic::Mnemonic;
pub use network_version_info::NetworkVersionInfo;
pub use network_version_info_query::NetworkVersionInfoQuery;
pub(crate) use network_version_info_query::NetworkVersionInfoQueryData;
pub use node_address::NodeAddress;
pub use node_address_book::NodeAddressBook;
pub use node_address_book_query::NodeAddressBookQuery;
pub(crate) use node_address_book_query::NodeAddressBookQueryData;
pub(crate) use protobuf::{
    FromProtobuf,
    ToProtobuf,
};
pub use query::{
    AnyQuery,
    AnyQueryResponse,
    Query,
};
pub use schedule::{
    ScheduleCreateTransaction,
    ScheduleDeleteTransaction,
    ScheduleId,
    ScheduleInfo,
    ScheduleInfoQuery,
    ScheduleSignTransaction,
};
pub use semantic_version::SemanticVersion;
pub use signer::Signer;
pub use staking_info::StakingInfo;
pub use system::{
    FreezeTransaction,
    FreezeType,
    SystemDeleteTransaction,
    SystemUndeleteTransaction,
};
pub use token::{
    AssessedCustomFee,
    NftId,
    TokenAssociateTransaction,
    TokenAssociation,
    TokenBurnTransaction,
    TokenCreateTransaction,
    TokenDeleteTransaction,
    TokenDissociateTransaction,
    TokenFeeScheduleUpdateTransaction,
    TokenFreezeTransaction,
    TokenGrantKycTransaction,
    TokenId,
    TokenInfo,
    TokenInfoQuery,
    TokenMintTransaction,
    TokenNftInfo,
    TokenNftInfoQuery,
    TokenNftTransfer,
    TokenPauseTransaction,
    TokenRevokeKycTransaction,
    TokenSupplyType,
    TokenType,
    TokenUnfreezeTransaction,
    TokenUnpauseTransaction,
    TokenUpdateTransaction,
    TokenWipeTransaction,
};
pub use topic::{
    TopicCreateTransaction,
    TopicDeleteTransaction,
    TopicId,
    TopicInfo,
    TopicInfoQuery,
    TopicMessage,
    TopicMessageQuery,
    TopicMessageSubmitTransaction,
    TopicUpdateTransaction,
};
pub use transaction::{
    AnyTransaction,
    Transaction,
};
pub use transaction_hash::TransactionHash;
pub use transaction_id::TransactionId;
pub use transaction_receipt::TransactionReceipt;
pub use transaction_receipt_query::TransactionReceiptQuery;
pub use transaction_record::TransactionRecord;
pub use transaction_record_query::TransactionRecordQuery;
pub(crate) use transaction_record_query::TransactionRecordQueryData;
pub use transaction_response::TransactionResponse;
pub use transfer::Transfer;
pub use transfer_transaction::TransferTransaction;
