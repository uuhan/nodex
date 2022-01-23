use crate::{api, prelude::*};
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub struct JsBigInt<T>(pub(crate) JsValue, PhantomData<T>);

impl<T> JsBigInt<T> {
    pub(crate) fn from_value(value: JsValue) -> JsBigInt<T> {
        JsBigInt(value, PhantomData)
    }

    #[cfg(feature = "v6")]
    /// This API converts the C int64_t type to the JavaScript BigInt type.
    pub fn new_i64(env: NapiEnv, value: i64) -> NapiResult<JsBigInt<i64>> {
        let value = napi_call!(=napi_create_bigint_int64, env, value);
        Ok(JsBigInt::from_raw(env, value))
    }

    #[cfg(feature = "v6")]
    /// This API converts the C unt64_t type to the JavaScript BigInt type.
    pub fn new_u64(env: NapiEnv, value: u64) -> NapiResult<JsBigInt<u64>> {
        let value = napi_call!(=napi_create_bigint_uint64, env, value);
        Ok(JsBigInt::from_raw(env, value))
    }
}

impl<T> NapiValueT for JsBigInt<T> {
    fn from_raw(env: NapiEnv, raw: napi_value) -> JsBigInt<T> {
        JsBigInt(JsValue(env, raw), PhantomData)
    }

    fn value(&self) -> JsValue {
        self.0
    }
}
