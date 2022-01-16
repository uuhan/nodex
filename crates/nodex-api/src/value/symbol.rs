use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsSymbol<'a>(pub(crate) JsValue<'a>);

impl<'a> JsSymbol<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsSymbol {
        JsSymbol(value)
    }
}

impl<'a> NapiValueT for JsSymbol<'a> {
    fn inner(&self) -> JsValue {
        self.0
    }
}
