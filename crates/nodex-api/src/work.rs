use crate::{api, prelude::*};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct NapiAsyncWork<T>(NapiEnv, napi_async_work, bool, PhantomData<T>);

impl<T> NapiAsyncWork<T> {
    pub(crate) fn from_raw(env: NapiEnv, work: napi_async_work) -> NapiAsyncWork<T> {
        NapiAsyncWork(env, work, false, PhantomData)
    }

    pub fn env(&self) -> NapiEnv {
        self.0
    }

    pub fn raw(&self) -> napi_async_work {
        self.1
    }

    /// This API allocates a work object that is used to execute logic asynchronously.
    /// It should be freed using napi_delete_async_work once the work is no longer required.
    /// async_resource_name should be a null-terminated, UTF-8-encoded string.
    ///
    /// The async_resource_name identifier is provided by the user and should be representative
    /// of the type of async work being performed. It is also recommended to apply namespacing
    /// to the identifier, e.g. by including the module name. See the async_hooks documentation
    /// for more information.
    ///
    /// # Arguments
    ///
    /// * `env` - napi_env
    /// * `name` - napi async work identifier
    /// * `state` - The state shared between `execute` & `complete`
    /// * `execute` - The native function which should be called to execute the logic asynchronously. The given function is called from a worker pool thread and can execute in parallel with the main event loop thread.
    /// * `complete` - The native function which will be called when the asynchronous logic is completed or is cancelled. The given function is called from the main event loop thread.
    #[allow(clippy::type_complexity)]
    pub fn new(
        env: NapiEnv,
        name: impl AsRef<str>,
        state: T,
        execute: impl FnMut(&mut T),
        complete: impl FnMut(NapiEnv, NapiStatus, T) -> NapiResult<()>,
    ) -> NapiResult<NapiAsyncWork<T>> {
        extern "C" fn napi_async_execute_callback<T>(env: NapiEnv, data: DataPointer) {
            unsafe {
                // NB: We just access the execute function here, so read complete function as
                // `dyn FnMut<&mut T>`. It only runs once.
                let (execute, _, state): &mut (
                    Box<dyn FnMut(&mut T)>,
                    Box<dyn FnMut(NapiEnv, NapiStatus, T) -> NapiResult<()>>,
                    T,
                ) = std::mem::transmute(&mut *(data as *mut _));
                execute(state);
            }
        }
        extern "C" fn napi_async_complete_callback<T>(
            env: NapiEnv,
            status: NapiStatus,
            data: DataPointer,
        ) {
            unsafe {
                let mut pair: Box<(
                    Box<dyn FnMut(&mut T)>,
                    Box<dyn FnMut(NapiEnv, NapiStatus, T)>,
                    T,
                )> = Box::from_raw(data as _);
                let mut complete = pair.1;
                complete(env, status, pair.2);
            }
        }

        let pair: Box<(
            Box<dyn FnMut(&mut T)>,
            Box<dyn FnMut(NapiEnv, NapiStatus, T) -> NapiResult<()>>,
            T,
        )> = Box::new((Box::new(execute), Box::new(complete), state));

        let work = napi_call!(
            =napi_create_async_work,
            env,
            env.object()?.raw(),
            env.string(name)?.raw(),
            Some(napi_async_execute_callback::<T>),
            Some(napi_async_complete_callback::<T>),
            Box::into_raw(pair) as _,
        );

        Ok(NapiAsyncWork::from_raw(env, work))
    }

    /// This API requests that the previously allocated work be scheduled for execution. Once it
    /// returns successfully, this API must not be called again with the same napi_async_work item
    /// or the result will be undefined.
    ///
    /// NB: The `NapiAsyncWork` can not be queued more than once.
    pub fn queue(&mut self) -> NapiResult<Option<()>> {
        if self.2 {
            Ok(None)
        } else {
            napi_call!(napi_queue_async_work, self.env(), self.raw());
            self.2 = true;
            Ok(Some(()))
        }
    }

    /// This API cancels queued work if it has not yet been started. If it has already
    /// started executing, it cannot be cancelled and napi_generic_failure will be returned.
    /// If successful, the complete callback will be invoked with a status value of
    /// napi_cancelled. The work should not be deleted before the complete callback invocation,
    /// even if it has been successfully cancelled.
    ///
    /// This API can be called even if there is a pending JavaScript exception.
    pub fn cancel(&self) -> NapiResult<()> {
        napi_call!(napi_cancel_async_work, self.env(), self.raw())
    }

    /// This API frees a previously allocated work object.
    /// This API can be called even if there is a pending JavaScript exception.
    ///
    /// NB: should not delete a queued task.
    pub fn delete(self) -> NapiResult<()> {
        if !self.2 {
            napi_call!(napi_delete_async_work, self.env(), self.raw());
        }
        Ok(())
    }
}
