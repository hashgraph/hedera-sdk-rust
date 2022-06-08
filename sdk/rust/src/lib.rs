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
mod execute;
mod file;
mod key;
mod mirror_query;
mod node_address;
mod node_address_book_query;
mod query;
mod schedule;
mod signature;
mod signer;
mod token;
mod topic;
mod transaction;
mod transaction_hash;
mod transaction_id;
mod transaction_receipt;
mod transaction_receipt_query;
mod transaction_receipt_response;
mod transaction_response;
mod transfer_transaction;

#[cfg(feature = "ffi")]
mod ffi;

pub use account::{
    AccountAddress, AccountAlias, AccountBalanceQuery, AccountBalanceResponse, AccountCreateTransaction, AccountDeleteTransaction, AccountId, AccountInfo, AccountInfoQuery, AccountUpdateTransaction
};
pub use client::Client;
pub use contract::{ContractAddress, ContractEvmAddress, ContractId};
pub use entity_id::EntityId;
pub use error::{Error, Result};
pub use file::{
    FileAppendTransaction, FileContentsQuery, FileContentsResponse, FileCreateTransaction, FileDeleteTransaction, FileId, FileUpdateTransaction
};
pub use hedera_proto::services::ResponseCodeEnum as Status;
pub use key::{Key, PrivateKey, PublicKey};
pub use mirror_query::{AnyMirrorQuery, AnyMirrorQueryResponse, MirrorQuery};
pub use node_address::NodeAddress;
pub use node_address_book_query::NodeAddressBookQuery;
pub(crate) use node_address_book_query::NodeAddressBookQueryData;
pub use protobuf::{FromProtobuf, ToProtobuf};
pub use query::{AnyQuery, AnyQueryResponse, Query};
pub use schedule::ScheduleId;
pub use signature::{Signature, SignaturePair};
pub use signer::Signer;
pub use token::{
    TokenId,
    TokenAssociateTransaction,
    TokenBurnTransaction,
    TokenDeleteTransaction,
    TokenDissociateTransaction,
    TokenFreezeTransaction,
    TokenGrantKycTransaction,
    TokenPauseTransaction,
    TokenRevokeKycTransaction,
    TokenUnfreezeTransaction,
    TokenUnpauseTransaction,
    TokenWipeTransaction,
};
pub use topic::{
    TopicCreateTransaction, TopicDeleteTransaction, TopicId, TopicMessage, TopicMessageQuery, TopicMessageSubmitTransaction, TopicUpdateTransaction
};
pub use transaction::Transaction;
pub use transaction_hash::TransactionHash;
pub use transaction_id::TransactionId;
pub use transaction_receipt::TransactionReceipt;
pub use transaction_receipt_query::TransactionReceiptQuery;
pub use transaction_receipt_response::TransactionReceiptResponse;
pub use transaction_response::TransactionResponse;
pub use transfer_transaction::TransferTransaction;
