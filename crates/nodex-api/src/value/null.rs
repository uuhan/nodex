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
        let value = napi_call!(=napi_get_null, env);
        Ok(JsNull(JsValue::from_raw(env, value)))
    }
}

napi_value_t!(JsNull);

impl NapiValueCheck for JsNull {
    fn check(&self) -> NapiResult<bool> {
        Ok(self.kind()? == NapiValuetype::Null)
    }
}
