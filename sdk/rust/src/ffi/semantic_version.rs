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

use std::ffi::{
    c_char,
    CString,
};
use std::ptr;

use super::error::Error;
use crate::ffi::util::cstr_from_ptr;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SemanticVersion {
    /// Increases with incompatible API changes
    major: u32,

    /// Increases with backwards-compatible new functionality
    minor: u32,

    /// Increases with backwards-compatible bug fixes]
    patch: u32,

    /// A pre-release version MAY be denoted by appending a hyphen and a series of dot separated identifiers (https://semver.org/#spec-item-9);
    /// so given a semver 0.14.0-alpha.1+21AF26D3, this field would contain ‘alpha.1’
    ///
    /// treat `null` as an empty string.
    ///
    /// # Safety
    ///
    /// - If allocated by Hedera, must be freed with `hedera_string_free`,
    ///   notably this means that it must not be freed with `free`.
    /// - If *not* allocated by Hedera, must be freed however it normally would,
    ///   notably this means that it must not be freed with `hedera_string_free`
    /// - This field must be valid for reads (unless it's null)
    /// - If this is allocated by Hedera,
    ///   this will also be valid for writes *if* the field is non-null,
    ///   however, the length of this field must *not* be changed.
    prerelease: *mut c_char,

    /// Build metadata MAY be denoted by appending a plus sign and a series of dot separated identifiers
    /// immediately following the patch or pre-release version (https://semver.org/#spec-item-10);
    /// so given a semver 0.14.0-alpha.1+21AF26D3, this field would contain ‘21AF26D3’
    ///
    /// treat `null` as an empty string.
    ///
    /// # Safety
    ///
    /// - If allocated by Hedera, must be freed with `hedera_string_free`,
    ///   notably this means that it must not be freed with `free`.
    /// - If *not* allocated by Hedera, must be freed however it normally would,
    ///   notably this means that it must not be freed with `hedera_string_free`
    /// - This field must be valid for reads (unless it's null)
    /// - If this is allocated by Hedera,
    ///   this will also be valid for writes *if* the field is non-null,
    ///   however, the length of this field must *not* be changed.
    build: *mut c_char,
}

impl SemanticVersion {
    pub(super) fn from_rust(semver: crate::SemanticVersion) -> Self {
        fn string_to_ptr(string: String) -> *mut c_char {
            match string.is_empty() {
                true => ptr::null_mut(),
                false => CString::new(string).unwrap().into_raw(),
            }
        }

        let crate::SemanticVersion { major, minor, patch, prerelease, build } = semver;

        let prerelease = string_to_ptr(prerelease);
        let build = string_to_ptr(build);

        Self { major, minor, patch, prerelease, build }
    }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_semantic_version_from_string(
    s: *const c_char,
    semver: *mut SemanticVersion,
) -> Error {
    assert!(!s.is_null());
    assert!(!semver.is_null());

    let s = unsafe { cstr_from_ptr(s) };

    let parsed = ffi_try!(s.parse());
    let parsed = SemanticVersion::from_rust(parsed);

    unsafe {
        ptr::write(semver, parsed);
    }

    Error::Ok
}
