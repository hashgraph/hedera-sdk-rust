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

use async_trait::async_trait;
use hedera_proto::services;
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use tonic::transport::Channel;

use super::ToQueryProtobuf;
use crate::account::{
    AccountBalanceQueryData,
    AccountInfoQueryData,
    AccountRecordsQueryData,
    AccountStakersQueryData,
};
use crate::contract::{
    ContractBytecodeQueryData,
    ContractCallQueryData,
    ContractInfoQueryData,
};
use crate::file::{
    FileContentsQueryData,
    FileInfoQueryData,
};
use crate::query::payment_transaction::PaymentTransactionData;
use crate::query::QueryExecute;
use crate::schedule::ScheduleInfoQueryData;
use crate::token::{
    TokenInfoQueryData,
    TokenNftInfoQueryData,
};
use crate::topic::TopicInfoQueryData;
use crate::transaction::AnyTransactionBody;
use crate::transaction_receipt_query::TransactionReceiptQueryData;
use crate::{
    AccountBalance,
    AccountInfo,
    AllProxyStakers,
    ContractFunctionResult,
    ContractInfo,
    FileContentsResponse,
    FileInfo,
    FromProtobuf,
    NetworkVersionInfo,
    NetworkVersionInfoQueryData,
    Query,
    ScheduleInfo,
    TokenInfo,
    TokenNftInfo,
    TopicInfo,
    Transaction,
    TransactionReceipt,
    TransactionRecord,
    TransactionRecordQueryData,
};

/// Any possible query that may be executed on the Hedera network.
pub type AnyQuery = Query<AnyQueryData>;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase", tag = "$type")]
pub enum AnyQueryData {
    AccountBalance(AccountBalanceQueryData),
    AccountInfo(AccountInfoQueryData),
    AccountStakers(AccountStakersQueryData),
    AccountRecords(AccountRecordsQueryData),
    TransactionReceipt(TransactionReceiptQueryData),
    TransactionRecord(TransactionRecordQueryData),
    FileContents(FileContentsQueryData),
    FileInfo(FileInfoQueryData),
    ContractBytecode(ContractBytecodeQueryData),
    ContractCall(ContractCallQueryData),
    TokenInfo(TokenInfoQueryData),
    ContractInfo(ContractInfoQueryData),
    TokenNftInfo(TokenNftInfoQueryData),
    TopicInfo(TopicInfoQueryData),
    ScheduleInfo(ScheduleInfoQueryData),
    NetworkVersionInfo(NetworkVersionInfoQueryData),
}

#[derive(Debug, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "$type")]
pub enum AnyQueryResponse {
    AccountBalance(AccountBalance),
    AccountInfo(AccountInfo),
    AccountStakers(AllProxyStakers),
    AccountRecords(Vec<TransactionRecord>),
    TransactionReceipt(TransactionReceipt),
    TransactionRecord(TransactionRecord),
    FileContents(FileContentsResponse),
    FileInfo(FileInfo),
    ContractBytecode(Vec<u8>),
    ContractCall(ContractFunctionResult),
    TokenInfo(TokenInfo),
    TopicInfo(TopicInfo),
    ContractInfo(ContractInfo),
    TokenNftInfo(TokenNftInfo),
    ScheduleInfo(ScheduleInfo),
    NetworkVersionInfo(NetworkVersionInfo),
}

impl ToQueryProtobuf for AnyQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        match self {
            Self::AccountBalance(data) => data.to_query_protobuf(header),
            Self::AccountInfo(data) => data.to_query_protobuf(header),
            Self::AccountStakers(data) => data.to_query_protobuf(header),
            Self::AccountRecords(data) => data.to_query_protobuf(header),
            Self::TransactionReceipt(data) => data.to_query_protobuf(header),
            Self::TransactionRecord(data) => data.to_query_protobuf(header),
            Self::FileContents(data) => data.to_query_protobuf(header),
            Self::FileInfo(data) => data.to_query_protobuf(header),
            Self::ContractBytecode(data) => data.to_query_protobuf(header),
            Self::ContractCall(data) => data.to_query_protobuf(header),
            Self::ContractInfo(data) => data.to_query_protobuf(header),
            Self::TokenNftInfo(data) => data.to_query_protobuf(header),
            Self::TokenInfo(data) => data.to_query_protobuf(header),
            Self::TopicInfo(data) => data.to_query_protobuf(header),
            Self::ScheduleInfo(data) => data.to_query_protobuf(header),
            Self::NetworkVersionInfo(data) => data.to_query_protobuf(header),
        }
    }
}

