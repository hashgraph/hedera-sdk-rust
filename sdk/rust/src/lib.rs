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

#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(absolute_paths_not_starting_with_crate)]
#![warn(clippy::cargo_common_metadata)]
#![warn(clippy::multiple_crate_versions)]
#![warn(clippy::pedantic)]
#![warn(clippy::future_not_send)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::enum_glob_use)]

#[macro_use]
mod protobuf;

mod account;
mod client;
mod contract;
mod entity_id;
mod error;
mod ethereum_transaction;
mod execute;
mod file;
mod key;
mod mirror_query;
mod network_version_info;
mod network_version_info_query;
mod node_address;
mod node_address_book_query;
mod query;
mod schedule;
mod signature;
mod signer;
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
    AccountStakersQuery,
    AccountUpdateTransaction,
    AllProxyStakers,
    ProxyStaker,
};
pub use client::Client;
pub use contract::{
    ContractBytecodeQuery,
    ContractCreateTransaction,
    ContractExecuteTransaction,
    ContractFunctionResult,
    ContractId,
    ContractInfo,
    ContractInfoQuery,
    ContractUpdateTransaction,
};
pub use entity_id::EntityId;
pub use error::{
    Error,
    Result,
};
pub use ethereum_transaction::EthereumTransaction;
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
pub use hedera_proto::services::ResponseCodeEnum as Status;
pub use key::{
    Key,
    PrivateKey,
    PublicKey,
};
pub use mirror_query::{
    AnyMirrorQuery,
    AnyMirrorQueryResponse,
    MirrorQuery,
};
pub use network_version_info::{
    NetworkVersionInfo,
    SemanticVersion,
};
pub use network_version_info_query::NetworkVersionInfoQuery;
pub(crate) use network_version_info_query::NetworkVersionInfoQueryData;
pub use node_address::NodeAddress;
pub use node_address_book_query::NodeAddressBookQuery;
pub(crate) use node_address_book_query::NodeAddressBookQueryData;
pub use protobuf::{
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
pub use signature::{
    Signature,
    SignaturePair,
};
pub use signer::Signer;
pub use system::{
    FreezeTransaction,
    FreezeType,
    SystemDeleteTransaction,
    SystemUndeleteTransaction,
};
pub use token::{
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
    TopicMessage,
    TopicMessageQuery,
    TopicMessageSubmitTransaction,
    TopicUpdateTransaction,
};
pub use transaction::Transaction;
pub use transaction_hash::TransactionHash;
pub use transaction_id::TransactionId;
pub use transaction_receipt::TransactionReceipt;
pub use transaction_receipt_query::TransactionReceiptQuery;
pub use transaction_record::TransactionRecord;
pub use transaction_record_query::TransactionRecordQuery;
pub(crate) use transaction_record_query::TransactionRecordQueryData;
pub use transaction_response::TransactionResponse;
pub use transfer_transaction::TransferTransaction;
