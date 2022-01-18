use crate::{api, prelude::*};
use std::mem::MaybeUninit;

/// Node-API provides the ability to establish a new 'scope' to which newly created handles
/// will be associated. Once those handles are no longer required, the scope can be 'closed'
/// and any handles associated with the scope are invalidated. The methods available to
/// open/close scopes are napi_open_handle_scope and napi_close_handle_scope.
///
/// Node-API only supports a single nested hierarchy of scopes. There is only one active scope
/// at any time, and all new handles will be associated with that scope while it is active.
/// Scopes must be closed in the reverse order from which they are opened. In addition,
/// all scopes created within a native method must be closed before returning from that method.
#[derive(Clone, Debug)]
pub struct NapiHandleScope(NapiEnv, napi_handle_scope);

impl NapiHandleScope {
    pub(crate) fn from_value(env: NapiEnv, handle: napi_handle_scope) -> NapiHandleScope {
        NapiHandleScope(env, handle)
    }

    pub fn env(&self) -> NapiEnv {
        self.0
    }

    pub fn raw(&self) -> napi_handle_scope {
        self.1
    }

    /// This API opens a new scope.
    pub fn open(env: NapiEnv) -> NapiResult<NapiHandleScope> {
        let handle = napi_call!(=napi_open_handle_scope, env);
        Ok(NapiHandleScope(env, handle))
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
        if let Err(e) = self.close() {
            log::warn!("[{}] napi_close_handle_scope failed.", e);
        }
    }
}

/// When nesting scopes, there are cases where a handle from an inner scope needs to live
/// beyond the lifespan of that scope. Node-API supports an 'escapable scope' in order to
/// support this case. An escapable scope allows one handle to be 'promoted' so that it
/// 'escapes' the current scope and the lifespan of the handle changes from the current scope
/// to that of the outer scope.
///
/// The methods available to open/close escapable scopes are napi_open_escapable_handle_scope
/// and napi_close_escapable_handle_scope.
///
/// The request to promote a handle is made through napi_escape_handle which can only be
/// called once.
#[derive(Clone, Debug)]
pub struct NapiEscapableHandleScope(NapiEnv, napi_escapable_handle_scope);

impl NapiEscapableHandleScope {
    pub(crate) fn from_value(
        env: NapiEnv,
        handle: napi_escapable_handle_scope,
    ) -> NapiEscapableHandleScope {
        NapiEscapableHandleScope(env, handle)
    }

    pub fn env(&self) -> NapiEnv {
        self.0
    }

    pub fn raw(&self) -> napi_escapable_handle_scope {
        self.1
    }

    /// This API opens a new scope from which one object can be promoted to the outer scope.
    pub fn open(env: NapiEnv) -> NapiResult<NapiEscapableHandleScope> {
        let handle = napi_call!(=napi_open_escapable_handle_scope, env);
        Ok(NapiEscapableHandleScope(env, handle))
    }

    /// This API closes the scope passed in. Scopes must be closed in the reverse
    /// order from which they were created.
    /// This API can be called even if there is a pending JavaScript exception.
    pub fn close(&mut self) -> NapiResult<()> {
        napi_call!(napi_close_escapable_handle_scope, self.env(), self.raw());
        Ok(())
    }

    /// This API promotes the handle to the JavaScript object so that it is valid for
    /// the lifetime of the outer scope. It can only be called once per scope. If it is
    /// called more than once an error will be returned.
    ///
    /// This API can be called even if there is a pending JavaScript exception.
    pub fn escape<T: NapiValueT>(&mut self, escapee: T) -> NapiResult<T> {
        let escapee = napi_call!(=napi_escape_handle, self.env(), self.1, escapee.raw());
        Ok(T::from_raw(self.env(), escapee))
    }
}

impl Drop for NapiEscapableHandleScope {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            log::warn!("[{}] napi_close_escapable_handle_scope failed.", e);
        }
    }
}
