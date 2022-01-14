#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod api {
    include!(concat!(env!("OUT_DIR"), "/api.rs"));
}

/// napi_property_attributes
#[repr(u32)]
pub enum NapiPropertyAttributes {
    Default = api::napi_property_attributes_napi_default,
    Writable = api::napi_property_attributes_napi_writable,
    Enumerable = api::napi_property_attributes_napi_enumerable,
    Configurable = api::napi_property_attributes_napi_configurable,
    Static = api::napi_property_attributes_napi_static,
    DefaultMethod = api::napi_property_attributes_napi_default_method,
    DefaultJsproperty = api::napi_property_attributes_napi_default_jsproperty,
}

/// napi_valuetype
#[repr(u32)]
pub enum NapiValuetype {
    Undefined = api::napi_valuetype_napi_undefined,
    Null = api::napi_valuetype_napi_null,
    Boolean = api::napi_valuetype_napi_boolean,
    Number = api::napi_valuetype_napi_number,
    String = api::napi_valuetype_napi_string,
    Symbol = api::napi_valuetype_napi_symbol,
    Object = api::napi_valuetype_napi_object,
    Function = api::napi_valuetype_napi_function,
    External = api::napi_valuetype_napi_external,
    Bigint = api::napi_valuetype_napi_bigint,
}

/// napi_typedarray_type
#[repr(u32)]
pub enum NapiTypedarrayType {
    Int8Array = api::napi_typedarray_type_napi_int8_array,
    Uint8Array = api::napi_typedarray_type_napi_uint8_array,
    Uint8ClampedArray = api::napi_typedarray_type_napi_uint8_clamped_array,
    Int16Array = api::napi_typedarray_type_napi_int16_array,
    Uint16Array = api::napi_typedarray_type_napi_uint16_array,
    Int32Array = api::napi_typedarray_type_napi_int32_array,
    Uint32Array = api::napi_typedarray_type_napi_uint32_array,
    Float32Array = api::napi_typedarray_type_napi_float32_array,
    Float64Array = api::napi_typedarray_type_napi_float64_array,
    Bigint64Array = api::napi_typedarray_type_napi_bigint64_array,
    Biguint64Array = api::napi_typedarray_type_napi_biguint64_array,
}

/// napi_status
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
