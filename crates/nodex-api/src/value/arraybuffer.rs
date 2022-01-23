use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsArrayBuffer(pub(crate) JsValue);

impl JsArrayBuffer {
    pub(crate) fn from_value(value: JsValue) -> JsArrayBuffer {
        JsArrayBuffer(value)
    }

    /// This API returns a Node-API value corresponding to a JavaScript ArrayBuffer. ArrayBuffers are used to represent fixed-length binary data buffers. They are normally used as a backing-buffer for TypedArray objects. The ArrayBuffer allocated will have an underlying byte buffer whose size is determined by the length parameter that's passed in. The underlying buffer is optionally returned back to the caller in case the caller wants to directly manipulate the buffer. This buffer can only be written to directly from native code. To write to this buffer from JavaScript, a typed array or DataView object would need to be created.
    /// JavaScript ArrayBuffer objects are described in Section 24.1 of the ECMAScript Language Specification.
    pub fn new(env: NapiEnv, buffer: impl AsRef<[u8]>) -> NapiResult<JsArrayBuffer> {
        let bytes = buffer.as_ref();
        let len = bytes.len();

        let mut data = MaybeUninit::uninit();
        let buffer = napi_call!(
            =napi_create_arraybuffer,
            env,
            len,
            data.as_mut_ptr(),
        );

        unsafe {
            let data = data.assume_init();
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), data as *mut u8, len);
        }

        Ok(JsArrayBuffer(JsValue::from_raw(env, buffer)))
    }

    /// This API creates a JavaScript DataView object over an existing ArrayBuffer. DataView
    /// objects provide an array-like view over an underlying data buffer, but one which allows
    /// items of different size and type in the ArrayBuffer.
    ///
    /// It is required that byte_length + byte_offset is less than or equal to the size in bytes
    /// of the array passed in. If not, a RangeError exception is raised.
    ///
    /// JavaScript DataView objects are described in Section 24.3 of the ECMAScript Language Specification.
    pub fn view(&self, offset: usize, length: usize) -> NapiResult<JsDataView> {
        let value = napi_call!(=napi_create_dataview, self.env(), length, self.raw(), offset);
        Ok(JsDataView::from_raw(self.env(), value))
    }

    /// This API creates a JavaScript TypedArray object over an existing ArrayBuffer. TypedArray
    /// objects provide an array-like view over an underlying data buffer where each element has
    /// the same underlying binary scalar datatype.
    ///
    /// It's required that (length * size_of_element) + byte_offset should be <= the size in bytes
    /// of the array passed in. If not, a RangeError exception is raised.
    ///
    /// JavaScript TypedArray objects are described in Section 22.2 of the ECMAScript Language
    /// Specification.
    pub fn typedarray(
        &self,
        typed: NapiTypedarrayType,
        offset: usize,
        length: usize,
    ) -> NapiResult<JsTypedArray> {
        let typed =
            napi_call!(=napi_create_typedarray, self.env(), typed, length, self.raw(), offset);
        Ok(JsTypedArray::from_raw(self.env(), typed))
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
            self.env(),
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

    #[cfg(feature = "v7")]
    /// If a non-detachable ArrayBuffer is passed in it returns napi_detachable_arraybuffer_expected.
    ///
    /// Generally, an ArrayBuffer is non-detachable if it has been detached before. The engine
    /// may impose additional conditions on whether an ArrayBuffer is detachable. For example,
    /// V8 requires that the ArrayBuffer be external, that is, created with napi_create_external_arraybuffer.
    ///
    /// This API represents the invocation of the ArrayBuffer detach operation as defined in
    /// Section 24.1.1.3 of the ECMAScript Language Specification.
    pub fn detach(&mut self) -> NapiResult<()> {
        napi_call!(napi_detach_arraybuffer, self.env(), self.raw());
        Ok(())
    }

    #[cfg(feature = "v7")]
    /// The ArrayBuffer is considered detached if its internal data is null.
    ///
    /// This API represents the invocation of the ArrayBuffer IsDetachedBuffer operation as
    /// defined in Section 24.1.1.2 of the ECMAScript Language Specification.
    pub fn is_detached(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_detached_arraybuffer, self.env(), self.raw()))
    }
}

napi_value_t!(JsArrayBuffer);
