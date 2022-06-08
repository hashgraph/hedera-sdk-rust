use async_trait::async_trait;
use itertools::Itertools;
use tonic::transport::Channel;
use hedera_proto::services;
// TODO use appropriate service client here
// example: use hedera_proto::services::token_service_client::TokenServiceClient;
use hedera_proto::services::token_service_client::TokenServiceClient;
use serde_with::base64::Base64;
use serde_with::{serde_as, skip_serializing_none, TimestampNanoSeconds};

use crate::protobuf::ToProtobuf;
use crate::{AccountId, Key, Transaction, TransactionId};
use crate::duration::Duration;
use crate::timestamp::Timestamp;
use crate::token::custom_fees::CustomFee;
use crate::token::token_supply_type::TokenSupplyType;
use crate::token::token_type::TokenType;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};

pub type TokenCreateTransaction = Transaction<TokenCreateTransactionData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenCreateTransactionData {
    name: Option<String>,
    symbol: Option<String>,
    decimals: Option<u32>,
    initial_supply: Option<u64>,
    treasury_account_id: Option<AccountId>,
    admin_key: Option<Key>,
    kyc_key: Option<Key>,
    freeze_key: Option<Key>,
    wipe_key: Option<Key>,
    supply_key: Option<Key>,
    freeze_default: Option<bool>,
    expiration_time: Option<Timestamp>,
    auto_renew_account_id: Option<AccountId>,
    auto_renew_period: Option<Duration>,
    memo: Option<String>,
    token_type: Option<TokenType>,
    token_supply_type: Option<TokenSupplyType>,
    max_supply: Option<i64>,
    fee_schedule_key: Option<Key>,
    custom_fees: Vec<CustomFee>, //TODO
    pause_key: Option<Key>,
}

// impl TokenCreateTransaction {
//     //TODO implement setters for `TokenCreateTransaction`
// }

// #[async_trait]
// impl TransactionExecute for TokenCreateTransactionData {
//     async fn execute(
//         &self,
//         channel: Channel,
//         request: services::Transaction,
//     ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
//         // TODO apply appropriate transaction name here
//         // get names from [module]_service.proto
//         // example: TokenServiceClient::new(channel).associate_tokens(request).await
//         TokenServiceClient::new(channel).create_token(request).await
//     }
// }

// impl ToTransactionDataProtobuf for TokenCreateTransactionData {
//     fn to_transaction_data_protobuf(
//         &self,
//         _node_account_id: AccountId,
//         _transaction_id: &TransactionId,
//     ) -> services::transaction_body::Data {
//         // TODO convert TokenCreateTransactionData members to protobufs
//         // example: self.some_variable.as_ref().map(SomeStruct::to_protobuf);
//
//         // get names from `transaction_body.proto`
//         services::transaction_body::Data::TokenCreation(services::TokenCreateTransactionBody {
//             // TODO build according to protobuf
//         })
//     }
// }
//
// impl From<TokenCreateTransactionData> for AnyTransactionData {
//     fn from(transaction: TokenCreateTransactionData) -> Self {
//         Self::TokenCreate(transaction)
//     }
// }
