use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsBigInt<'a>(pub(crate) JsValue<'a>);

impl<'a> JsBigInt<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsBigInt {
        JsBigInt(value)
    }

    // TODO: [napi](https://nodejs.org/api/n-api.html)
}

impl<'a> NapiValueT for JsBigInt<'a> {
    fn inner(&self) -> JsValue {
        self.0
    }
}
