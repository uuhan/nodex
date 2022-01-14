#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod api {
    include!(concat!(env!("OUT_DIR"), "/api.rs"));
}

/// napi_boolean
#[repr(u32)]
pub enum NapiBoolean {
    True = api::true_,
    False = api::false_,
}

bitflags::bitflags! {
    /// napi_property_attributes
    #[repr(C)]
    pub struct NapiPropertyAttributes: u32 {
        const Default = api::napi_property_attributes_napi_default;
        const Writable = api::napi_property_attributes_napi_writable;
        const Enumerable = api::napi_property_attributes_napi_enumerable;
        const Configurable = api::napi_property_attributes_napi_configurable;
        const Static = api::napi_property_attributes_napi_static;
        const DefaultMethod = api::napi_property_attributes_napi_default_method;
        const DefaultJsproperty = api::napi_property_attributes_napi_default_jsproperty;
    }
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

/// napi_key_collection_mode
#[repr(u32)]
pub enum NapiKeyCollectionMode {
    KeyIncludePrototypes = api::napi_key_collection_mode_napi_key_include_prototypes,
    KeyOwnOnly = api::napi_key_collection_mode_napi_key_own_only,
}

/// napi_key_filter
#[repr(u32)]
pub enum NapiKeyFilter {
    KeyAllProperties = api::napi_key_filter_napi_key_all_properties,
    Writable = api::napi_key_filter_napi_key_writable,
    Enumerable = api::napi_key_filter_napi_key_enumerable,
    Configurable = api::napi_key_filter_napi_key_configurable,
    SkipStrings = api::napi_key_filter_napi_key_skip_strings,
    SkipSymbols = api::napi_key_filter_napi_key_skip_symbols,
}

/// napi_key_conversion
#[repr(u32)]
pub enum NapiKeyConversion {
    KeepNumbers = api::napi_key_conversion_napi_key_keep_numbers,
    NumbersToStrings = api::napi_key_conversion_napi_key_numbers_to_strings,
}

/// napi_threadsafe_function_release_mode
#[repr(u32)]
pub enum NapiThreadsafeFunctionReleaseMode {
    Release = api::napi_threadsafe_function_release_mode_napi_tsfn_release,
    Abort = api::napi_threadsafe_function_release_mode_napi_tsfn_abort,
}

/// napi_threadsafe_function_call_mode
#[repr(u32)]
pub enum NapiThreadsafeFunctionCallMode {
    Nonblocking = api::napi_threadsafe_function_call_mode_napi_tsfn_nonblocking,
    Blocking = api::napi_threadsafe_function_call_mode_napi_tsfn_blocking,
}

pub mod env;
pub mod value;

// useful macros
pub use nodex_macros::init;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_napi_boolean() {
        assert_eq!(NapiBoolean::True as u32, 1);
        assert_eq!(NapiBoolean::False as u32, 0);
    }
}
