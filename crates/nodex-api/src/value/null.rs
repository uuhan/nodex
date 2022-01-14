use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsNull<'a>(pub(crate) JsValue<'a>);

impl<'a> JsNull<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsNull {
        JsNull(value)
    }

    pub fn new(env: Env<'a>, value: impl AsRef<str>) -> NapiResult<JsNull<'a>> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_get_null(env.raw(), result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(JsNull(JsValue::from_raw(env, value)))
    }
}

impl<'a> ValueInner for JsNull<'a> {
    fn downcast(&self) -> JsValue {
        self.0
    }
}