#[async_trait]
impl QueryExecute for AnyQueryData {
    type Response = AnyQueryResponse;

    fn is_payment_required(&self) -> bool {
        match self {
            Self::AccountInfo(query) => query.is_payment_required(),
            Self::AccountBalance(query) => query.is_payment_required(),
            Self::AccountStakers(query) => query.is_payment_required(),
            Self::AccountRecords(query) => query.is_payment_required(),
            Self::TransactionReceipt(query) => query.is_payment_required(),
            Self::TransactionRecord(query) => query.is_payment_required(),
            Self::FileContents(query) => query.is_payment_required(),
            Self::FileInfo(query) => query.is_payment_required(),
            Self::ContractBytecode(query) => query.is_payment_required(),
            Self::ContractCall(query) => query.is_payment_required(),
            Self::ContractInfo(query) => query.is_payment_required(),
            Self::TokenNftInfo(query) => query.is_payment_required(),
            Self::TokenInfo(query) => query.is_payment_required(),
            Self::TopicInfo(query) => query.is_payment_required(),
            Self::ScheduleInfo(query) => query.is_payment_required(),
            Self::NetworkVersionInfo(query) => query.is_payment_required(),
        }
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        match self {
            Self::AccountInfo(query) => query.execute(channel, request).await,
            Self::AccountBalance(query) => query.execute(channel, request).await,
            Self::AccountStakers(query) => query.execute(channel, request).await,
            Self::AccountRecords(query) => query.execute(channel, request).await,
            Self::TransactionReceipt(query) => query.execute(channel, request).await,
            Self::TransactionRecord(query) => query.execute(channel, request).await,
            Self::FileContents(query) => query.execute(channel, request).await,
            Self::FileInfo(query) => query.execute(channel, request).await,
            Self::ContractBytecode(query) => query.execute(channel, request).await,
            Self::ContractCall(query) => query.execute(channel, request).await,
            Self::ContractInfo(query) => query.execute(channel, request).await,
            Self::TokenNftInfo(query) => query.execute(channel, request).await,
            Self::TokenInfo(query) => query.execute(channel, request).await,
            Self::TopicInfo(query) => query.execute(channel, request).await,
            Self::ScheduleInfo(query) => query.execute(channel, request).await,
            Self::NetworkVersionInfo(query) => query.execute(channel, request).await,
        }
    }

    fn should_retry_pre_check(&self, status: crate::Status) -> bool {
        match self {
            Self::AccountInfo(query) => query.should_retry_pre_check(status),
            Self::AccountBalance(query) => query.should_retry_pre_check(status),
            Self::AccountStakers(query) => query.should_retry_pre_check(status),
            Self::AccountRecords(query) => query.should_retry_pre_check(status),
            Self::TransactionReceipt(query) => query.should_retry_pre_check(status),
            Self::TransactionRecord(query) => query.should_retry_pre_check(status),
            Self::FileContents(query) => query.should_retry_pre_check(status),
            Self::FileInfo(query) => query.should_retry_pre_check(status),
            Self::ContractBytecode(query) => query.should_retry_pre_check(status),
            Self::ContractCall(query) => query.should_retry_pre_check(status),
            Self::ContractInfo(query) => query.should_retry_pre_check(status),
            Self::TokenNftInfo(query) => query.should_retry_pre_check(status),
            Self::TokenInfo(query) => query.should_retry_pre_check(status),
            Self::TopicInfo(query) => query.should_retry_pre_check(status),
            Self::ScheduleInfo(query) => query.should_retry_pre_check(status),
            Self::NetworkVersionInfo(query) => query.should_retry_pre_check(status),
        }
    }

