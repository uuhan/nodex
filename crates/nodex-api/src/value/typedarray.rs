use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsTypedArray(pub(crate) JsValue);

impl JsTypedArray {
    pub(crate) fn from_value(value: JsValue) -> JsTypedArray {
        JsTypedArray(value)
    }
}

napi_value_t!(JsTypedArray);
