use crate::{api, prelude::*};

#[derive(Copy, Clone, Debug)]
pub struct JsGlobal(pub(crate) JsValue);

impl JsGlobal {
    pub(crate) fn from_value(value: JsValue) -> JsGlobal {
        JsGlobal(value)
    }

    /// This API returns the global object.
    pub fn new(env: NapiEnv) -> NapiResult<JsGlobal> {
        let value = napi_call!(=napi_get_global, env);
        Ok(JsGlobal(JsValue::from_raw(env, value)))
    }
}

napi_value_t!(JsGlobal);

impl NapiValueCheck for JsGlobal {
    fn check(&self) -> NapiResult<bool> {
        // FIXME: global check
        Ok(self.kind()? == NapiValuetype::Object)
    }
}
