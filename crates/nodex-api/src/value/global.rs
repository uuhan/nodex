use crate::{api, prelude::*};

#[derive(Copy, Clone, Debug)]
pub struct JsGlobal<'a>(pub(crate) JsValue<'a>);

impl<'a> JsGlobal<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsGlobal {
        JsGlobal(value)
    }

    /// This API returns the global object.
    pub fn new(env: NapiEnv<'a>) -> NapiResult<JsGlobal<'a>> {
        let value = napi_call!(=napi_get_global, env.raw());
        Ok(JsGlobal(JsValue::from_raw(env, value)))
    }
}

impl<'a> NapiValueT for JsGlobal<'a> {
    fn inner(&self) -> JsValue {
        self.0
    }
}
