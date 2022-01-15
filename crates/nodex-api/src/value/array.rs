use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsArray<'a>(pub(crate) JsValue<'a>);

impl<'a> JsArray<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsArray {
        JsArray(value)
    }

    pub fn new(env: NapiEnv<'a>, value: impl AsRef<str>) -> NapiResult<JsArray<'a>> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_create_array(env.raw(), result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(JsArray(JsValue::from_raw(env, value)))
    }

    /// get the size of array
    pub fn size(&self) -> NapiResult<u32> {
        let len = unsafe {
            let mut result = MaybeUninit::uninit();
            let status =
                api::napi_get_array_length(self.env().raw(), self.raw(), result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }
            result.assume_init()
        };

        Ok(len)
    }
}

impl<'a> ValueInner for JsArray<'a> {
    fn downcast(&self) -> JsValue {
        self.0
    }
}
