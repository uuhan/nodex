use crate::{api, prelude::*};
use std::borrow::Cow;
use std::mem::MaybeUninit;

#[derive(Copy, Clone, Debug)]
pub struct JsString<'a>(pub(crate) JsValue<'a>);

impl<'a> JsString<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsString {
        JsString(value)
    }

    /// Default js-string constructor.
    #[inline]
    pub fn new(env: NapiEnv<'a>, value: impl AsRef<str>) -> NapiResult<JsString<'a>> {
        Self::utf8(env, value)
    }

    /// Default js-string getter.
    #[inline]
    pub fn get(&self) -> NapiResult<String> {
        self.get_utf8()
    }

    /// This API creates a JavaScript string value from a UTF8-encoded C string. The native string is copied.
    /// The JavaScript string type is described in Section 6.1.4 of the ECMAScript Language Specification.
    pub fn utf8(env: NapiEnv<'a>, value: impl AsRef<str>) -> NapiResult<JsString<'a>> {
        let value = napi_call!(
            =napi_create_string_utf8,
            env.raw(),
            value.as_ref().as_ptr() as *const _,
            value.as_ref().len(),
        );

        Ok(JsString(JsValue::from_raw(env, value)))
    }

    /// This API returns the UTF8-encoded string corresponding the value passed in.
    pub fn get_utf8(&self) -> NapiResult<String> {
        let size = napi_call!(
            =napi_get_value_string_utf8,
            self.env().raw(),
            self.raw(),
            std::ptr::null_mut(),
            0,
        );

        let mut buffer = vec![0u8; size + 1];
        let size = napi_call!(
            =napi_get_value_string_utf8,
            self.env().raw(),
            self.raw(),
            buffer.as_mut_ptr() as *mut _,
            // should contains the NULL terminator
            size + 1,
        );

        unsafe {
            // remove trailing NULL
            buffer.set_len(size);
            Ok(String::from_utf8_unchecked(buffer))
        }
    }
}

impl<'a> NapiValueT for JsString<'a> {
    fn inner(&self) -> JsValue {
        self.0
    }
}

impl<'a> Into<String> for JsString<'a> {
    fn into(self) -> String {
        self.get_utf8().unwrap()
    }
}
