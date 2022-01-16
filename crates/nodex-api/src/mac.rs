// common macros

#[macro_export]
macro_rules! napi_call {
    // [out] result: napi function which has output
    (=$napi:ident, $($args:expr),+ $(,)?) => {
        unsafe {
            let mut result = std::mem::MaybeUninit::uninit();
            let status = crate::api::$napi($($args),+, result.as_mut_ptr());
            if status.err() {
                return Err(status)
            }
            result.assume_init()
        }
    };

    ($napi:ident, $($args:expr),+ $(,)?) => {
        unsafe {
            let status = crate::api::$napi($($args),+);
            if status.err() {
                return Err(status)
            }
        }
    }
}
