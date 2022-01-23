use crate::prelude::*;
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub struct NapiThreadsafeFunction<Data, const N: usize>(
    NapiEnv,
    napi_threadsafe_function,
    PhantomData<Data>,
);

unsafe impl<Data, const N: usize> Send for NapiThreadsafeFunction<Data, N> {}
unsafe impl<Data, const N: usize> Sync for NapiThreadsafeFunction<Data, N> {}

impl<Data, const N: usize> NapiThreadsafeFunction<Data, N> {
    pub(crate) fn from_raw(env: NapiEnv, tsfn: napi_threadsafe_function) -> Self {
        NapiThreadsafeFunction(env, tsfn, PhantomData)
    }

    pub fn env(&self) -> NapiEnv {
        self.0
    }

    pub fn raw(&self) -> napi_threadsafe_function {
        self.1
    }

    #[allow(clippy::type_complexity)]
    /// Create a napi_threadsafe_function
    ///
    /// R: the returned value of function.
    /// N: the maximum size of the queue, 0 for no limit.
    pub fn new<R>(
        env: NapiEnv,
        name: impl AsRef<str>,
        func: Function<R>,
        finalizer: impl FnOnce(NapiEnv) -> NapiResult<()>,
        callback: impl FnMut(Function<R>, Data) -> NapiResult<()>,
    ) -> NapiResult<NapiThreadsafeFunction<Data, N>>
    where
        R: NapiValueT,
    {
        unsafe extern "C" fn finalizer_trampoline(
            env: NapiEnv,
            finalizer: DataPointer,
            _: DataPointer,
        ) {
            let finalizer: Box<Box<dyn FnOnce(NapiEnv) -> NapiResult<()>>> =
                Box::from_raw(finalizer as _);

            if let Err(err) = finalizer(env) {
                log::error!("NapiThreadsafeFunction::finalizer(): {}", err);
            }
        }

        unsafe extern "C" fn call_js_trampoline<R, Data>(
            env: NapiEnv,
            cb: napi_value,
            context: DataPointer,
            data: DataPointer,
        ) {
            let context: &mut Box<dyn FnMut(Function<R>, Data) -> NapiResult<()>> =
                std::mem::transmute(&mut *(context as *mut _));
            let data: Box<Data> = Box::from_raw(data as _);

            if let Err(e) = context(Function::<R>::from_raw(env, cb), *data) {
                log::error!("NapiThreadsafeFunction::call_js_trampoline: {}", e);
            }
        }

        let context: Box<Box<dyn FnMut(Function<R>, Data) -> NapiResult<()>>> =
            Box::new(Box::new(callback));
        // NB: leak here
        let context = Box::into_raw(context);
        let finalizer: Box<Box<dyn FnOnce(NapiEnv) -> NapiResult<()>>> =
            Box::new(Box::new(move |env| -> NapiResult<()> {
                unsafe {
                    Box::from_raw(context);
                }
                finalizer(env)
            }));

        let tsfn = napi_call!(
            =napi_create_threadsafe_function,
            env,
            func.raw(),
            std::ptr::null_mut(),
            env.string(name.as_ref())?.raw(),
            0,
            N,
            Box::into_raw(finalizer) as _,
            Some(finalizer_trampoline),
            context as _,
            Some(call_js_trampoline::<R, Data>),
        );

        Ok(NapiThreadsafeFunction(env, tsfn, PhantomData))
    }

    // pub fn context<C>(&self) -> NapiResult<&mut C> {
    //     let context = napi_call!(=napi_get_threadsafe_function_context, self.raw());
    //     unsafe { Ok(std::mem::transmute(context)) }
    // }

    /// This API should not be called with napi_tsfn_blocking from a JavaScript thread, because,
    /// if the queue is full, it may cause the JavaScript thread to deadlock.
    ///
    /// This API will return napi_closing if napi_release_threadsafe_function() was called with
    /// abort set to napi_tsfn_abort from any thread. The value is only added to the queue if
    /// the API returns napi_ok.
    ///
    /// This API may be called from any thread which makes use of func.
    pub fn call(&self, data: Data, mode: NapiThreadsafeFunctionCallMode) -> NapiResult<()> {
        napi_call!(
            napi_call_threadsafe_function,
            self.raw(),
            Box::into_raw(Box::new(data)) as _,
            mode,
        );
        Ok(())
    }

    #[inline]
    pub fn blocking(&self, data: Data) -> NapiResult<()> {
        self.call(data, NapiThreadsafeFunctionCallMode::Blocking)
    }

    #[inline]
    pub fn non_blocking(&self, data: Data) -> NapiResult<()> {
        self.call(data, NapiThreadsafeFunctionCallMode::Nonblocking)
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
    pub fn release(self) -> NapiResult<()> {
        napi_call!(
            napi_release_threadsafe_function,
            self.raw(),
            NapiTsfnReleaseMode::Release
        );
        Ok(())
    }

    /// A thread should call this API when it stops making use of func. Passing func to any
    /// thread-safe APIs after having called this API has undefined results, as func may have
    /// been destroyed.
    ///
    /// This API may be called from any thread which will stop making use of func.
    pub fn abort(self) -> NapiResult<()> {
        napi_call!(
            napi_release_threadsafe_function,
            self.raw(),
            NapiTsfnReleaseMode::Abort
        );
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

pub type NapiTsfn<Data, const N: usize> = NapiThreadsafeFunction<Data, N>;
