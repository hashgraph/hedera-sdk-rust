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
mod error;
mod execute;
mod file;
mod key;
mod query;
mod schedule;
mod serde;
mod signature;
mod signer;
mod token;
mod topic;
mod transaction;
mod transaction_hash;
mod transaction_id;
mod transaction_receipt;
mod transaction_receipt_query;
mod transaction_response;
mod transfer_transaction;

#[cfg(feature = "ffi")]
mod ffi;

pub use account::{
    AccountAlias, AccountBalance, AccountBalanceQuery, AccountCreateTransaction, AccountDeleteTransaction, AccountId, AccountIdOrAlias, AccountInfo, AccountInfoQuery, AccountUpdateTransaction
};
pub use client::Client;
pub use contract::{ContractEvmAddress, ContractId, ContractIdOrEvmAddress};
pub use error::{Error, Result};
pub use file::FileId;
pub use hedera_proto::services::ResponseCodeEnum as Status;
pub use key::{Key, PrivateKey, PublicKey};
pub use protobuf::{FromProtobuf, ToProtobuf};
pub use query::{AnyQuery, Query};
pub use schedule::ScheduleId;
pub use signature::{Signature, SignaturePair};
pub use signer::Signer;
pub use token::TokenId;
pub use topic::{
    TopicCreateTransaction, TopicDeleteTransaction, TopicId, TopicMessageSubmitTransaction, TopicUpdateTransaction
};
pub use transaction::Transaction;
pub use transaction_hash::TransactionHash;
pub use transaction_id::TransactionId;
pub use transaction_receipt::TransactionReceipt;
pub use transaction_receipt_query::TransactionReceiptQuery;
pub use transaction_response::TransactionResponse;
pub use transfer_transaction::TransferTransaction;
