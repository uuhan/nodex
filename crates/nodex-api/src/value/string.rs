use std::{mem::MaybeUninit, os::raw::c_char};

use crate::{api, prelude::*};

#[derive(Copy, Clone, Debug)]
pub struct JsString<'a>(pub(crate) Value<'a>);

impl<'a> JsString<'a> {
    pub fn from_value(value: Value) -> JsString {
        JsString(value)
    }

    pub fn env(&self) -> Env<'a> {
        self.0.env()
    }

    pub fn raw(&self) -> napi_value {
        self.0.raw()
    }

    pub fn value(&self) -> Value<'a> {
        self.0
    }

    pub fn new(env: Env<'a>, value: impl AsRef<str>) -> NapiResult<JsString<'a>> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_create_string_utf8(
                env.raw(),
                value.as_ref().as_ptr() as *const c_char,
                value.as_ref().len() as u64,
                result.as_mut_ptr(),
            );

            if status != NapiStatus::Ok {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(JsString(Value::from_raw(env, value)))
    }
}
