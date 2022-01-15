use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsArrayBuffer<'a>(pub(crate) JsValue<'a>);

impl<'a> JsArrayBuffer<'a> {
    pub fn from_value(value: JsValue) -> JsArrayBuffer {
        JsArrayBuffer(value)
    }

    /// create new array_buffer from slice-like data
    pub fn new(env: NapiEnv<'a>, value: impl AsRef<[u8]>) -> NapiResult<JsArrayBuffer<'a>> {
        let bytes = value.as_ref();
        let len = bytes.len();

        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let mut data = MaybeUninit::uninit();

            let status = api::napi_create_arraybuffer(
                env.raw(),
                len,
                data.as_mut_ptr(),
                result.as_mut_ptr(),
            );

            if status.err() {
                return Err(status);
            }

            let data = data.assume_init();
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), data as *mut u8, len);

            result.assume_init()
        };

        Ok(JsArrayBuffer(JsValue::from_raw(env, value)))
    }

    /// get the underlaying slice
    pub fn get(&self) -> NapiResult<&[u8]> {
        unsafe {
            let mut result = MaybeUninit::uninit();
            let mut len = MaybeUninit::uninit();
            let status = api::napi_get_arraybuffer_info(
                self.env().raw(),
                self.raw(),
                result.as_mut_ptr(),
                len.as_mut_ptr(),
            );

            if status.err() {
                return Err(status);
            }

            let (result, len) = (result.assume_init(), len.assume_init());
            let slice = std::slice::from_raw_parts(result as _, len);

            Ok(slice)
        }
    }
}

impl<'a> ValueInner for JsArrayBuffer<'a> {
    fn downcast(&self) -> JsValue {
        self.0
    }
}
