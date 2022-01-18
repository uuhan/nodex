use crate::{api, prelude::*};

type DataPointer = *mut std::ffi::c_void;

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
    /// * `execute` - The native function which should be called to execute the logic asynchronously. The given function is called from a worker pool thread and can execute in parallel with the main event loop thread.
    /// * `complete` - The native function which will be called when the asynchronous logic is completed or is cancelled. The given function is called from the main event loop thread.
    pub fn new(
        env: NapiEnv,
        name: impl AsRef<str>,
        execute: impl Fn(NapiEnv),
        complete: impl Fn(NapiEnv, NapiStatus),
    ) -> NapiResult<NapiAsyncWork> {
        extern "C" fn napi_async_execute_callback(env: napi_env, data: DataPointer) {
            unsafe {
                let env = NapiEnv::from_raw(env);
                let (execute, _): &mut (Box<dyn Fn(NapiEnv)>, Box<dyn Fn(NapiEnv, NapiStatus)>) =
                    std::mem::transmute(data);
                execute(env);
            }
        }
        extern "C" fn napi_async_complete_callback(
            env: napi_env,
            status: NapiStatus,
            data: DataPointer,
        ) {
            unsafe {
                let env = NapiEnv::from_raw(env);
                let pair: Box<(Box<dyn Fn(NapiEnv)>, Box<dyn Fn(NapiEnv, NapiStatus)>)> =
                    Box::from_raw(data as _);
                let mut complete = pair.1;
                complete(env, status);
            }
        }

        let pair: Box<(Box<dyn Fn(NapiEnv)>, Box<dyn Fn(NapiEnv, NapiStatus)>)> =
            Box::new((Box::new(execute), Box::new(complete)));

        let work = napi_call!(
            =napi_create_async_work,
            env.raw(),
            env.object()?.raw(),
            env.string(name)?.raw(),
            Some(napi_async_execute_callback),
            Some(napi_async_complete_callback),
            Box::into_raw(pair) as _,
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
