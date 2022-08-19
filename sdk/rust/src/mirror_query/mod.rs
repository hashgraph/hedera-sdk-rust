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
    AnyMirrorQueryResponse,
};
pub(crate) use subscribe::MirrorQuerySubscribe;

/// A query that can be executed on the Hedera mirror network.
#[derive(Clone, Debug, Default)]
pub struct MirrorQuery<D>
where
    D: MirrorQuerySubscribe,
{
    pub(crate) data: D,
    // TODO: request_timeout
}

impl<D> MirrorQuery<D>
where
    D: MirrorQuerySubscribe + Default,
{
    /// Create a new query ready for configuration and execution.
    pub fn new() -> Self {
        Self::default()
    }
}
