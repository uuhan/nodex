use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsBoolean(pub(crate) JsValue);

impl JsBoolean {
    pub(crate) fn from_value(value: JsValue) -> JsBoolean {
        JsBoolean(value)
    }

    /// This API is used to return the JavaScript singleton object that is used to represent the
    /// given boolean value.
    pub fn new(env: NapiEnv, value: bool) -> NapiResult<JsBoolean> {
        let value = napi_call!(=napi_get_boolean, env.raw(), value);
        Ok(JsBoolean(JsValue::from_raw(env, value)))
    }

    /// If a non-boolean napi_value is passed in it returns napi_boolean_expected.
    /// This API returns the C boolean primitive equivalent of the given JavaScript Boolean.
    pub fn get(&self) -> NapiResult<bool> {
        let value = napi_call!(=napi_get_value_bool, self.env().raw(), self.raw());
        Ok(value)
    }
}

napi_value_t!(JsBoolean);
