use crate::{api, prelude::*};
use std::{marker::PhantomData, mem::MaybeUninit, os::raw::c_char};

#[derive(Debug, Copy, Clone)]
pub struct JsPromise<L: NapiValueT, R: NapiValueT>(
    pub(crate) JsValue,
    pub(crate) napi_deferred,
    PhantomData<L>,
    PhantomData<R>,
);

impl<L: NapiValueT + Copy, R: NapiValueT + Copy> JsPromise<L, R> {
    pub(crate) fn from_raw(value: JsValue, deferred: napi_deferred) -> JsPromise<L, R> {
        JsPromise(value, deferred, PhantomData, PhantomData)
    }

    pub fn env(&self) -> NapiEnv {
        self.0.env()
    }

    pub fn raw(&self) -> napi_value {
        self.0.raw()
    }

    pub fn value(&self) -> JsValue {
        self.0
    }

    /// This API creates a deferred object and a JavaScript promise.
    pub fn new(env: NapiEnv) -> NapiResult<JsPromise<L, R>> {
        let mut deferred = MaybeUninit::uninit();

        let promise = napi_call!(
            =napi_create_promise,
            env,
            deferred.as_mut_ptr(),
        );

        let deferred = unsafe { deferred.assume_init() };

        Ok(Self::from_raw(JsValue(env, promise), deferred))
    }

    /// This API resolves a JavaScript promise by way of the deferred object with which it is
    /// associated. Thus, it can only be used to resolve JavaScript promises for which the
    /// corresponding deferred object is available. This effectively means that the promise must
    /// have been created using napi_create_promise() and the deferred object returned from that
    /// call must have been retained in order to be passed to this API.
    ///
    /// The deferred object is freed upon successful completion.
    pub fn resolve(&self, resolution: L) -> NapiResult<()> {
        napi_call!(napi_resolve_deferred, self.env(), self.1, resolution.raw())
    }

    /// This API rejects a JavaScript promise by way of the deferred object with which it is
    /// associated. Thus, it can only be used to reject JavaScript promises for which the
    /// corresponding deferred object is available. This effectively means that the promise
    /// must have been created using napi_create_promise() and the deferred object returned
    /// from that call must have been retained in order to be passed to this API.
    ///
    /// The deferred object is freed upon successful completion.
    pub fn reject(&self, rejection: R) -> NapiResult<()> {
        napi_call!(napi_reject_deferred, self.env(), self.1, rejection.raw())
    }
}

impl<L: NapiValueT + Copy + 'static, R: NapiValueT + Copy + 'static> JsPromise<L, R> {
    /// Spawn a busy task in the libuv pool.
    pub fn spawn<T>(
        env: NapiEnv,
        mut work: impl FnMut(&mut T) + Send + 'static,
        mut complete: impl FnMut(Self, NapiStatus, T) -> NapiResult<()> + 'static,
    ) -> NapiResult<JsPromise<L, R>>
    where
        T: Default,
    {
        let promise: JsPromise<L, R> = JsPromise::new(env)?;
        env.async_work(
            "napi-promise-task",
            T::default(),
            move |state| work(state),
            // NB: execute in the main js thread.
            move |_, status, state| complete(promise, status, state),
        )?
        .queue()?;

        Ok(promise)
    }
}

impl<L: NapiValueT + Copy, R: NapiValueT + Copy> NapiValueCheck for JsPromise<L, R> {
    fn check(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_promise, self.env(), self.raw()))
    }
}
