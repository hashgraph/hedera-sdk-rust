use std::os::raw::c_void;

/// Alias to a callback that refers to a function with the C calling convention.
pub(super) type CallbackHandle<T> = extern "C" fn(context: *const c_void, value: T);

/// Wrapper around a C callback handle and an associated opaque context pointer.
pub(super) struct Callback<T> {
    handle: CallbackHandle<T>,
    context: *const c_void,
}

impl<T> Callback<T> {
    pub(super) fn new(context: *const c_void, handle: CallbackHandle<T>) -> Self {
        Self { handle, context }
    }

    pub(super) fn call(self, value: T) {
        (self.handle)(self.context, value)
    }
}

// NOTE: The context pointer is referring to state that is never inspected, only passed
unsafe impl<T> Send for Callback<T> {}
