use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsBigInt<'a>(pub(crate) JsValue<'a>);

impl<'a> JsBigInt<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsBigInt {
        JsBigInt(value)
    }
}

impl<'a> ValueInner for JsBigInt<'a> {
    fn downcast(&self) -> JsValue {
        self.0
    }
}
