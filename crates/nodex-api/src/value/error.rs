use crate::{api, prelude::*};
use std::ffi::CString;

#[derive(Copy, Clone, Debug)]
pub struct JsError(pub(crate) JsValue);

impl JsError {
    pub(crate) fn from_value(value: JsValue) -> JsError {
        JsError(value)
    }

    /// Return Error
    #[inline]
    pub fn error(
        env: NapiEnv,
        msg: impl AsRef<str>,
        code: Option<impl AsRef<str>>,
    ) -> NapiResult<JsError> {
        let code = if let Some(code) = code {
            env.string(code.as_ref())?.raw()
        } else {
            std::ptr::null_mut()
        };
        let msg = env.string(msg.as_ref())?;
        let err = napi_call!(=napi_create_error, env, code, msg.raw());
        Ok(JsError(JsValue(env, err)))
    }

    /// Return TypeError
    #[inline]
    pub fn type_error(
        env: NapiEnv,
        msg: impl NapiValueT,
        code: Option<impl NapiValueT>,
    ) -> NapiResult<JsError> {
        let code = if let Some(code) = code {
            code.value().raw()
        } else {
            std::ptr::null_mut()
        };
        let err = napi_call!(=napi_create_type_error, env, code, msg.value().raw());
        Ok(JsError(JsValue(env, err)))
    }

    /// Return RangeError
    pub fn range_error(
        env: NapiEnv,
        msg: impl NapiValueT,
        code: Option<impl NapiValueT>,
    ) -> NapiResult<JsError> {
        let code = if let Some(code) = code {
            code.value().raw()
        } else {
            std::ptr::null_mut()
        };
        let err = napi_call!(=napi_create_range_error, env, code, msg.value().raw());
        Ok(JsError(JsValue(env, err)))
    }
}

napi_value_t!(JsError);

impl NapiValueCheck for JsError {
    fn check(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_error, self.env(), self.raw()))
    }
}
