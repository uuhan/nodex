use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsTypedArray<'a>(pub(crate) JsValue<'a>);

impl<'a> JsTypedArray<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsTypedArray {
        JsTypedArray(value)
    }
}

impl<'a> NapiValueT for JsTypedArray<'a> {
    fn inner(&self) -> JsValue {
        self.0
    }
}
