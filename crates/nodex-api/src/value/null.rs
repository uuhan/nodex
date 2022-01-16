use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsNull(pub(crate) JsValue);

impl JsNull {
    pub(crate) fn from_value(value: JsValue) -> JsNull {
        JsNull(value)
    }

    /// This API returns the null object.
    pub fn new(env: NapiEnv) -> NapiResult<JsNull> {
        let value = napi_call!(=napi_get_null, env.raw());
        Ok(JsNull(JsValue::from_raw(env, value)))
    }
}

impl NapiValueT for JsNull {
    fn value(&self) -> JsValue {
        self.0
    }
}
