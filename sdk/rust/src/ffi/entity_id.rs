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

use libc::size_t;

use crate::ffi::error::Error;
use crate::{
    EntityId,
    FileId,
    ScheduleId,
    TokenId,
    TopicId,
};

// note(sr): This abstraction is worthwhile because it reduces the amount of duplicated non-trival unsafe code.
// generics (on `FromProtobuf`/`ToProtobuf`) make it very hard to do this without this hack.
// Alternatively, lack of generics, on the inherent impls make it hard with out the hack.
trait FromToBytes {
    fn ffi_from_bytes(bytes: &[u8]) -> crate::Result<Self>
    where
        Self: Sized;
    fn ffi_to_bytes(&self) -> Box<[u8]>;
}

trait FromToEntityId {
    fn from_entity_id(id: EntityId) -> Self;
    fn to_entity_id(&self) -> EntityId;
}

macro_rules! impl_ffi_convert_traits_for {
    ($($type:ty),*$(,)?) => {
        $(
            impl FromToBytes for $type {
                fn ffi_from_bytes(bytes: &[u8]) -> crate::Result<Self> {
                    Self::from_bytes(bytes)
                }

                fn ffi_to_bytes(&self) -> Box<[u8]> {
                    self.to_bytes().into_boxed_slice()
                }
            }

            impl FromToEntityId for $type {
                fn from_entity_id(id: EntityId) -> Self {
                    let EntityId { shard, realm, num, checksum } = id;
                    Self { shard, realm, num, checksum }
                }

                fn to_entity_id(&self) -> EntityId {
                    let Self { shard, realm, num, checksum } = *self;
                    EntityId { shard, realm, num, checksum }
                }
            }
        )*
    };
}

impl_ffi_convert_traits_for!(FileId, TokenId, ScheduleId, TopicId);

/// # Safety
/// - `id_shard`, `id_realm`, and `id_num` must all be valid for writes.
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
unsafe fn id_from_bytes<I: FromToBytes + FromToEntityId>(
    bytes: *const u8,
    bytes_size: size_t,
    id_shard: *mut u64,
    id_realm: *mut u64,
    id_num: *mut u64,
) -> Error {
    assert!(!bytes.is_null());
    assert!(!id_shard.is_null());
    assert!(!id_realm.is_null());
    assert!(!id_num.is_null());

    // safety: caller promises that `bytes` is valid for r/w of up to `bytes_size`, which is exactly what `slice::from_raw_parts` wants.
    let bytes = unsafe { slice::from_raw_parts(bytes, bytes_size) };

    let id = ffi_try!(I::ffi_from_bytes(bytes));
    let id = id.to_entity_id();

    // safety: function contract states that all of these must be valid for writes.
    unsafe {
        ptr::write(id_shard, id.shard);
        ptr::write(id_realm, id.realm);
        ptr::write(id_num, id.num);
    }

    Error::Ok
}

/// # Safety
/// - `buf` must be valid for writes.
unsafe fn id_to_bytes<I: FromToBytes + FromToEntityId>(
    id_shard: u64,
    id_realm: u64,
    id_num: u64,
    buf: *mut *mut u8,
) -> size_t {
    // todo: use `as_maybe_uninit_ref` once that's stable.
    assert!(!buf.is_null());

    let id = EntityId { shard: id_shard, realm: id_realm, num: id_num, checksum: None };
    let id = I::from_entity_id(id);
    let bytes = id.ffi_to_bytes();

    let bytes = Box::leak(bytes);
    let len = bytes.len();
    let bytes = bytes.as_mut_ptr();

    // safety: invariants promise that `buf` must be valid for writes.
    unsafe {
        ptr::write(buf, bytes);
    }

    len
}

/// Parse a Hedera `FileId` from the passed bytes.
///
/// # Safety
/// - `file_id_shard`, `file_id_realm`, and `file_id_num` must all be valid for writes.
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
#[no_mangle]
pub unsafe extern "C" fn hedera_file_id_from_bytes(
    bytes: *const u8,
    bytes_size: size_t,
    file_id_shard: *mut u64,
    file_id_realm: *mut u64,
    file_id_num: *mut u64,
) -> Error {
    // safety: invariants pushed up to the caller.
    unsafe { id_from_bytes::<FileId>(bytes, bytes_size, file_id_shard, file_id_realm, file_id_num) }
}

