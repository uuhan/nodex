use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsBigInt(pub(crate) JsValue);

impl JsBigInt {
    pub(crate) fn from_value(value: JsValue) -> JsBigInt {
        JsBigInt(value)
    }

    // TODO: [napi](https://nodejs.org/api/n-api.html)
}

impl NapiValueT for JsBigInt {
    fn value(&self) -> JsValue {
        self.0
    }
}