    fn should_retry(&self, response: &services::Response) -> bool {
        match self {
            Self::AccountInfo(query) => query.should_retry(response),
            Self::AccountBalance(query) => query.should_retry(response),
            Self::AccountStakers(query) => query.should_retry(response),
            Self::AccountRecords(query) => query.should_retry(response),
            Self::TransactionReceipt(query) => query.should_retry(response),
            Self::TransactionRecord(query) => query.should_retry(response),
            Self::FileContents(query) => query.should_retry(response),
            Self::FileInfo(query) => query.should_retry(response),
            Self::ContractBytecode(query) => query.should_retry(response),
            Self::ContractCall(query) => query.should_retry(response),
            Self::ContractInfo(query) => query.should_retry(response),
            Self::TokenNftInfo(query) => query.should_retry(response),
            Self::TokenInfo(query) => query.should_retry(response),
            Self::TopicInfo(query) => query.should_retry(response),
            Self::ScheduleInfo(query) => query.should_retry(response),
            Self::NetworkVersionInfo(query) => query.should_retry(response),
        }
    }
}

impl FromProtobuf<services::response::Response> for AnyQueryResponse {
    fn from_protobuf(response: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        use services::response::Response::*;

        Ok(match response {
            TransactionGetReceipt(_) => {
                Self::TransactionReceipt(TransactionReceipt::from_protobuf(response)?)
            }
            CryptoGetInfo(_) => Self::AccountInfo(AccountInfo::from_protobuf(response)?),
            CryptogetAccountBalance(_) => {
                Self::AccountBalance(AccountBalance::from_protobuf(response)?)
            }
            FileGetContents(_) => {
                Self::FileContents(FileContentsResponse::from_protobuf(response)?)
            }
            ContractGetBytecodeResponse(_) => {
                Self::ContractBytecode(Vec::<u8>::from_protobuf(response)?)
            }
            ContractCallLocal(_) => {
                Self::ContractCall(ContractFunctionResult::from_protobuf(response)?)
            }
            ContractGetInfo(_) => Self::ContractInfo(ContractInfo::from_protobuf(response)?),
            TokenGetNftInfo(_) => Self::TokenNftInfo(TokenNftInfo::from_protobuf(response)?),
            ConsensusGetTopicInfo(_) => Self::TopicInfo(TopicInfo::from_protobuf(response)?),
            ScheduleGetInfo(_) => Self::ScheduleInfo(ScheduleInfo::from_protobuf(response)?),
            CryptoGetProxyStakers(_) => {
                Self::AccountStakers(AllProxyStakers::from_protobuf(response)?)
            }
            CryptoGetAccountRecords(_) => {
                Self::AccountRecords(Vec::<TransactionRecord>::from_protobuf(response)?)
            }
            TransactionGetRecord(_) => {
                Self::TransactionRecord(TransactionRecord::from_protobuf(response)?)
            }
            NetworkGetVersionInfo(_) => {
                Self::NetworkVersionInfo(NetworkVersionInfo::from_protobuf(response)?)
            }
            FileGetInfo(_) => Self::FileInfo(FileInfo::from_protobuf(response)?),
            TokenGetInfo(_) => Self::TokenInfo(TokenInfo::from_protobuf(response)?),
            TokenGetNftInfos(_) => Self::TokenNftInfo(TokenNftInfo::from_protobuf(response)?),
            // Unimplemented on hedera services
            TransactionGetFastRecord(_)
            | CryptoGetLiveHash(_)
            | GetBySolidityId(_)
            | TokenGetAccountNftInfos(_)
            | NetworkGetExecutionTime(_)
            | ContractGetRecordsResponse(_)
            | AccountDetails(_)
            | GetByKey(_) => unreachable!(),
        })
    }
}

// NOTE: as we cannot derive serde on Query<T> directly as `T`,
//  we create a proxy type that has the same layout but is only for AnyQueryData and does
//  derive(Deserialize).

#[derive(serde::Deserialize, serde::Serialize)]
struct AnyQueryProxy {
    #[serde(flatten)]
    data: AnyQueryData,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    payment: Option<AnyTransactionBody<PaymentTransactionData>>,
}

impl<D> Serialize for Query<D>
where
    D: QueryExecute,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // TODO: remove the clones, should be possible with Cows

        let payment = self.data.is_payment_required().then(|| self.payment.body.clone().into());

        AnyQueryProxy { payment, data: self.data.clone().into() }.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AnyQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        <AnyQueryProxy as Deserialize>::deserialize(deserializer).map(|query| Self {
            data: query.data,
            payment: Transaction {
                body: query.payment.map(Into::into).unwrap_or_default(),
                signers: Vec::new(),
            },
        })
    }
}
