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

use time_0_2::{
    Duration,
    OffsetDateTime,
};

impl From<super::services::Duration> for Duration {
    fn from(pb: super::services::Duration) -> Self {
        Self::seconds(pb.seconds)
    }
}

impl From<Duration> for super::services::Duration {
    fn from(duration: Duration) -> Self {
        Self { seconds: duration.whole_seconds() }
    }
}

impl From<super::services::TimestampSeconds> for OffsetDateTime {
    fn from(pb: super::services::TimestampSeconds) -> Self {
        OffsetDateTime::from_unix_timestamp(pb.seconds)
    }
}

impl From<OffsetDateTime> for super::services::TimestampSeconds {
    fn from(dt: OffsetDateTime) -> Self {
        Self { seconds: dt.unix_timestamp() }
    }
}

impl From<super::services::Timestamp> for OffsetDateTime {
    fn from(pb: super::services::Timestamp) -> Self {
        OffsetDateTime::from_unix_timestamp(pb.seconds) + Duration::nanoseconds(pb.nanos.into())
    }
}

impl From<OffsetDateTime> for super::services::Timestamp {
    fn from(dt: OffsetDateTime) -> Self {
        let unix = dt - OffsetDateTime::unix_epoch();

        Self { seconds: unix.whole_seconds(), nanos: unix.subsec_nanoseconds() }
    }
}
