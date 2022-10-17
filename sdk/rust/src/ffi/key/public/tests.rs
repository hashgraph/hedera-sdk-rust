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
    CStr,
    CString,
};
use std::ptr::{
    self,
    addr_of_mut,
};
use std::str::FromStr;

use assert_matches::assert_matches;
use expect_test::expect;

use crate::ffi::c_util::hedera_string_free;
use crate::ffi::error::Error;
use crate::ffi::key::public::{
    hedera_public_key_free,
    hedera_public_key_from_string,
    hedera_public_key_to_string,
};
use crate::{
    PublicKey,
    Signature,
};

#[test]
fn ed25519_from_str() {
    const PK: &str =
        "302a300506032b6570032100e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7";

    let pk_str = CString::new(PK).unwrap();

    let s = {
        let pk = {
            let mut pk = ptr::null_mut();
            // safety: the passed string is valid for reads during this function,
            // safety: the passed key is valid for writes (not reads)
            let error = unsafe { hedera_public_key_from_string(pk_str.as_ptr(), addr_of_mut!(pk)) };
            assert_eq!(error, Error::Ok);
            // safety: pk is now initialized.
            pk
        };

        // note (not a safety concern): This block *can* leak if it panics. it *won't*, because we produce utf8, but if we ever stop using utf8 in 5 decades, and we run out of memory on failing tests, this is where to look.
        let s = {
            // safety: PK is valid for reads (proven via)
            let ptr = unsafe { hedera_public_key_to_string(pk) };
            // safety: this CStr is lifetime bounded to `ptr`, so we ensure it lives shorter by putting it in a block
            let s = {
                // safety:
                // `hedera_public_key_to_string` returns a C String (notice the space), and promises it has a null terminator (C Strings end with one)
                // `hedera_public_key_to_string` promises that its ptr is valid for reads through the null terminator until `hedera_string_free` is called, and we won't call it until after this block.
                let s = unsafe { CStr::from_ptr(ptr) };
                s.to_str().unwrap().to_owned()
            };

            // safety: we haven't called this method on `ptr` yet, `ptr` has been initialized by `hedera_public_key_to_string` which is a method that allows this to be called on the resultb.
            unsafe { hedera_string_free(ptr) };
            s
        };

        // safety: we haven't called this method on `pk` yet, `pk` has been initialized `hedera_public_key_from_string`, which is a method that allows this to be called on the result.
        unsafe { hedera_public_key_free(pk) };
        s
    };

    assert_eq!(PK, s)
}

// todo: The rest of these tests (it's a lot of work )
