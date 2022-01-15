use crate::{api, prelude::*};
use std::{marker::PhantomData, mem::MaybeUninit, os::raw::c_void};

#[derive(Copy, Clone, Debug)]
pub struct JsExternal<'a, T>(pub(crate) JsValue<'a>, PhantomData<T>);

impl<'a, T> JsExternal<'a, T> {
    pub(crate) fn from_value(value: JsValue) -> JsExternal<T> {
        JsExternal(value, PhantomData)
    }

    /// create a string
    pub fn new(env: NapiEnv<'a>, value: T) -> NapiResult<JsExternal<'a, T>> {
        // NB: first leak value.
        let value = Box::into_raw(Box::new(value));

        unsafe extern "C" fn finalize<T>(_env: napi_env, data: *mut c_void, _hint: *mut c_void) {
            // NB: collect leaked value when the external value is being collected.
            Box::from_raw(data as *mut T);
        }

        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_create_external(
                env.raw(),
                value as *mut c_void,
                Some(finalize::<T>),
                std::ptr::null_mut(),
                result.as_mut_ptr(),
            );

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(JsExternal(JsValue::from_raw(env, value), PhantomData))
    }

    /// get the underlaying external value
    pub fn get(&self) -> NapiResult<&T> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status =
                api::napi_get_value_external(self.env().raw(), self.raw(), result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        let value = unsafe { &*(value as *const T) };

        Ok(value)
    }
}

impl<'a, T> ValueInner for JsExternal<'a, T> {
    fn downcast(&self) -> JsValue {
        self.0
    }
}
