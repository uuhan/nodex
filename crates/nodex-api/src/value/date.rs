use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsDate(pub(crate) JsValue);

impl JsDate {
    pub(crate) fn from_value(value: JsValue) -> JsDate {
        JsDate(value)
    }

    #[cfg(feature = "v5")]
    /// This API does not observe leap seconds; they are ignored, as ECMAScript aligns with POSIX time specification.
    /// This API allocates a JavaScript Date object.
    /// JavaScript Date objects are described in Section 20.3 of the ECMAScript Language Specification.
    pub fn new(env: NapiEnv, time: f64) -> NapiResult<JsDate> {
        let value = napi_call!(=napi_create_date, env.raw(), time);
        Ok(JsDate(JsValue::from_raw(env, value)))
    }

    #[cfg(feature = "v5")]
    /// This API does not observe leap seconds; they are ignored, as ECMAScript aligns with POSIX time specification.
    /// Returns napi_ok if the API succeeded. If a non-date napi_value is passed in it returns napi_date_expected.
    /// This API returns the C double primitive of time value for the given JavaScript Date.
    pub fn get(&self) -> NapiResult<f64> {
        Ok(napi_call!(=napi_get_date_value, self.env(), self.raw()))
    }
}

napi_value_t!(JsDate);
