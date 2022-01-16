use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsPromise<'a>(pub(crate) JsValue<'a>, pub(crate) napi_deferred);

impl<'a> JsPromise<'a> {
    pub(crate) fn from_value(value: JsValue, deferred: napi_deferred) -> JsPromise {
        JsPromise(value, deferred)
    }

    /// This API creates a deferred object and a JavaScript promise.
    pub fn new(env: NapiEnv<'a>) -> NapiResult<JsPromise<'a>> {
        let mut deferred = MaybeUninit::uninit();

        let promise = napi_call!(
            =napi_create_promise,
            env.raw(),
            deferred.as_mut_ptr(),
        );

        let deferred = unsafe { deferred.assume_init() };

        Ok(JsPromise(JsValue::from_raw(env, promise), deferred))
    }

    /// This API resolves a JavaScript promise by way of the deferred object with which it is
    /// associated. Thus, it can only be used to resolve JavaScript promises for which the
    /// corresponding deferred object is available. This effectively means that the promise must
    /// have been created using napi_create_promise() and the deferred object returned from that
    /// call must have been retained in order to be passed to this API.
    ///
    /// The deferred object is freed upon successful completion.
    pub fn resolve(&self, resolution: impl NapiValueT) -> NapiResult<()> {
        napi_call!(
            napi_resolve_deferred,
            self.env().raw(),
            self.1,
            resolution.raw()
        );
        Ok(())
    }

    /// This API rejects a JavaScript promise by way of the deferred object with which it is
    /// associated. Thus, it can only be used to reject JavaScript promises for which the
    /// corresponding deferred object is available. This effectively means that the promise
    /// must have been created using napi_create_promise() and the deferred object returned
    /// from that call must have been retained in order to be passed to this API.
    ///
    /// The deferred object is freed upon successful completion.
    pub fn reject(&self, rejection: impl NapiValueT) -> NapiResult<()> {
        napi_call!(
            napi_reject_deferred,
            self.env().raw(),
            self.1,
            rejection.raw()
        );
        Ok(())
    }
}

impl<'a> NapiValueT for JsPromise<'a> {
    fn inner(&self) -> JsValue {
        self.0
    }
}
