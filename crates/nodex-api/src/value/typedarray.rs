use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsTypedArray(pub(crate) JsValue);

impl JsTypedArray {
    pub(crate) fn from_value(value: JsValue) -> JsTypedArray {
        JsTypedArray(value)
    }
}

impl NapiValueT for JsTypedArray {
    fn value(&self) -> JsValue {
        self.0
    }
}
