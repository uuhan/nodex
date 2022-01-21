use crate::{api, prelude::*};

#[derive(Copy, Clone, Debug)]
pub struct NapiThreadsafeFunction(NapiEnv, napi_threadsafe_function);

impl NapiThreadsafeFunction {
    pub(crate) fn from_raw(env: NapiEnv, tsfn: napi_threadsafe_function) -> Self {
        NapiThreadsafeFunction(env, tsfn)
    }

    pub fn env(&self) -> NapiEnv {
        self.0
    }

    pub fn raw(&self) -> napi_threadsafe_function {
        self.1
    }

    /// Create a napi_threadsafe_function
    pub fn new<R, C, Finalizer>(
        env: NapiEnv,
        func: Function<R>,
        context: C,
        finalizer: Finalizer,
        // call_js: napi_threadsafe_function_call_js,
    ) -> NapiResult<NapiThreadsafeFunction>
    where
        R: NapiValueT,
        Finalizer: FnOnce(NapiEnv) -> NapiResult<()>,
    {
        unsafe extern "C" fn finalizer_trampoline(
            env: NapiEnv,
            finalizer: DataPointer,
            _: DataPointer,
        ) {
            // NB: here we collect the memory of finalizer closure
            let finalizer: Box<Box<dyn FnOnce(NapiEnv) -> NapiResult<()>>> =
                Box::from_raw(finalizer as _);
            if let Err(err) = finalizer(env) {
                log::error!("NapiValueT::finalizer(): {}", err);
            }
        }

        unsafe extern "C" fn call_js_trampoline<C>(
            env: NapiEnv,
            cb: napi_value,
            context: DataPointer,
            data: DataPointer,
        ) {
            let context: &mut Box<C> = std::mem::transmute(context);
        }

        let finalizer: Box<Box<dyn FnOnce(NapiEnv) -> NapiResult<()>>> =
            Box::new(Box::new(finalizer));
        let tsfn = napi_call!(
            =napi_create_threadsafe_function,
            env,
            func.raw(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            0,
            Box::into_raw(finalizer) as _,
            Some(finalizer_trampoline),
            Box::into_raw(Box::new(context)) as _,
            None,
            // Some(call_js_trampoline::<C>),
        );
        Ok(NapiThreadsafeFunction(env, tsfn))
    }

    pub fn get_context(&self) -> NapiResult<DataPointer> {
        Ok(napi_call!(=napi_get_threadsafe_function_context, self.raw()))
    }

    /// This API should not be called with napi_tsfn_blocking from a JavaScript thread, because,
    /// if the queue is full, it may cause the JavaScript thread to deadlock.
    ///
    /// This API will return napi_closing if napi_release_threadsafe_function() was called with
    /// abort set to napi_tsfn_abort from any thread. The value is only added to the queue if
    /// the API returns napi_ok.
    ///
    /// This API may be called from any thread which makes use of func.
    pub fn call(&self, data: DataPointer, mode: NapiThreadsafeFunctionCallMode) -> NapiResult<()> {
        napi_call!(napi_call_threadsafe_function, self.raw(), data, mode);
        Ok(())
    }

    /// A thread should call this API before passing func to any other thread-safe function APIs
    /// to indicate that it will be making use of func. This prevents func from being destroyed
    /// when all other threads have stopped making use of it.
    ///
    /// This API may be called from any thread which will start making use of func.
    pub fn acquire(&self) -> NapiResult<()> {
        napi_call!(napi_acquire_threadsafe_function, self.raw());
        Ok(())
    }

    /// A thread should call this API when it stops making use of func. Passing func to any
    /// thread-safe APIs after having called this API has undefined results, as func may have
    /// been destroyed.
    ///
    /// This API may be called from any thread which will stop making use of func.
    pub fn release(self, mode: NapiThreadsafeFunctionReleaseMode) -> NapiResult<()> {
        napi_call!(napi_release_threadsafe_function, self.raw(), mode);
        Ok(())
    }

    /// This API is used to indicate that the event loop running on the main thread should not
    /// exit until func has been destroyed. Similar to uv_ref it is also idempotent.
    ///
    /// Neither does napi_unref_threadsafe_function mark the thread-safe functions as able to be
    /// destroyed nor does napi_ref_threadsafe_function prevent it from being destroyed.
    /// napi_acquire_threadsafe_function and napi_release_threadsafe_function are available for
    /// that purpose.
    ///
    /// This API may only be called from the main thread.
    pub fn refer(&self) -> NapiResult<()> {
        napi_call!(napi_ref_threadsafe_function, self.env(), self.raw());
        Ok(())
    }

    /// This API is used to indicate that the event loop running on the main thread may exit
    /// before func is destroyed. Similar to uv_unref it is also idempotent.
    ///
    /// This API may only be called from the main thread.
    pub fn unref(&self) -> NapiResult<()> {
        napi_call!(napi_unref_threadsafe_function, self.env(), self.raw());
        Ok(())
    }
}
