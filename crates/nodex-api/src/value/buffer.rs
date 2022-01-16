use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsBuffer(pub(crate) JsValue);

impl JsBuffer {
    pub(crate) fn from_value(value: JsValue) -> JsBuffer {
        JsBuffer(value)
    }
}

impl NapiValueT for JsBuffer {
    fn value(&self) -> JsValue {
        self.0
    }
}
