#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod api {
    include!(concat!(env!("OUT_DIR"), "/api.rs"));

    #[repr(u32)]
    pub enum NapiStatus {
        Ok = napi_status_napi_ok,
        InvalidArg = napi_status_napi_invalid_arg,
        ObjectExpected = napi_status_napi_object_expected,
        StringExpected = napi_status_napi_string_expected,
        NameExpected = napi_status_napi_name_expected,
        FunctionExpected = napi_status_napi_function_expected,
        NumberExpected = napi_status_napi_number_expected,
        BooleanExpected = napi_status_napi_boolean_expected,
        ArrayExpected = napi_status_napi_array_expected,
        GenericFailure = napi_status_napi_generic_failure,
        PendingException = napi_status_napi_pending_exception,
        Cancelled = napi_status_napi_cancelled,
        EscapeCalledTwice = napi_status_napi_escape_called_twice,
        HandleScopeMismatch = napi_status_napi_handle_scope_mismatch,
        CallbackScopeMismatch = napi_status_napi_callback_scope_mismatch,
        QueueFull = napi_status_napi_queue_full,
        Closing = napi_status_napi_closing,
        BigintExpected = napi_status_napi_bigint_expected,
        DateExpected = napi_status_napi_date_expected,
        ArraybufferExpected = napi_status_napi_arraybuffer_expected,
        DetachableArraybufferExpected = napi_status_napi_detachable_arraybuffer_expected,
        WouldDeadlock = napi_status_napi_would_deadlock,
    }
}

pub mod env;
pub mod value;

// useful macros
pub use node_api_macros::init;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_napi_true_() {
        assert_eq!(api::true_, 1);
    }
}
