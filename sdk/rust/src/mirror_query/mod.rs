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

mod any;
mod subscribe;

pub(crate) use any::AnyMirrorQueryData;
pub use any::{
    AnyMirrorQuery,
    AnyMirrorQueryMessage,
    AnyMirrorQueryResponse,
};
pub(crate) use subscribe::{
    subscribe,
    MirrorRequest,
};

use self::subscribe::MirrorQueryExecutable;

/// A query that can be executed on the Hedera mirror network.
#[derive(Clone, Debug, Default)]
pub struct MirrorQuery<D> {
    pub(crate) data: D,
    // Field needs to exist even though it currently does nothing
    #[allow(dead_code)]
    pub(crate) common: MirrorQueryCommon,
}

// intentionally inaccessable despite publicity.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct MirrorQueryCommon {
    // empty for now
    // TODO: request_timeout
}

impl<D> MirrorQuery<D>
where
    D: MirrorQueryExecutable + Default,
{
    /// Create a new query ready for configuration and execution.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
