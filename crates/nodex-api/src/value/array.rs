use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsArray(pub(crate) JsValue);

impl JsArray {
    pub(crate) fn from_value(value: JsValue) -> JsArray {
        JsArray(value)
    }

    /// This API returns a Node-API value corresponding to a JavaScript Array type. The Array's
    /// length property is set to the passed-in length parameter. However, the underlying buffer
    /// is not guaranteed to be pre-allocated by the VM when the array is created. That behavior
    /// is left to the underlying VM implementation. If the buffer must be a contiguous block of
    /// memory that can be directly read and/or written via C, consider using napi_create_external_arraybuffer.
    ///
    /// JavaScript arrays are described in Section 22.1 of the ECMAScript Language Specification.
    pub fn new(env: NapiEnv, length: usize) -> NapiResult<JsArray> {
        let value = napi_call!(=napi_create_array_with_length, env, length);
        Ok(JsArray(JsValue::from_raw(env, value)))
    }

    /// This API returns a Node-API value corresponding to a JavaScript Array type. JavaScript
    /// arrays are described in Section 22.1 of the ECMAScript Language Specification.
    pub fn empty(env: NapiEnv) -> NapiResult<JsArray> {
        let value = napi_call!(=napi_create_array, env);
        Ok(JsArray(JsValue::from_raw(env, value)))
    }

    /// This API returns the length of an array.
    /// Array length is described in Section 22.1.4.1 of the ECMAScript Language Specification.
    pub fn len(&self) -> NapiResult<u32> {
        let len = napi_call!(=napi_get_array_length, self.env(), self.raw());
        Ok(len)
    }

    /// This array is empty.
    pub fn is_empty(&self) -> NapiResult<bool> {
        Ok(self.len()? == 0)
    }

    /// Get element at `index`
    #[inline]
    pub fn get(&self, index: u32) -> NapiResult<JsValue> {
        Ok(JsValue::from_raw(
            self.env(),
            napi_call!(=napi_get_element, self.env(), self.raw(), index),
        ))
    }
}

napi_value_t!(JsArray);
