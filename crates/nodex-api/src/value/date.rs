use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsDate<'a>(pub(crate) JsValue<'a>);

impl<'a> JsDate<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsDate {
        JsDate(value)
    }
}

impl<'a> ValueInner for JsDate<'a> {
    fn downcast(&self) -> JsValue {
        self.0
    }
}
