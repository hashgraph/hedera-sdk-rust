use async_trait::async_trait;
use hedera_proto::services;
use serde::{Deserialize, Deserializer};
use tonic::transport::Channel;

use super::ToQueryProtobuf;
use crate::account::{AccountBalanceQueryData, AccountInfoQueryData};
use crate::query::payment_transaction::PaymentTransaction;
use crate::query::QueryExecute;
use crate::{AccountBalance, AccountInfo, FromProtobuf, Query};

pub type AnyQuery = Query<AnyQueryData>;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AnyQueryData {
    AccountBalance(AccountBalanceQueryData),
    AccountInfo(AccountInfoQueryData),
}

#[derive(Debug, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AnyQueryResponse {
    AccountBalance(AccountBalance),
    AccountInfo(AccountInfo),
}

impl ToQueryProtobuf for AnyQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        match self {
            Self::AccountBalance(data) => data.to_query_protobuf(header),
            Self::AccountInfo(data) => data.to_query_protobuf(header),
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
            CryptoGetInfo(_) => Self::AccountInfo(AccountInfo::from_protobuf(response)?),
            CryptogetAccountBalance(_) => {
                Self::AccountBalance(AccountBalance::from_protobuf(response)?)
            }

            _ => todo!(),
        })
    }
}

// NOTE: as we cannot derive Deserialize on Query<T> directly as `T` is not Deserialize,
//  we create a proxy type that has the same layout but is only for AnyQueryData and does
//  derive(Deserialize).

#[derive(serde::Deserialize, Debug)]
struct AnyQueryProxy {
    #[serde(flatten)]
    data: AnyQueryData,
    // TODO: payment: Option<PaymentTransaction>
}

impl<'de> Deserialize<'de> for AnyQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        <AnyQueryProxy as Deserialize>::deserialize(deserializer)
            .inspect(|query| {
                log::trace!("wtf, {:#?}", query);
            })
            .map(|query| Self { data: query.data, payment: PaymentTransaction::default() })
    }
}
