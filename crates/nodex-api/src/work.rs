use crate::{api, prelude::*};
use std::{marker::PhantomData, mem::MaybeUninit};

#[derive(Clone, Debug)]
pub struct NapiAsyncWork<'a> {
    env: NapiEnv<'a>,
    work: napi_async_work,
}

impl<'a> NapiAsyncWork<'a> {
    pub(crate) fn from_value(env: NapiEnv<'a>, work: napi_async_work) -> NapiAsyncWork {
        NapiAsyncWork { env, work }
    }

    pub fn env(&self) -> NapiEnv<'a> {
        self.env
    }

    pub fn raw(&self) -> napi_async_work {
        self.work
    }

    /// This API allocates a work object that is used to execute logic asynchronously. It should be freed using napi_delete_async_work once the work is no longer required.
    /// async_resource_name should be a null-terminated, UTF-8-encoded string.
    /// The async_resource_name identifier is provided by the user and should be representative of the type of async work being performed. It is also recommended to apply namespacing to the identifier, e.g. by including the module name. See the async_hooks documentation for more information.
    pub fn new(env: NapiEnv<'a>, name: impl AsRef<str>) -> NapiResult<NapiAsyncWork<'a>> {
        // TODO: nodejs async work
        let work = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_create_async_work(
                env.raw(),
                env.object()?.raw(),
                env.string(name)?.raw(),
                None,
                None,
                std::ptr::null_mut(),
                result.as_mut_ptr(),
            );

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(NapiAsyncWork { env, work })
    }
}
