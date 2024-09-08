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
use std::error::Error;

use serde::de::StdError;

/// Punches through all the layers of `tonic::Status` sources to check if this is a `hyper::Error` that is canceled.
pub(super) fn is_hyper_canceled(status: &tonic::Status) -> bool {
    let source = status
        .source()
        .and_then(|it| it.downcast_ref::<tonic::transport::Error>())
        .and_then(StdError::source);

    let Some(source) = source else {
        return false;
    };

    if let Some(hyper_0) = source.downcast_ref::<hyper_0::Error>() {
        // tonic 0.11 (current dependency)
        hyper_0.is_canceled()
    } else if let Some(hyper_1) = source.downcast_ref::<hyper::Error>() {
        // tonic 0.12
        hyper_1.is_canceled()
    } else {
        false
    }
}

/// Tests some non-detection scenarios.
///
/// Because hyper does not expose constructors for its error variants, there is no
/// reasonable way to construct a test for positive detection of a hyper cancellation.
#[cfg(test)]
mod test_is_hyper_canceled {
    use tonic::Code;

    use super::is_hyper_canceled;

    #[test]
    fn ignores_tonic_abort() {
        let input = tonic::Status::new(Code::Aborted, "foo");

        assert!(!is_hyper_canceled(&input));
    }

    #[test]
    fn ignores_tonic_cancel() {
        let input = tonic::Status::new(Code::Cancelled, "foo");

        assert!(!is_hyper_canceled(&input));
    }
}

pub(super) fn is_tonic_status_transient(status: &tonic::Status) -> bool {
    let source = status
        .source()
        .and_then(|it| it.downcast_ref::<tonic::transport::Error>())
        .and_then(StdError::source);

    let Some(source) = source else {
        return false;
    };

    if let Some(hyper_0) = source.downcast_ref::<hyper_0::Error>() {
        // tonic 0.11 (current dependency)
        let source: Option<&h2_03::Error> = hyper_0.source().and_then(|s| s.downcast_ref());

        source.map(|s| s.is_go_away()).unwrap_or_default()
    } else if let Some(hyper_1) = source.downcast_ref::<hyper::Error>() {
        // tonic 0.12
        let source: Option<&h2::Error> = hyper_1.source().and_then(|s| s.downcast_ref());

        source.map(|s| s.is_go_away()).unwrap_or_default()
    } else {
        false
    }
}
