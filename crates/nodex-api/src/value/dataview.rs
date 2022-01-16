use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsDataView(pub(crate) JsValue);

impl JsDataView {
    pub(crate) fn from_value(value: JsValue) -> JsDataView {
        JsDataView(value)
    }
}

impl NapiValueT for JsDataView {
    fn value(&self) -> JsValue {
        self.0
    }
}
