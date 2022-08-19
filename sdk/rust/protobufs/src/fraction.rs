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

use fraction::Fraction;

impl From<super::services::Fraction> for Fraction {
    fn from(pb: super::services::Fraction) -> Self {
        Fraction::new(pb.numerator as u64, pb.denominator as u64)
    }
}

impl From<Fraction> for super::services::Fraction {
    fn from(frac: Fraction) -> Self {
        Self {
            numerator: frac.numer().copied().unwrap_or_default() as i64,
            denominator: frac.denom().copied().unwrap_or_default() as i64,
        }
    }
}
