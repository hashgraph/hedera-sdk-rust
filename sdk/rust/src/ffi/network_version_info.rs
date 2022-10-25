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

use std::{
    ptr,
    slice,
};

use crate::ffi::error::Error;
use crate::ffi::SemanticVersion;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetworkVersionInfo {
    /// Version of the protobuf schema in use by the network.
    protobuf_version: SemanticVersion,

    /// Version of the Hedera services in use by the network.
    services_version: SemanticVersion,
}

impl NetworkVersionInfo {
    unsafe fn to_rust(self) -> crate::NetworkVersionInfo {
        crate::NetworkVersionInfo {
            protobuf_version: unsafe { self.protobuf_version.to_rust() },
            services_version: unsafe { self.services_version.to_rust() },
        }
    }

    fn from_rust(info: crate::NetworkVersionInfo) -> Self {
        Self {
            protobuf_version: SemanticVersion::from_rust(info.protobuf_version),
            services_version: SemanticVersion::from_rust(info.services_version),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_network_version_info_from_bytes(
    bytes: *const u8,
    bytes_size: libc::size_t,
    info: *mut NetworkVersionInfo,
) -> Error {
    assert!(!bytes.is_null());
    assert!(!info.is_null());

    let bytes = unsafe { slice::from_raw_parts(bytes, bytes_size) };

    let parsed = ffi_try!(crate::NetworkVersionInfo::from_bytes(bytes));
    let parsed = NetworkVersionInfo::from_rust(parsed);

    unsafe {
        ptr::write(info, parsed);
    }

    Error::Ok
}

#[no_mangle]
pub unsafe extern "C" fn hedera_network_version_info_to_bytes(
    info: NetworkVersionInfo,
    buf: *mut *mut u8,
) -> libc::size_t {
    let info = unsafe { info.to_rust() };
    let bytes = info.to_bytes().into_boxed_slice();

    let bytes = Box::leak(bytes);
    let len = bytes.len();
    let bytes = bytes.as_mut_ptr();

    // safety: invariants promise that `buf` must be valid for writes.
    unsafe {
        ptr::write(buf, bytes);
    }

    len
}

#[cfg(test)]
mod tests {
    use super::{
        NetworkVersionInfo,
        SemanticVersion,
    };

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn semantic_version_size_align() {
        assert_eq!(std::mem::size_of::<SemanticVersion>(), 32);
        assert_eq!(std::mem::align_of::<SemanticVersion>(), 8);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn network_version_info_size_align() {
        assert_eq!(std::mem::size_of::<NetworkVersionInfo>(), 64);
        assert_eq!(std::mem::align_of::<NetworkVersionInfo>(), 8);
    }
}
