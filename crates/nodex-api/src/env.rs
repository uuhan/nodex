use std::{marker::PhantomData, mem::MaybeUninit};

use crate::{
    api::{self, napi_node_version},
    prelude::*,
};

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

    /// get current global object
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

    /// get node version
    /// the returned buffer is statically allocated and does not need to be freed.
    pub fn node_version(&self) -> NapiResult<napi_node_version> {
        unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_get_node_version(self.raw(), result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            Ok(std::ptr::read(result.assume_init()))
        }
    }

    /// get napi version
    pub fn napi_version(&self) -> NapiResult<u32> {
        unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_get_version(self.raw(), result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            Ok(result.assume_init())
        }
    }
}
