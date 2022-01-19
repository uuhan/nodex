use crate::{api, prelude::*};
use std::{marker::PhantomData, mem::MaybeUninit};

#[derive(Clone, Debug)]
pub struct NapiAsyncContext {
    env: NapiEnv,
    context: napi_async_context,
}

impl NapiAsyncContext {
    pub(crate) fn from_value(env: NapiEnv, context: napi_async_context) -> NapiAsyncContext {
        NapiAsyncContext { env, context }
    }

    pub fn env(&self) -> NapiEnv {
        self.env
    }

    pub fn raw(&self) -> napi_async_context {
        self.context
    }

    /// The async_resource object needs to be kept alive until napi_async_destroy to keep async_hooks related API acts correctly. In order to retain ABI compatibility with previous versions, napi_async_contexts are not maintaining the strong reference to the async_resource objects to avoid introducing causing memory leaks. However, if the async_resource is garbage collected by JavaScript engine before the napi_async_context was destroyed by napi_async_destroy, calling napi_async_context related APIs like napi_open_callback_scope and napi_make_callback can cause problems like loss of async context when using the AsyncLocalStorage API.
    /// In order to retain ABI compatibility with previous versions, passing NULL for async_resource does not result in an error. However, this is not recommended as this will result poor results with async_hooks init hooks and async_hooks.executionAsyncResource() as the resource is now required by the underlying async_hooks implementation in order to provide the linkage between async callbacks.
    pub fn new(env: NapiEnv, name: impl AsRef<str>) -> NapiResult<NapiAsyncContext> {
        let context = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_async_init(
                env,
                env.object()?.raw(),
                env.string(name)?.raw(),
                result.as_mut_ptr(),
            );

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(NapiAsyncContext { env, context })
    }

    /// This API can be called even if there is a pending JavaScript exception.
    pub fn destroy(&mut self) -> NapiResult<()> {
        unsafe {
            let status = api::napi_async_destroy(self.env(), self.raw());

            if status.err() {
                return Err(status);
            }

            Ok(())
        }
    }
}

impl Drop for NapiAsyncContext {
    fn drop(&mut self) {
        self.destroy();
    }
}
