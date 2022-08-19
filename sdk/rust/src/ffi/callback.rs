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

use std::os::raw::c_void;

use crate::ffi::error::Error;

/// Wrapper around a C callback handle and an associated opaque context pointer.
pub(super) struct Callback<T> {
    handle: extern "C" fn(context: *const c_void, err: Error, value: T),
    context: *const c_void,
}

impl<T> Callback<T> {
    pub(super) fn new(
        context: *const c_void,
        handle: extern "C" fn(context: *const c_void, err: Error, value: T),
    ) -> Self {
        Self { handle, context }
    }

    pub(super) fn call(&self, err: Error, value: T) {
        (self.handle)(self.context, err, value)
    }
}

// NOTE: The context pointer is referring to state that is never inspected, only passed
unsafe impl<T> Send for Callback<T> {}
