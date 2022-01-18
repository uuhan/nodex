use crate::{api, prelude::*};
use std::{marker::PhantomData, mem::MaybeUninit};

#[derive(Clone, Debug)]
pub struct NapiAsyncWork(NapiEnv, napi_async_work);

impl NapiAsyncWork {
    pub(crate) fn from_value(env: NapiEnv, work: napi_async_work) -> NapiAsyncWork {
        NapiAsyncWork(env, work)
    }

    pub fn env(&self) -> NapiEnv {
        self.0
    }

    pub fn raw(&self) -> napi_async_work {
        self.1
    }

    /// This API allocates a work object that is used to execute logic asynchronously. It should be freed using napi_delete_async_work once the work is no longer required.
    /// async_resource_name should be a null-terminated, UTF-8-encoded string.
    /// The async_resource_name identifier is provided by the user and should be representative of the type of async work being performed. It is also recommended to apply namespacing to the identifier, e.g. by including the module name. See the async_hooks documentation for more information.
    pub fn new(env: NapiEnv, name: impl AsRef<str>) -> NapiResult<NapiAsyncWork> {
        let work = napi_call!(
            =napi_create_async_work,
            env.raw(),
            env.object()?.raw(),
            env.string(name)?.raw(),
            None,
            None,
            std::ptr::null_mut(),
        );

        Ok(NapiAsyncWork(env, work))
    }

    /// This API requests that the previously allocated work be scheduled for execution. Once it
    /// returns successfully, this API must not be called again with the same napi_async_work item
    /// or the result will be undefined.
    pub fn queue(&self) -> NapiResult<()> {
        Ok(napi_call!(
            napi_queue_async_work,
            self.env().raw(),
            self.raw(),
        ))
    }

    /// This API cancels queued work if it has not yet been started. If it has already
    /// started executing, it cannot be cancelled and napi_generic_failure will be returned.
    /// If successful, the complete callback will be invoked with a status value of
    /// napi_cancelled. The work should not be deleted before the complete callback invocation,
    /// even if it has been successfully cancelled.
    ///
    /// This API can be called even if there is a pending JavaScript exception.
    pub fn cancel(&self) -> NapiResult<()> {
        Ok(napi_call!(
            napi_cancel_async_work,
            self.env().raw(),
            self.raw(),
        ))
    }

    /// This API frees a previously allocated work object.
    /// This API can be called even if there is a pending JavaScript exception.
    pub fn delete(&self) -> NapiResult<()> {
        Ok(napi_call!(
            napi_delete_async_work,
            self.env().raw(),
            self.raw(),
        ))
    }
}
