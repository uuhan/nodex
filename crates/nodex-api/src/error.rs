use crate::api;

/// napi_status
#[repr(u32)]
#[derive(thiserror::Error, Debug)]
pub enum NapiStatus {
    #[error("napi ok")]
    Ok = api::napi_status_napi_ok,
    #[error("napi error: invalid arg")]
    InvalidArg = api::napi_status_napi_invalid_arg,
    #[error("napi error: object expected")]
    ObjectExpected = api::napi_status_napi_object_expected,
    #[error("napi error: string expected")]
    StringExpected = api::napi_status_napi_string_expected,
    #[error("napi error: name expected")]
    NameExpected = api::napi_status_napi_name_expected,
    #[error("napi error: function expected")]
    FunctionExpected = api::napi_status_napi_function_expected,
    #[error("napi error: number expected")]
    NumberExpected = api::napi_status_napi_number_expected,
    #[error("napi error: boolean expected")]
    BooleanExpected = api::napi_status_napi_boolean_expected,
    #[error("napi error: array expected")]
    ArrayExpected = api::napi_status_napi_array_expected,
    #[error("napi error: generic failure")]
    GenericFailure = api::napi_status_napi_generic_failure,
    #[error("napi error: pending exception")]
    PendingException = api::napi_status_napi_pending_exception,
    #[error("napi error: cancelled")]
    Cancelled = api::napi_status_napi_cancelled,
    #[error("napi error: escape called twice")]
    EscapeCalledTwice = api::napi_status_napi_escape_called_twice,
    #[error("napi error: handle scope mismatch")]
    HandleScopeMismatch = api::napi_status_napi_handle_scope_mismatch,
    #[error("napi error: callback scope mismatch")]
    CallbackScopeMismatch = api::napi_status_napi_callback_scope_mismatch,
    #[error("napi error: queue full")]
    QueueFull = api::napi_status_napi_queue_full,
    #[error("napi error: closing")]
    Closing = api::napi_status_napi_closing,
    #[error("napi error: bigint excepted")]
    BigintExpected = api::napi_status_napi_bigint_expected,
    #[error("napi error: date expected")]
    DateExpected = api::napi_status_napi_date_expected,
    #[error("napi error: arraybuffer expected")]
    ArraybufferExpected = api::napi_status_napi_arraybuffer_expected,
    #[error("napi error: detachable arraybuffer expected")]
    DetachableArraybufferExpected = api::napi_status_napi_detachable_arraybuffer_expected,
    #[error("napi error: would deadlock")]
    WouldDeadlock = api::napi_status_napi_would_deadlock,
}

pub type NapiResult<T> = Result<T, NapiStatus>;
