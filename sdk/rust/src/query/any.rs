use async_trait::async_trait;
use hedera_proto::services;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tonic::transport::Channel;

use super::ToQueryProtobuf;
use crate::account::{AccountBalanceQueryData, AccountInfoQueryData};
use crate::contract::{ContractBytecodeQueryData, ContractCallQueryData, ContractInfoQueryData};
use crate::file::{FileContentsQueryData, FileInfoQueryData};
use crate::query::payment_transaction::PaymentTransactionData;
use crate::query::QueryExecute;
use crate::token::TokenNftInfoQueryData;
use crate::transaction::AnyTransactionBody;
use crate::transaction_receipt_query::TransactionReceiptQueryData;
use crate::{AccountBalanceResponse, AccountInfo, ContractBytecodeResponse, ContractCallResponse, ContractInfo, FileContentsResponse, FromProtobuf, Query, TokenNftInfoResponse, Transaction, TransactionReceiptResponse, FileInfo};

/// Any possible query that may be executed on the Hedera network.
pub type AnyQuery = Query<AnyQueryData>;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase", tag = "$type")]
pub enum AnyQueryData {
    AccountBalance(AccountBalanceQueryData),
    AccountInfo(AccountInfoQueryData),
    TransactionReceipt(TransactionReceiptQueryData),
    FileContents(FileContentsQueryData),
    FileInfo(FileInfoQueryData), //added
    ContractBytecode(ContractBytecodeQueryData),
    ContractCall(ContractCallQueryData),
    ContractInfo(ContractInfoQueryData),
    TokenNftInfo(TokenNftInfoQueryData),
}

#[derive(Debug, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "$type")]
pub enum AnyQueryResponse {
    AccountBalance(AccountBalanceResponse),
    AccountInfo(AccountInfo),
    TransactionReceipt(TransactionReceiptResponse),
    FileContents(FileContentsResponse),
    FileInfo(FileInfo),
    ContractBytecode(ContractBytecodeResponse),
    ContractCall(ContractCallResponse),
    ContractInfo(ContractInfo),
    TokenNftInfo(TokenNftInfoResponse),
}

impl ToQueryProtobuf for AnyQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        match self {
            Self::AccountBalance(data) => data.to_query_protobuf(header),
            Self::AccountInfo(data) => data.to_query_protobuf(header),
            Self::TransactionReceipt(data) => data.to_query_protobuf(header),
            Self::FileContents(data) => data.to_query_protobuf(header),
            Self::FileInfo(data) => data.to_query_protobuf(header),
            Self::ContractBytecode(data) => data.to_query_protobuf(header),
            Self::ContractCall(data) => data.to_query_protobuf(header),
            Self::ContractInfo(data) => data.to_query_protobuf(header),
            Self::TokenNftInfo(data) => data.to_query_protobuf(header),
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
            Self::TransactionReceipt(query) => query.is_payment_required(),
            Self::FileContents(query) => query.is_payment_required(),
            Self::FileInfo(query) => query.is_payment_required(),
            Self::ContractBytecode(query) => query.is_payment_required(),
            Self::ContractCall(query) => query.is_payment_required(),
            Self::ContractInfo(query) => query.is_payment_required(),
            Self::TokenNftInfo(query) => query.is_payment_required(),
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
            Self::TransactionReceipt(query) => query.execute(channel, request).await,
            Self::FileContents(query) => query.execute(channel, request).await,
            Self::FileInfo(query) => query.execute(channel, request).await,
            Self::ContractBytecode(query) => query.execute(channel, request).await,
            Self::ContractCall(query) => query.execute(channel, request).await,
            Self::ContractInfo(query) => query.execute(channel, request).await,
            Self::TokenNftInfo(query) => query.execute(channel, request).await,
        }
    }

    fn should_retry_pre_check(&self, status: crate::Status) -> bool {
        match self {
            Self::AccountInfo(query) => query.should_retry_pre_check(status),
            Self::AccountBalance(query) => query.should_retry_pre_check(status),
            Self::TransactionReceipt(query) => query.should_retry_pre_check(status),
            Self::FileContents(query) => query.should_retry_pre_check(status),
            Self::FileInfo(query) => query.should_retry_pre_check(status),
            Self::ContractBytecode(query) => query.should_retry_pre_check(status),
            Self::ContractCall(query) => query.should_retry_pre_check(status),
            Self::ContractInfo(query) => query.should_retry_pre_check(status),
            Self::TokenNftInfo(query) => query.should_retry_pre_check(status),
        }
    }

    fn should_retry(&self, response: &services::Response) -> bool {
        match self {
            Self::AccountInfo(query) => query.should_retry(response),
            Self::AccountBalance(query) => query.should_retry(response),
            Self::TransactionReceipt(query) => query.should_retry(response),
            Self::FileContents(query) => query.should_retry(response),
            Self::FileInfo(query) => query.should_retry(response),
            Self::ContractBytecode(query) => query.should_retry(response),
            Self::ContractCall(query) => query.should_retry(response),
            Self::ContractInfo(query) => query.should_retry(response),
            Self::TokenNftInfo(query) => query.should_retry(response),
        }
    }
}

impl FromProtobuf for AnyQueryResponse {
    type Protobuf = services::response::Response;

    fn from_protobuf(response: Self::Protobuf) -> crate::Result<Self>
    where
        Self: Sized,
    {
        use services::response::Response::*;

        Ok(match response {
            TransactionGetReceipt(_) => {
                Self::TransactionReceipt(TransactionReceiptResponse::from_protobuf(response)?)
            }
            CryptoGetInfo(_) => Self::AccountInfo(AccountInfo::from_protobuf(response)?),
            CryptogetAccountBalance(_) => {
                Self::AccountBalance(AccountBalanceResponse::from_protobuf(response)?)
            }
            FileGetContents(_) => {
                Self::FileContents(FileContentsResponse::from_protobuf(response)?)
            }
            ContractGetBytecodeResponse(_) => {
                Self::ContractBytecode(ContractBytecodeResponse::from_protobuf(response)?)
            }
            ContractCallLocal(_) => {
                Self::ContractCall(ContractCallResponse::from_protobuf(response)?)
            }
            ContractGetInfo(_) => Self::ContractInfo(ContractInfo::from_protobuf(response)?),
            TokenGetNftInfo(_) => Self::TokenNftInfo(TokenNftInfoResponse::from_protobuf(response)?),

            _ => todo!(),
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
