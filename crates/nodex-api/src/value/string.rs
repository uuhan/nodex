use std::{mem::MaybeUninit, os::raw::c_char};

use crate::{api, prelude::*};

#[derive(Copy, Clone, Debug)]
pub struct JsString<'a>(pub(crate) JsValue<'a>);

impl<'a> JsString<'a> {
    pub fn from_value(value: JsValue) -> JsString {
        JsString(value)
    }

    pub fn new(env: Env<'a>, value: impl AsRef<str>) -> NapiResult<JsString<'a>> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_create_string_utf8(
                env.raw(),
                value.as_ref().as_ptr() as *const c_char,
                value.as_ref().len(),
                result.as_mut_ptr(),
            );

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(JsString(JsValue::from_raw(env, value)))
    }
}

impl<'a> ValueInner for JsString<'a> {
    fn downcast(&self) -> JsValue {
        self.0
    }
}
