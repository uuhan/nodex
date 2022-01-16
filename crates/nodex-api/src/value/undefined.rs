use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsUndefined<'a>(pub(crate) JsValue<'a>);

impl<'a> JsUndefined<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsUndefined {
        JsUndefined(value)
    }

    /// This API returns the Undefined object.
    pub fn new(env: NapiEnv<'a>) -> NapiResult<JsUndefined<'a>> {
        let value = napi_call!(=napi_get_undefined, env.raw());
        Ok(JsUndefined(JsValue::from_raw(env, value)))
    }
}

impl<'a> NapiValueT for JsUndefined<'a> {
    fn inner(&self) -> JsValue {
        self.0
    }
}
