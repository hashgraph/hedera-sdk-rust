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

#![allow(non_camel_case_types)]
#![allow(clippy::default_trait_access, clippy::doc_markdown)]

#[cfg(feature = "time_0_3")]
mod time_0_3;

#[cfg(feature = "fraction")]
mod fraction;

// fixme: Do this, just, don't warn 70 times in generated code.
#[allow(clippy::derive_partial_eq_without_eq)]
pub mod services {
    tonic::include_proto!("proto");
}

// fixme: Do this, just, don't warn 70 times in generated code.
#[allow(clippy::derive_partial_eq_without_eq)]
pub mod mirror {
    tonic::include_proto!("mirror/com.hedera.mirror.api.proto");
}

// fixme: Do this, just, don't warn 70 times in generated code.
#[allow(clippy::derive_partial_eq_without_eq)]
pub mod streams {
    tonic::include_proto!("streams/proto");
}

impl Extend<services::NodeAddress> for services::NodeAddressBook {
    fn extend<T: IntoIterator<Item = services::NodeAddress>>(&mut self, iter: T) {
        self.node_address.extend(iter)
    }
}