/// Serialize the passed `FileId` as bytes
///
/// # Safety
/// - `buf` must be valid for writes.
#[no_mangle]
pub unsafe extern "C" fn hedera_file_id_to_bytes(
    file_id_shard: u64,
    file_id_realm: u64,
    file_id_num: u64,
    buf: *mut *mut u8,
) -> size_t {
    // safety: invariants pushed up to the caller.
    unsafe { id_to_bytes::<FileId>(file_id_shard, file_id_realm, file_id_num, buf) }
}

/// Parse a Hedera `TopicId` from the passed bytes.
///
/// # Safety
/// - `topic_id_shard`, `topic_id_realm`, and `topic_id_num` must all be valid for writes.
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
#[no_mangle]
pub unsafe extern "C" fn hedera_topic_id_from_bytes(
    bytes: *const u8,
    bytes_size: size_t,
    topic_id_shard: *mut u64,
    topic_id_realm: *mut u64,
    topic_id_num: *mut u64,
) -> Error {
    // safety: invariants pushed up to the caller.
    unsafe {
        id_from_bytes::<TopicId>(bytes, bytes_size, topic_id_shard, topic_id_realm, topic_id_num)
    }
}

/// Serialize the passed `TopicId` as bytes
///
/// # Safety
/// - `buf` must be valid for writes.
#[no_mangle]
pub unsafe extern "C" fn hedera_topic_id_to_bytes(
    topic_id_shard: u64,
    topic_id_realm: u64,
    topic_id_num: u64,
    buf: *mut *mut u8,
) -> size_t {
    // safety: invariants pushed up to the caller.
    unsafe { id_to_bytes::<TopicId>(topic_id_shard, topic_id_realm, topic_id_num, buf) }
}

/// Parse a Hedera `TokenId` from the passed bytes.
///
/// # Safety
/// - `token_id_shard`, `token_id_realm`, and `token_id_num` must all be valid for writes.
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
#[no_mangle]
pub unsafe extern "C" fn hedera_token_id_from_bytes(
    bytes: *const u8,
    bytes_size: size_t,
    token_id_shard: *mut u64,
    token_id_realm: *mut u64,
    token_id_num: *mut u64,
) -> Error {
    // safety: invariants pushed up to the caller.
    unsafe {
        id_from_bytes::<TokenId>(bytes, bytes_size, token_id_shard, token_id_realm, token_id_num)
    }
}

/// Serialize the passed `TokenId` as bytes
///
/// # Safety
/// - `buf` must be valid for writes.
#[no_mangle]
pub unsafe extern "C" fn hedera_token_id_to_bytes(
    token_id_shard: u64,
    token_id_realm: u64,
    token_id_num: u64,
    buf: *mut *mut u8,
) -> size_t {
    // safety: invariants pushed up to the caller.
    unsafe { id_to_bytes::<TokenId>(token_id_shard, token_id_realm, token_id_num, buf) }
}

/// Parse a Hedera `ScheduleId` from the passed bytes.
///
/// # Safety
/// - `schedule_id_shard`, `schedule_id_realm`, and `schedule_id_num` must all be valid for writes.
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
#[no_mangle]
pub unsafe extern "C" fn hedera_schedule_id_from_bytes(
    bytes: *const u8,
    bytes_size: size_t,
    schedule_id_shard: *mut u64,
    schedule_id_realm: *mut u64,
    schedule_id_num: *mut u64,
) -> Error {
    // safety: invariants pushed up to the caller.
    unsafe {
        id_from_bytes::<ScheduleId>(
            bytes,
            bytes_size,
            schedule_id_shard,
            schedule_id_realm,
            schedule_id_num,
        )
    }
}

/// Serialize the passed `ScheduleId` as bytes
///
/// # Safety
/// - `buf` must be valid for writes.
#[no_mangle]
pub unsafe extern "C" fn hedera_schedule_id_to_bytes(
    schedule_id_shard: u64,
    schedule_id_realm: u64,
    schedule_id_num: u64,
    buf: *mut *mut u8,
) -> size_t {
    // safety: invariants pushed up to the caller.
    unsafe { id_to_bytes::<TokenId>(schedule_id_shard, schedule_id_realm, schedule_id_num, buf) }
}
