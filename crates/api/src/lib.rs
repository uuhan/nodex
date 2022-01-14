#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod api {
    include!(concat!(env!("OUT_DIR"), "/api.rs"));
}

#[repr(u32)]
pub enum NapiStatus {
    Ok = api::napi_status_napi_ok,
    InvalidArg = api::napi_status_napi_invalid_arg,
    ObjectExpected = api::napi_status_napi_object_expected,
    StringExpected = api::napi_status_napi_string_expected,
    NameExpected = api::napi_status_napi_name_expected,
    FunctionExpected = api::napi_status_napi_function_expected,
    NumberExpected = api::napi_status_napi_number_expected,
    BooleanExpected = api::napi_status_napi_boolean_expected,
    ArrayExpected = api::napi_status_napi_array_expected,
    GenericFailure = api::napi_status_napi_generic_failure,
    PendingException = api::napi_status_napi_pending_exception,
    Cancelled = api::napi_status_napi_cancelled,
    EscapeCalledTwice = api::napi_status_napi_escape_called_twice,
    HandleScopeMismatch = api::napi_status_napi_handle_scope_mismatch,
    CallbackScopeMismatch = api::napi_status_napi_callback_scope_mismatch,
    QueueFull = api::napi_status_napi_queue_full,
    Closing = api::napi_status_napi_closing,
    BigintExpected = api::napi_status_napi_bigint_expected,
    DateExpected = api::napi_status_napi_date_expected,
    ArraybufferExpected = api::napi_status_napi_arraybuffer_expected,
    DetachableArraybufferExpected = api::napi_status_napi_detachable_arraybuffer_expected,
    WouldDeadlock = api::napi_status_napi_would_deadlock,
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
