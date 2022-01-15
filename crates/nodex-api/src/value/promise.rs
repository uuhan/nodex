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
        let (promise, deferred) = unsafe {
            let mut promise = MaybeUninit::uninit();
            let mut deferred = MaybeUninit::uninit();
            let status =
                api::napi_create_promise(env.raw(), deferred.as_mut_ptr(), promise.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            (promise.assume_init(), deferred.assume_init())
        };

        Ok(JsPromise(JsValue::from_raw(env, promise), deferred))
    }

    /// This API resolves a JavaScript promise by way of the deferred object with which it is
    /// associated. Thus, it can only be used to resolve JavaScript promises for which the
    /// corresponding deferred object is available. This effectively means that the promise must
    /// have been created using napi_create_promise() and the deferred object returned from that
    /// call must have been retained in order to be passed to this API.
    ///
    /// The deferred object is freed upon successful completion.
    pub fn resolve(&self, resolution: impl ValueInner) -> NapiResult<()> {
        unsafe {
            let status = api::napi_resolve_deferred(self.env().raw(), self.1, resolution.raw());

            if status.err() {
                return Err(status);
            }

            Ok(())
        }
    }

    /// This API rejects a JavaScript promise by way of the deferred object with which it is
    /// associated. Thus, it can only be used to reject JavaScript promises for which the
    /// corresponding deferred object is available. This effectively means that the promise
    /// must have been created using napi_create_promise() and the deferred object returned
    /// from that call must have been retained in order to be passed to this API.
    ///
    /// The deferred object is freed upon successful completion.
    pub fn reject(&self, rejection: impl ValueInner) -> NapiResult<()> {
        unsafe {
            let status = api::napi_reject_deferred(self.env().raw(), self.1, rejection.raw());

            if status.err() {
                return Err(status);
            }

            Ok(())
        }
    }
}

impl<'a> ValueInner for JsPromise<'a> {
    fn downcast(&self) -> JsValue {
        self.0
    }
}
