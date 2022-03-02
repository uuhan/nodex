use crate::{api, prelude::*};
use std::{marker::PhantomData, mem::MaybeUninit};

#[derive(Clone, Debug)]
pub struct NapiAsyncContext {
    env: NapiEnv,
    context: napi_async_context,
}

impl NapiAsyncContext {
    pub(crate) fn from_raw(env: NapiEnv, context: napi_async_context) -> NapiAsyncContext {
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
        let context = napi_call!(
            =napi_async_init,
            env,
            env.object()?.raw(),
            env.string(name)?.raw(),
        );

        Ok(NapiAsyncContext { env, context })
    }

    /// This API can be called even if there is a pending JavaScript exception.
    pub fn destroy(&mut self) -> NapiResult<()> {
        napi_call!(napi_async_destroy, self.env(), self.raw())
    }

    /// This method allows a JavaScript function object to be called from a native add-on.
    /// This API is similar to napi_call_function. However, it is used to call from native
    /// code back into JavaScript after returning from an async operation (when there is no
    /// other script on the stack). It is a fairly simple wrapper around node::MakeCallback.
    ///
    /// Note it is not necessary to use napi_make_callback from within a napi_async_complete_callback;
    /// in that situation the callback's async context has already been set up, so a direct call to
    /// napi_call_function is sufficient and appropriate. Use of the napi_make_callback function may
    /// be required when implementing custom async behavior that does not use napi_create_async_work.
    ///
    /// Any process.nextTicks or Promises scheduled on the microtask queue by JavaScript during
    /// he callback are ran before returning back to C/C++.
    pub fn make_callback<R, T>(&self, this: JsObject, func: Function<R>, args: T) -> NapiResult<R>
    where
        R: NapiValueT,
        T: ToJsArgs,
    {
        let env = self.env();
        let args = args
            .to_js_args(env)?
            .0
            .into_iter()
            .map(|value| value.raw())
            .collect::<Vec<_>>();

        let value = napi_call!(
            =napi_make_callback,
            self.env(),
            self.raw(),
            this.raw(),
            func.raw(),
            T::len(),
            args.as_ptr(),
        );

        Ok(R::from_raw(env, value))
    }

    #[cfg(feature = "v3")]
    /// There are cases (for example, resolving promises) where it is necessary to have the
    /// equivalent of the scope associated with a callback in place when making certain
    /// Node-API calls. If there is no other script on the stack the napi_open_callback_scope
    /// and napi_close_callback_scope functions can be used to open/close the required scope.
    pub fn scope(&self) -> NapiResult<NapiCallbackScope> {
        let env = self.env();
        let scope = napi_call!(
            =napi_open_callback_scope,
            env,
            env.object()?.raw(),
            self.raw(),
        );
        Ok(NapiCallbackScope::from_raw(env, scope))
    }
}

impl Drop for NapiAsyncContext {
    fn drop(&mut self) {
        self.destroy();
    }
}
