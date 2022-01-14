use std::marker::PhantomData;

use crate::api::napi_env;

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
}
