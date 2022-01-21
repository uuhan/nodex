#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/// this mod is generated by cargo build -p nodex-api --features gen-api
pub mod api;
pub mod api_impl;
#[macro_use]
mod mac;

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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

/// napi_key_collection_mode
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NapiKeyCollectionMode {
    KeyIncludePrototypes = api::napi_key_collection_mode_napi_key_include_prototypes,
    KeyOwnOnly = api::napi_key_collection_mode_napi_key_own_only,
}

/// napi_key_filter
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

#[cfg(feature = "v4")]
#[doc = "napi_threadsafe_function_release_mode"]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NapiThreadsafeFunctionReleaseMode {
    Release = api::napi_threadsafe_function_release_mode_napi_tsfn_release,
    Abort = api::napi_threadsafe_function_release_mode_napi_tsfn_abort,
}

#[cfg(feature = "v4")]
#[doc = "napi_threadsafe_function_call_mode"]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NapiThreadsafeFunctionCallMode {
    Nonblocking = api::napi_threadsafe_function_call_mode_napi_tsfn_nonblocking,
    Blocking = api::napi_threadsafe_function_call_mode_napi_tsfn_blocking,
}

pub mod args;
pub mod callback;
pub mod context;
pub mod descriptor;
pub mod env;
mod error;
pub mod reference;
pub mod rt;
pub mod scope;
pub mod value;
pub mod work;

// napi status and result type
pub use error::{NapiResult, NapiStatus};

pub mod prelude {
    use super::*;

    pub use super::NapiKeyCollectionMode;
    pub use super::NapiKeyConversion;
    pub use super::NapiKeyFilter;
    pub use super::NapiPropertyAttributes;
    #[cfg(feature = "v4")]
    pub use super::NapiThreadsafeFunctionCallMode;
    #[cfg(feature = "v4")]
    pub use super::NapiThreadsafeFunctionReleaseMode;
    pub use super::NapiTypedarrayType;
    pub use super::NapiValuetype;

    pub use env::NapiEnv;
    pub use error::{NapiResult, NapiStatus};

    pub use args::*;
    pub use callback::CallbackInfo;
    pub use context::NapiAsyncContext;
    pub use descriptor::{DescriptorBuilder, NapiPropertyDescriptor};
    pub use reference::NapiRef;
    pub use scope::*;
    pub use value::*;
    pub use work::NapiAsyncWork;

    #[cfg(feature = "v8")]
    pub use api::napi_async_cleanup_hook_handle;
    pub use api::{
        napi_async_context, napi_async_work, napi_callback, napi_callback_info,
        napi_callback_scope, napi_deferred, napi_env, napi_escapable_handle_scope,
        napi_handle_scope, napi_property_descriptor, napi_ref, napi_value, NapiExtendedErrorInfo,
    };

    pub type DataPointer = *mut std::ffi::c_void;
    pub type CharPointer = *mut std::os::raw::c_char;
}

pub const fn napi_version_guard() -> u32 {
    #[cfg(feature = "v8")]
    return 8;
    #[cfg(feature = "v7")]
    return 7;
    #[cfg(feature = "v6")]
    return 6;
    #[cfg(feature = "v5")]
    return 5;
    #[cfg(feature = "v4")]
    return 4;
    #[cfg(feature = "v3")]
    return 3;
    #[cfg(feature = "v2")]
    return 2;
    #[cfg(feature = "v1")]
    return 1;
    panic!("please select a napi version to use.")
}

/// The function call does not return, the process will be terminated.
/// This API can be called even if there is a pending JavaScript exception.
#[inline]
pub fn fatal_error(msg: impl AsRef<str>, loc: Option<impl AsRef<str>>) {
    let (loc, loc_len) = if let Some(loc) = loc {
        (loc.as_ref().as_ptr() as *const _, loc.as_ref().len())
    } else {
        (std::ptr::null(), 0)
    };
    unsafe {
        api::napi_fatal_error(
            loc,
            loc_len,
            msg.as_ref().as_ptr() as *const _,
            msg.as_ref().len(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
