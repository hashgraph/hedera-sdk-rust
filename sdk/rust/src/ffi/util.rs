use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;

pub(crate) unsafe fn cstr_from_ptr<'a>(ptr: *const c_char) -> Cow<'a, str> {
    assert!(!ptr.is_null());

    CStr::from_ptr(ptr).to_string_lossy()
}
