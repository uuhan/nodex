use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsObject<'a>(pub(crate) JsValue<'a>);

impl<'a> JsObject<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsObject {
        JsObject(value)
    }

    pub fn new(env: NapiEnv<'a>) -> NapiResult<JsObject<'a>> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_create_object(env.raw(), result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(JsObject(JsValue::from_raw(env, value)))
    }

    /// get value by key
    pub fn get(&self, key: impl ValueInner) -> NapiResult<JsValue> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_get_property(
                self.env().raw(),
                self.raw(),
                key.downcast().raw(),
                result.as_mut_ptr(),
            );

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(JsValue::from_raw(self.env(), value))
    }

    /// set value by key
    pub fn set(&mut self, key: impl ValueInner, value: impl ValueInner) -> NapiResult<()> {
        unsafe {
            let status = api::napi_set_property(
                self.env().raw(),
                self.raw(),
                key.downcast().raw(),
                value.downcast().raw(),
            );

            if status.err() {
                return Err(status);
            }

            Ok(())
        }
    }

    #[cfg(feature = "v8")]
    /// Object.freeze
    pub fn freeze(&mut self) -> NapiResult<()> {
        unsafe {
            let status = api::napi_object_freeze(
                self.env().raw(),
                self.raw(),
            );

            if status.err() {
                return Err(status);
            }

            Ok(())
        }
    }

    #[cfg(feature = "v8")]
    /// Object.seal
    pub fn seal(&mut self) -> NapiResult<()> {
        unsafe {
            let status = api::napi_object_seal(
                self.env().raw(),
                self.raw(),
            );

            if status.err() {
                return Err(status);
            }

            Ok(())
        }
    }
}

impl<'a> ValueInner for JsObject<'a> {
    fn downcast(&self) -> JsValue {
        self.0
    }
}
