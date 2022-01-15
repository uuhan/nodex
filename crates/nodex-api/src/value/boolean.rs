use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsBoolean<'a>(pub(crate) JsValue<'a>);

impl<'a> JsBoolean<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsBoolean {
        JsBoolean(value)
    }

    /// create a boolean
    pub fn new(env: NapiEnv<'a>, value: bool) -> NapiResult<JsBoolean<'a>> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_get_boolean(env.raw(), value, result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(JsBoolean(JsValue::from_raw(env, value)))
    }

    /// get the underlaying boolean value
    pub fn get(&self) -> NapiResult<bool> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status =
                api::napi_get_value_bool(self.env().raw(), self.raw(), result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(value)
    }
}

impl<'a> ValueInner for JsBoolean<'a> {
    fn downcast(&self) -> JsValue {
        self.0
    }
}
