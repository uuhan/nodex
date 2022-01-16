use crate::{api, prelude::*};
use std::mem::MaybeUninit;

#[derive(Clone, Debug)]
pub struct NapiHandleScope {
    env: NapiEnv,
    handle: napi_handle_scope,
}

impl NapiHandleScope {
    pub(crate) fn from_value(env: NapiEnv, handle: napi_handle_scope) -> NapiHandleScope {
        NapiHandleScope { env, handle }
    }

    pub fn env(&self) -> NapiEnv {
        self.env
    }

    pub fn raw(&self) -> napi_handle_scope {
        self.handle
    }

    /// create a new napi_handle_scope
    pub fn new(env: NapiEnv) -> NapiResult<NapiHandleScope> {
        let handle = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_open_handle_scope(env.raw(), result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(NapiHandleScope { env, handle })
    }

    /// close this napi_handle_scope
    pub fn close(&mut self) -> NapiResult<()> {
        unsafe {
            let status = api::napi_close_handle_scope(self.env().raw(), self.raw());

            if status.err() {
                return Err(status);
            }

            Ok(())
        }
    }
}

impl Drop for NapiHandleScope {
    fn drop(&mut self) {
        self.close();
    }
}
