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

    /// This API opens a new scope.
    pub fn open(env: NapiEnv) -> NapiResult<NapiHandleScope> {
        let handle = napi_call!(=napi_open_handle_scope, env);
        Ok(NapiHandleScope { env, handle })
    }

    /// This API closes the scope passed in. Scopes must be closed in the reverse
    /// order from which they were created.
    /// This API can be called even if there is a pending JavaScript exception.
    pub fn close(&mut self) -> NapiResult<()> {
        napi_call!(napi_close_handle_scope, self.env(), self.raw());
        Ok(())
    }
}

impl Drop for NapiHandleScope {
    fn drop(&mut self) {
        self.close();
    }
}

#[derive(Clone, Debug)]
pub struct NapiEscapableHandleScope {
    env: NapiEnv,
    handle: napi_escapable_handle_scope,
}

impl NapiEscapableHandleScope {
    pub(crate) fn from_value(
        env: NapiEnv,
        handle: napi_escapable_handle_scope,
    ) -> NapiEscapableHandleScope {
        NapiEscapableHandleScope { env, handle }
    }

    pub fn env(&self) -> NapiEnv {
        self.env
    }

    pub fn raw(&self) -> napi_escapable_handle_scope {
        self.handle
    }

    /// This API opens a new scope from which one object can be promoted to the outer scope.
    pub fn open(env: NapiEnv) -> NapiResult<NapiEscapableHandleScope> {
        let handle = napi_call!(=napi_open_escapable_handle_scope, env);
        Ok(NapiEscapableHandleScope { env, handle })
    }

    /// This API closes the scope passed in. Scopes must be closed in the reverse
    /// order from which they were created.
    /// This API can be called even if there is a pending JavaScript exception.
    pub fn close(&mut self) -> NapiResult<()> {
        napi_call!(napi_close_escapable_handle_scope, self.env(), self.raw());
        Ok(())
    }
}

impl Drop for NapiEscapableHandleScope {
    fn drop(&mut self) {
        self.close();
    }
}
