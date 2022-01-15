use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsNumber<'a>(pub(crate) JsValue<'a>);

impl<'a> JsNumber<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsNumber {
        JsNumber(value)
    }

    /// create a number
    pub fn new(env: NapiEnv<'a>, value: f64) -> NapiResult<JsNumber<'a>> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_create_double(env.raw(), value, result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(JsNumber(JsValue::from_raw(env, value)))
    }

    /// get the underlaying number
    pub fn get(&self) -> NapiResult<f64> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status =
                api::napi_get_value_double(self.env().raw(), self.raw(), result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(value)
    }
}

impl<'a> ValueInner for JsNumber<'a> {
    fn downcast(&self) -> JsValue {
        self.0
    }
}
