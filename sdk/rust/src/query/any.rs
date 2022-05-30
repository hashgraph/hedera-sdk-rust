use async_trait::async_trait;
use hedera_proto::services;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tonic::transport::Channel;

use super::ToQueryProtobuf;
use crate::account::{AccountBalanceQueryData, AccountInfoQueryData};
use crate::query::payment_transaction::PaymentTransactionData;
use crate::query::QueryExecute;
use crate::transaction::AnyTransactionBody;
use crate::transaction_receipt_query::TransactionReceiptQueryData;
use crate::{
    AccountBalance, AccountInfo, FromProtobuf, Query, Transaction, TransactionReceiptResponse
};

/// Any possible query that may be executed on the Hedera network.
pub type AnyQuery = Query<AnyQueryData>;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AnyQueryData {
    AccountBalance(AccountBalanceQueryData),
    AccountInfo(AccountInfoQueryData),
    TransactionReceipt(TransactionReceiptQueryData),
}

#[derive(Debug, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AnyQueryResponse {
    AccountBalance(AccountBalance),
    AccountInfo(AccountInfo),
    TransactionReceipt(TransactionReceiptResponse),
}

impl ToQueryProtobuf for AnyQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        match self {
            Self::AccountBalance(data) => data.to_query_protobuf(header),
            Self::AccountInfo(data) => data.to_query_protobuf(header),
            Self::TransactionReceipt(data) => data.to_query_protobuf(header),
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
                Self::AccountBalance(AccountBalance::from_protobuf(response)?)
            }

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
