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

/// Get an optional field from a protobuf object, returning an error if the field does not exist.
macro_rules! pb_getf {
    ($expr:expr, $field:ident) => {{
        $expr.$field.ok_or_else(|| {
            $crate::Error::from_protobuf(concat!("unexpected missing `", stringify!($field), "`"))
        })
    }};
}

/// Get a specific variant from a `oneof` field of a protobuf object, returning an error
/// if the variant is not as expected.
macro_rules! pb_getv {
    ($expr:expr, $variant:ident, $ty:ty) => {{
        use $ty::*;

        match $expr {
            $variant(it) => it,

            _ => {
                return Err($crate::Error::from_protobuf(format!(
                    concat!("unexpected {:?} received, expecting `", stringify!($variant), "`"),
                    $expr
                )));
            }
        }
    }};
}
