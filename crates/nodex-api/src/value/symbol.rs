use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsSymbol<'a>(pub(crate) JsValue<'a>);

impl<'a> JsSymbol<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsSymbol {
        JsSymbol(value)
    }

    /// This API creates a JavaScript symbol value from a UTF8-encoded C string.
    /// The JavaScript symbol type is described in Section 19.4 of the ECMAScript Language Specification.
    pub fn new(env: NapiEnv<'a>) -> NapiResult<JsSymbol<'a>> {
        let value = napi_call!(=napi_create_symbol, env.raw(), std::ptr::null_mut());
        Ok(JsSymbol::from_value(JsValue::from_raw(env, value)))
    }

    /// Symbol with description.
    pub fn description(env: NapiEnv<'a>, desc: JsString) -> NapiResult<JsSymbol<'a>> {
        let value = napi_call!(=napi_create_symbol, env.raw(), desc.raw());
        Ok(JsSymbol::from_value(JsValue::from_raw(env, value)))
    }
}

impl<'a> NapiValueT for JsSymbol<'a> {
    fn inner(&self) -> JsValue {
        self.0
    }
}
