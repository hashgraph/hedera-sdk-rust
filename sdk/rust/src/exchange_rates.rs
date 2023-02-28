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
use time::OffsetDateTime;

use crate::protobuf::FromProtobuf;

/// The current and next exchange rates between [`Hbar`](crate::HbarUnit::Hbar) and USD-cents.
#[derive(Debug, Clone)]
pub struct ExchangeRates {
    /// The current exchange rate between [`Hbar`](crate::HbarUnit::Hbar) and USD-cents.
    pub current_rate: ExchangeRate,
    /// The next exchange rate between [`Hbar`](crate::HbarUnit::Hbar) and USD-cents.
    pub next_rate: ExchangeRate,
}

impl ExchangeRates {
    /// Create a new `ExchangeRates` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::from_bytes(bytes)
    }
}

impl FromProtobuf<services::ExchangeRateSet> for ExchangeRates {
    fn from_protobuf(pb: services::ExchangeRateSet) -> crate::Result<Self> {
        Ok(Self {
            current_rate: ExchangeRate::from_protobuf(pb_getf!(pb, current_rate)?)?,
            next_rate: ExchangeRate::from_protobuf(pb_getf!(pb, next_rate)?)?,
        })
    }
}

/// Denotes a conversion between Hbars and cents (USD).
#[derive(Debug, Clone)]
pub struct ExchangeRate {
    /// Denotes [`Hbar`](crate::HbarUnit::Hbar) equivalent to cents (USD).
    pub hbars: u32,

    /// Denotes cents (USD) equivalent to [`Hbar`](crate::HbarUnit::Hbar)
    pub cents: u32,

    /// Expiration time of this exchange rate.
    pub expiration_time: OffsetDateTime,

    /// Calculated exchange rate
    pub exchange_rate_in_cents: f64,
}

impl FromProtobuf<services::ExchangeRate> for ExchangeRate {
    fn from_protobuf(pb: services::ExchangeRate) -> crate::Result<Self> {
        let hbars = pb.hbar_equiv as u32;
        let cents = pb.cent_equiv as u32;

        Ok(Self {
            hbars,
            cents,
            expiration_time: pb_getf!(pb, expiration_time)?.into(),
            exchange_rate_in_cents: f64::from(cents) / f64::from(hbars),
        })
    }
}
