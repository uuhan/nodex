use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

/// Js Dataview
///
/// * N - Offset
/// * M - Length
#[derive(Copy, Clone, Debug)]
pub struct JsDataView(pub(crate) JsValue);

impl JsDataView {
    pub(crate) fn from_value(value: JsValue) -> JsDataView {
        JsDataView(value)
    }

    /// Any of the out parameters may be NULL if that property is unneeded.
    ///
    /// This API returns various properties of a DataView.
    ///
    /// Info includes (void* data, ArrayBuffer, offset, length)
    pub fn info(&self) -> NapiResult<(DataPointer, JsArrayBuffer, usize, usize)> {
        let mut length = MaybeUninit::uninit();
        let mut data = MaybeUninit::uninit();
        let mut buffer = MaybeUninit::uninit();
        let offset = napi_call!(
            =napi_get_dataview_info,
            self.env(),
            self.raw(),
            length.as_mut_ptr(),
            data.as_mut_ptr(),
            buffer.as_mut_ptr(),
        );
        unsafe {
            let length = length.assume_init();
            let data = data.assume_init();
            let buffer = buffer.assume_init();
            Ok((
                data,
                JsArrayBuffer::from_raw(self.env(), buffer),
                offset,
                length,
            ))
        }
    }
}

napi_value_t!(JsDataView);
