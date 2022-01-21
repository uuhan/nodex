use crate::{api, prelude::*};

#[derive(Copy, Clone, Debug)]
pub struct CallbackInfo(NapiEnv, napi_callback_info);

impl CallbackInfo {
    pub(crate) fn from_raw(env: NapiEnv, info: napi_callback_info) -> Self {
        CallbackInfo(env, info)
    }

    pub fn env(&self) -> NapiEnv {
        self.0
    }

    pub fn raw(&self) -> napi_callback_info {
        self.1
    }

    /// This API returns the new.target of the constructor call. If the current callback is not a
    /// constructor call, the result is NULL.
    pub fn get_new_target(&self) -> NapiResult<Option<JsObject>> {
        let value = napi_call!(=napi_get_new_target, self.env(), self.raw());
        if value.is_null() {
            Ok(None)
        } else {
            Ok(Some(JsObject::from_raw(self.env(), value)))
        }
    }
}
