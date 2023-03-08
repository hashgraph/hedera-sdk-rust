use std::{
    ptr,
    slice,
};

use libc::{
    c_void,
    size_t,
};

use crate::PublicKey;

#[repr(C)]
pub struct Signers {
    /// may only be null if signers_size is 0.
    signers: *const Signer,
    signers_size: size_t,
    /// Free this array of signers (must *not* free the contexts for the original signers)
    free: Option<unsafe extern "C" fn(signers: *const Signer, signers_size: size_t)>,
}

impl Signers {
    pub(super) fn as_slice(&self) -> &[Signer] {
        if self.signers.is_null() {
            &[]
        } else {
            unsafe { slice::from_raw_parts(self.signers, self.signers_size) }
        }
    }
}

impl Drop for Signers {
    fn drop(&mut self) {
        if let Some(free) = self.free {
            // we don't touch self after this.
            unsafe { free(self.signers, self.signers_size) }
        }
    }
}

#[repr(C)]
pub struct Signer {
    /// Safety:
    /// - Must not be null
    /// - must be properly aligned
    /// - must be dereferencable in the rust sense.
    public_key: *const PublicKey,
    /// Safety: It must be safe to send `context` to other threads.
    /// Safety: It must be safe to share `context` between threads.
    context: *mut c_void,
    /// Safety:
    /// Must not be null
    /// must be callable with the appropriate arguments
    sign_func: Option<
        unsafe extern "C" fn(
            context: *mut c_void,
            message: *const u8,
            message_size: size_t,
            signature: *mut *const u8,
        ) -> size_t,
    >,
    // note: can't use typedefs in ffi apparently, cbindgen doesn't know what it
    /// Safety:
    /// Must not be null
    /// must be callable with the appropriate arguments
    free_signature_func: Option<
        unsafe extern "C" fn(context: *mut c_void, signature: *mut u8, signature_size: size_t),
    >,
    /// Safety:
    /// May be null
    /// must be callable with the appropriate arguments
    free_context_func: Option<unsafe extern "C" fn(context: *mut c_void)>,
}

impl Signer {
    pub(super) fn to_csigner(&self) -> CSigner {
        // the dance here is because we actually want to own the public key (it's currently in a `Box`, we probably should replace it with a `)
        let public_key = *unsafe { self.public_key.as_ref() }.unwrap();
        let public_key = Box::new(public_key);
        let sign_func = self.sign_func.unwrap();
        let free_signature_func = self.free_signature_func.unwrap();
        CSigner {
            public_key,
            context: self.context,
            sign_func,
            free_signature_func,
            free_context_func: self.free_context_func,
        }
    }
}

/// # Safety
/// - `message` must not be freed, at all.
/// - `signature` must be non-null after returning from this function.
/// - `signature` must be valid for reads of up to the returned length bytes.
pub(super) type SignFn = unsafe extern "C" fn(
    context: *mut c_void,
    message: *const u8,
    message_size: size_t,
    signature: *mut *const u8,
) -> size_t;

pub(super) type FreeSignatureFn =
    unsafe extern "C" fn(context: *mut c_void, signature: *mut u8, signature_size: size_t);

pub(crate) struct CSigner {
    public_key: Box<PublicKey>,
    /// Safety: It must be safe to send `context` to other threads.
    /// Safety: It must be safe to share `context` between threads.
    context: *mut c_void,
    sign_func: SignFn,
    free_signature_func: FreeSignatureFn,
    free_context_func: Option<unsafe extern "C" fn(context: *mut c_void)>,
}

impl CSigner {
    pub(crate) fn public_key(&self) -> PublicKey {
        *self.public_key
    }

    pub(crate) fn sign(&self, message: &[u8]) -> (PublicKey, Vec<u8>) {
        let message_size = message.len();
        let message = message.as_ptr();

        let mut signature = ptr::null();

        let signature_size = unsafe {
            (self.sign_func)(self.context, message, message_size, ptr::addr_of_mut!(signature))
        };

        assert!(!signature.is_null());
        let signature_out = unsafe { slice::from_raw_parts(signature, signature_size) }.to_vec();

        unsafe {
            (self.free_signature_func)(self.context, signature.cast_mut(), signature_size);
        }

        (*self.public_key, signature_out)
    }
}

impl Drop for CSigner {
    fn drop(&mut self) {
        if let Some(free_context) = self.free_context_func {
            unsafe { free_context(self.context) }
        }
    }
}

// Safety: It must be safe to send `context` (the `*mut c_void`) to other threads.
unsafe impl Send for CSigner {}
// Safety: it must be safe to share `context` (the `*mut c_void`) between threads.
unsafe impl Sync for CSigner {}
