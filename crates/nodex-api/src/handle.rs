use crate::{api, prelude::*};
use std::mem::MaybeUninit;

#[derive(Copy, Clone, Debug)]
pub struct NapiHandleScope<'a> {
    env: NapiEnv<'a>,
    handle: napi_handle_scope,
}

impl<'a> NapiHandleScope<'a> {
    pub(crate) fn from_value(env: NapiEnv<'a>, handle: napi_handle_scope) -> NapiHandleScope {
        NapiHandleScope { env, handle }
    }

    pub fn env(&self) -> NapiEnv<'a> {
        self.env
    }

    pub fn raw(&self) -> napi_handle_scope {
        self.handle
    }

    /// create a new napi_handle_scope
    pub fn new(env: NapiEnv<'a>) -> NapiResult<NapiHandleScope<'a>> {
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
