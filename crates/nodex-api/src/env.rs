use std::{marker::PhantomData, mem::MaybeUninit};

use crate::{api, prelude::*};

#[derive(Clone, Copy, Debug)]
pub struct Env<'a>(napi_env, PhantomData<&'a napi_env>);

impl<'a> AsRef<napi_env> for Env<'a> {
    fn as_ref(&self) -> &napi_env {
        &self.0
    }
}

impl<'a> Env<'a> {
    /// create `Env` from raw napi_env
    pub fn from_raw(env: napi_env) -> Env<'a> {
        Env(env, PhantomData)
    }

    /// access raw napi_env from `Env`
    pub fn raw(&self) -> napi_env {
        self.0
    }

    pub fn global(&self) -> NapiResult<JsValue> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_get_global(self.raw(), result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(JsValue::from_raw(*self, value))
    }
}
