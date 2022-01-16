use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsNull<'a>(pub(crate) JsValue<'a>);

impl<'a> JsNull<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsNull {
        JsNull(value)
    }

    /// This API returns the null object.
    pub fn new(env: NapiEnv<'a>) -> NapiResult<JsNull<'a>> {
        let value = napi_call!(=napi_get_null, env.raw());
        Ok(JsNull(JsValue::from_raw(env, value)))
    }
}

impl<'a> NapiValueT for JsNull<'a> {
    fn inner(&self) -> JsValue {
        self.0
    }
}
