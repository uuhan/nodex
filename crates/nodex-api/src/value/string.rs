use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsString<'a>(pub(crate) JsValue<'a>);

impl<'a> JsString<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsString {
        JsString(value)
    }

    /// This API creates a JavaScript string value from a UTF8-encoded C string. The native string is copied.
    /// The JavaScript string type is described in Section 6.1.4 of the ECMAScript Language Specification.
    pub fn new(env: NapiEnv<'a>, value: impl AsRef<str>) -> NapiResult<JsString<'a>> {
        let value = napi_call!(
            =napi_create_string_utf8,
            env.raw(),
            value.as_ref().as_ptr() as *const c_char,
            value.as_ref().len(),
        );

        Ok(JsString(JsValue::from_raw(env, value)))
    }
}

impl<'a> ValueInner for JsString<'a> {
    fn downcast(&self) -> JsValue {
        self.0
    }
}
