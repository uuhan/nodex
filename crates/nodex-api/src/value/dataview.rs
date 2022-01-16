use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsDataView<'a>(pub(crate) JsValue<'a>);

impl<'a> JsDataView<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsDataView {
        JsDataView(value)
    }
}

impl<'a> NapiValueT for JsDataView<'a> {
    fn inner(&self) -> JsValue {
        self.0
    }
}
