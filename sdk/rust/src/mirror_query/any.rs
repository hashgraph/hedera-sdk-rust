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

use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use futures_util::TryStreamExt;

use super::subscribe::MirrorQueryExecutable;
use crate::topic::TopicMessageQueryData;
use crate::{
    MirrorQuery,
    NodeAddress,
    NodeAddressBookQueryData,
    TopicMessage,
};

/// Represents any possible query to the mirror network.
pub type AnyMirrorQuery = MirrorQuery<AnyMirrorQueryData>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", tag = "$type"))]
pub enum AnyMirrorQueryData {
    NodeAddressBook(NodeAddressBookQueryData),
    TopicMessage(TopicMessageQueryData),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", tag = "$type"))]
pub enum AnyMirrorQueryMessage {
    NodeAddressBook(NodeAddress),
    TopicMessage(TopicMessage),
}

/// Represents the response of any possible query to the mirror network.
#[cfg_attr(feature = "ffi", derive(serde::Serialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", tag = "$type"))]
pub enum AnyMirrorQueryResponse {
    /// Response for `AnyMirrorQuery::NodeAddressBook`.
    NodeAddressBook(<NodeAddressBookQueryData as MirrorQueryExecutable>::Response),
    /// Response for `AnyMirrorQuery::TopicMessage`.
    TopicMessage(<TopicMessageQueryData as MirrorQueryExecutable>::Response),
}

impl MirrorQueryExecutable for AnyMirrorQueryData {
    type Item = AnyMirrorQueryMessage;

    type Response = AnyMirrorQueryResponse;

    type ItemStream<'a> = BoxStream<'a, crate::Result<Self::Item>>
        where
            Self: 'a;

    fn subscribe_with_optional_timeout<'a>(
        &self,
        params: &crate::mirror_query::MirrorQueryCommon,
        client: &'a crate::Client,
        timeout: Option<std::time::Duration>,
    ) -> Self::ItemStream<'a>
    where
        Self: 'a,
    {
        match self {
            AnyMirrorQueryData::NodeAddressBook(it) => Box::pin(
                it.subscribe_with_optional_timeout(params, client, timeout)
                    .map_ok(Self::Item::from),
            ),
            AnyMirrorQueryData::TopicMessage(it) => Box::pin(
                it.subscribe_with_optional_timeout(params, client, timeout)
                    .map_ok(Self::Item::from),
            ),
        }
    }

    fn execute_with_optional_timeout<'a>(
        &'a self,
        params: &'a super::MirrorQueryCommon,
        client: &'a crate::Client,
        timeout: Option<std::time::Duration>,
    ) -> BoxFuture<'a, crate::Result<Self::Response>> {
        match self {
            AnyMirrorQueryData::NodeAddressBook(it) => Box::pin(async move {
                it.execute_with_optional_timeout(params, client, timeout)
                    .await
                    .map(Self::Response::from)
            }),
            AnyMirrorQueryData::TopicMessage(it) => Box::pin(async move {
                it.execute_with_optional_timeout(params, client, timeout)
                    .await
                    .map(Self::Response::from)
            }),
        }
    }
}

// NOTE: as we cannot derive serde on MirrorQuery<T> directly as `T`,
//  we create a proxy type that has the same layout but is only for AnyMirrorQueryData and does
//  derive(Deserialize).

#[cfg(feature = "ffi")]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
struct AnyMirrorQueryProxy {
    #[cfg_attr(feature = "ffi", serde(flatten))]
    data: AnyMirrorQueryData,

    #[cfg_attr(feature = "ffi", serde(flatten))]
    common: super::MirrorQueryCommon,
}

#[cfg(feature = "ffi")]
impl<D> serde::Serialize for MirrorQuery<D>
where
    D: MirrorQueryExecutable + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO: remove the clones, should be possible with Cows

        AnyMirrorQueryProxy { data: self.data.clone().into(), common: self.common.clone() }
            .serialize(serializer)
    }
}

#[cfg(feature = "ffi")]
impl<'de> serde::Deserialize<'de> for AnyMirrorQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <AnyMirrorQueryProxy as serde::Deserialize>::deserialize(deserializer)
            .map(|query| Self { data: query.data, common: query.common })
    }
}
