use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsUndefined<'a>(pub(crate) JsValue<'a>);

impl<'a> JsUndefined<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsUndefined {
        JsUndefined(value)
    }

    pub fn new(env: NapiEnv<'a>, value: impl AsRef<str>) -> NapiResult<JsUndefined<'a>> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_get_undefined(env.raw(), result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(JsUndefined(JsValue::from_raw(env, value)))
    }
}

impl<'a> ValueInner for JsUndefined<'a> {
    fn downcast(&self) -> JsValue {
        self.0
    }
}
