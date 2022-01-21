use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsArray(pub(crate) JsValue);

impl JsArray {
    pub(crate) fn from_value(value: JsValue) -> JsArray {
        JsArray(value)
    }

    /// This API returns a Node-API value corresponding to a JavaScript Array type. JavaScript
    /// arrays are described in Section 22.1 of the ECMAScript Language Specification.
    pub fn new(env: NapiEnv, value: impl AsRef<str>) -> NapiResult<JsArray> {
        let value = napi_call!(=napi_create_array, env);
        Ok(JsArray(JsValue::from_raw(env, value)))
    }

    /// This API returns the length of an array.
    /// Array length is described in Section 22.1.4.1 of the ECMAScript Language Specification.
    pub fn len(&self) -> NapiResult<u32> {
        let len = napi_call!(=napi_get_array_length, self.env(), self.raw());
        Ok(len)
    }
}

napi_value_t!(JsArray);
