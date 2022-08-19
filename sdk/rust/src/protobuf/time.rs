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

use hedera_proto::services;
use time::{
    Duration,
    OffsetDateTime,
};

use crate::ToProtobuf;

impl ToProtobuf for Duration {
    type Protobuf = services::Duration;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::Duration { seconds: self.whole_seconds() }
    }
}

impl ToProtobuf for OffsetDateTime {
    type Protobuf = services::Timestamp;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::Timestamp { seconds: self.unix_timestamp(), nanos: self.nanosecond() as i32 }
    }
}
