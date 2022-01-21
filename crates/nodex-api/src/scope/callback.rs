use crate::{api, prelude::*};
use std::mem::MaybeUninit;

/// There are cases (for example, resolving promises) where it is necessary to have the equivalent
/// of the scope associated with a callback in place when making certain Node-API calls. If there
/// is no other script on the stack the napi_open_callback_scope and napi_close_callback_scope
/// functions can be used to open/close the required scope.
#[derive(Clone, Debug)]
pub struct NapiCallbackScope(NapiEnv, napi_callback_scope);

impl NapiCallbackScope {
    pub(crate) fn from_raw(env: NapiEnv, scope: napi_callback_scope) -> NapiCallbackScope {
        NapiCallbackScope(env, scope)
    }

    pub fn env(&self) -> NapiEnv {
        self.0
    }

    pub fn raw(&self) -> napi_callback_scope {
        self.1
    }

    pub fn close(&mut self) -> NapiResult<()> {
        napi_call!(napi_close_callback_scope, self.env(), self.raw());
        Ok(())
    }
}

impl Drop for NapiCallbackScope {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            log::warn!("[{}] napi_close_callback_scope failed.", e);
        }
    }
}
