use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsNumber(pub(crate) JsValue);

impl JsNumber {
    pub(crate) fn from_value(value: JsValue) -> JsNumber {
        JsNumber(value)
    }

    /// This API is used to convert from the C int32_t type to the JavaScript number type.
    /// The JavaScript number type is described in Section 6.1.6 of the ECMAScript Language Specification.
    pub fn int32(env: NapiEnv, value: i32) -> NapiResult<JsNumber> {
        let value = napi_call!(
            =napi_create_int32,
            env.raw(),
            value,
        );

        Ok(JsNumber(JsValue::from_raw(env, value)))
    }

    /// This API is used to convert from the C uint32_t type to the JavaScript number type.
    /// The JavaScript number type is described in Section 6.1.6 of the ECMAScript Language Specification.
    pub fn uint32(env: NapiEnv, value: u32) -> NapiResult<JsNumber> {
        let value = napi_call!(
            =napi_create_uint32,
            env.raw(),
            value,
        );

        Ok(JsNumber(JsValue::from_raw(env, value)))
    }

    /// This API is used to convert from the C int64_t type to the JavaScript number type.
    /// The JavaScript number type is described in Section 6.1.6 of the ECMAScript Language Specification. Note the complete range of int64_t cannot be represented with full precision in JavaScript. Integer values outside the range of Number.MIN_SAFE_INTEGER -(2**53 - 1) - Number.MAX_SAFE_INTEGER (2**53 - 1) will lose precision.
    pub fn int64(env: NapiEnv, value: i64) -> NapiResult<JsNumber> {
        let value = napi_call!(
            =napi_create_int64,
            env.raw(),
            value,
        );

        Ok(JsNumber(JsValue::from_raw(env, value)))
    }

    /// This API is used to convert from the C double type to the JavaScript number type.
    /// The JavaScript number type is described in Section 6.1.6 of the ECMAScript Language Specification.
    pub fn double(env: NapiEnv, value: f64) -> NapiResult<JsNumber> {
        let value = napi_call!(
            =napi_create_double,
            env.raw(),
            value,
        );

        Ok(JsNumber(JsValue::from_raw(env, value)))
    }

    /// If a non-number napi_value is passed in napi_number_expected.
    /// This API returns the C int32 primitive equivalent of the given JavaScript number.
    /// If the number exceeds the range of the 32 bit integer, then the result is truncated to the equivalent of the bottom 32 bits. This can result in a large positive number becoming a negative number if the value is > 231 - 1.
    /// Non-finite number values (NaN, +Infinity, or -Infinity) set the result to zero.
    pub fn get_value_int32(&self) -> NapiResult<i32> {
        Ok(napi_call!(=napi_get_value_int32, self.env().raw(), self.raw()))
    }

    /// If a non-number napi_value is passed in it returns napi_number_expected.
    /// This API returns the C primitive equivalent of the given napi_value as a uint32_t.
    pub fn get_value_uint32(&self) -> NapiResult<u32> {
        Ok(napi_call!(=napi_get_value_uint32, self.env().raw(), self.raw()))
    }

    /// If a non-number napi_value is passed in it returns napi_number_expected.
    /// This API returns the C int64 primitive equivalent of the given JavaScript number.
    /// number values outside the range of Number.MIN_SAFE_INTEGER -(2**53 - 1) - Number.MAX_SAFE_INTEGER (2**53 - 1) will lose precision.
    /// Non-finite number values (NaN, +Infinity, or -Infinity) set the result to zero.
    pub fn get_value_int64(&self) -> NapiResult<i64> {
        Ok(napi_call!(=napi_get_value_int64, self.env().raw(), self.raw()))
    }

    /// If a non-number napi_value is passed in it returns napi_number_expected.
    /// This API returns the C double primitive equivalent of the given JavaScript number.
    pub fn get_value_double(&self) -> NapiResult<f64> {
        Ok(napi_call!(=napi_get_value_double, self.env().raw(), self.raw()))
    }
}

napi_value_t!(JsNumber);
