use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsBuffer<'a>(pub(crate) JsValue<'a>);

impl<'a> JsBuffer<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsBuffer {
        JsBuffer(value)
    }
}

impl<'a> NapiValueT for JsBuffer<'a> {
    fn inner(&self) -> JsValue {
        self.0
    }
}
