use std::fmt::{self, Formatter};

use hedera_proto::services;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use super::ToQueryProtobuf;
use crate::account::{AccountBalanceQueryData, AccountInfoQueryData};
use crate::query::payment_transaction::PaymentTransaction;
use crate::query::QueryData;
use crate::Query;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AnyQueryData {
    AccountBalance(AccountBalanceQueryData),
    AccountInfo(AccountInfoQueryData),
}

impl QueryData for AnyQueryData {}

impl ToQueryProtobuf for AnyQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        match self {
            Self::AccountBalance(data) => data.to_query_protobuf(header),
            Self::AccountInfo(data) => data.to_query_protobuf(header),
        }
    }
}

impl<'de> Deserialize<'de> for Query<AnyQueryData> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Data,
        }

        struct QueryVisitor;

        impl<'de> Visitor<'de> for QueryVisitor {
            type Value = Query<AnyQueryData>;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("struct Query<AnyQueryData>")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut data = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Data => {
                            if data.is_some() {
                                return Err(de::Error::duplicate_field("secs"));
                            }

                            data = Some(map.next_value()?);
                        }
                    }
                }

                let data = data.ok_or_else(|| de::Error::missing_field("secs"))?;

                // TODO: parse payment transaction from JSON
                Ok(Query { data, payment: PaymentTransaction::default() })
            }
        }

        deserializer.deserialize_struct("Query", &["data"], QueryVisitor)
    }
}
