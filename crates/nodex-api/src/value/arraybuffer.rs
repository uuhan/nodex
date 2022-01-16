use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsArrayBuffer<'a>(pub(crate) JsValue<'a>);

impl<'a> JsArrayBuffer<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsArrayBuffer {
        JsArrayBuffer(value)
    }

    /// This API returns a Node-API value corresponding to a JavaScript ArrayBuffer. ArrayBuffers are used to represent fixed-length binary data buffers. They are normally used as a backing-buffer for TypedArray objects. The ArrayBuffer allocated will have an underlying byte buffer whose size is determined by the length parameter that's passed in. The underlying buffer is optionally returned back to the caller in case the caller wants to directly manipulate the buffer. This buffer can only be written to directly from native code. To write to this buffer from JavaScript, a typed array or DataView object would need to be created.
    /// JavaScript ArrayBuffer objects are described in Section 24.1 of the ECMAScript Language Specification.
    pub fn new(env: NapiEnv<'a>, value: impl AsRef<[u8]>) -> NapiResult<JsArrayBuffer<'a>> {
        let bytes = value.as_ref();
        let len = bytes.len();

        let mut data = MaybeUninit::uninit();
        let value = napi_call!(
            =napi_create_arraybuffer,
            env.raw(),
            len,
            data.as_mut_ptr(),
        );

        unsafe {
            let data = data.assume_init();
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), data as *mut u8, len);
        }

        Ok(JsArrayBuffer(JsValue::from_raw(env, value)))
    }

    /// This API is used to retrieve the underlying data buffer of an ArrayBuffer and its length.
    ///
    /// WARNING: Use caution while using this API. The lifetime of the underlying data buffer
    /// is managed by the ArrayBuffer even after it's returned. A possible safe way to use this
    /// API is in conjunction with napi_create_reference, which can be used to guarantee control
    /// over the lifetime of the ArrayBuffer. It's also safe to use the returned data buffer
    /// within the same callback as long as there are no calls to other APIs that might trigger
    /// a GC.
    pub fn get_arraybuffer_info(&self) -> NapiResult<&[u8]> {
        let mut result = MaybeUninit::uninit();
        let mut len = MaybeUninit::uninit();

        napi_call!(
            napi_get_arraybuffer_info,
            self.env().raw(),
            self.raw(),
            result.as_mut_ptr(),
            len.as_mut_ptr(),
        );

        unsafe {
            let (result, len) = (result.assume_init(), len.assume_init());
            let slice = std::slice::from_raw_parts(result as _, len);
            Ok(slice)
        }
    }
}

impl<'a> NapiValueT for JsArrayBuffer<'a> {
    fn inner(&self) -> JsValue {
        self.0
    }
}
