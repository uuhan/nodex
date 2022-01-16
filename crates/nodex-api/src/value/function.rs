use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsFunction<'a>(pub(crate) JsValue<'a>);

impl<'a> JsFunction<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsFunction {
        JsFunction(value)
    }

    /// create a function
    pub fn new(
        env: NapiEnv<'a>,
        name: impl AsRef<str>,
        value: unsafe extern "C" fn(env: napi_env, info: napi_callback_info) -> napi_value,
    ) -> NapiResult<JsFunction<'a>> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_create_function(
                env.raw(),
                name.as_ref().as_ptr() as *const c_char,
                name.as_ref().len(),
                Some(value),
                std::ptr::null_mut(),
                result.as_mut_ptr(),
            );

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(JsFunction(JsValue::from_raw(env, value)))
    }

    /// create a function with rust-closure
    pub fn with(
        env: NapiEnv<'a>,
        name: impl AsRef<str>,
        func: impl FnMut(),
    ) -> NapiResult<JsFunction<'a>> {
        // NB: leak the func closure
        let func: Box<Box<dyn FnMut()>> = Box::new(Box::new(func));

        // TODO: it just works but not very useful by current design
        // use the trampoline function to call into the closure
        extern "C" fn trampoline(env: napi_env, info: napi_callback_info) -> napi_value {
            let mut argc = MaybeUninit::zeroed();
            let mut argv = MaybeUninit::uninit();
            let mut data = MaybeUninit::uninit();
            let mut this = MaybeUninit::uninit();

            let (argc, argv, this, mut func) = unsafe {
                let status = api::napi_get_cb_info(
                    env,
                    info,
                    argc.as_mut_ptr(),
                    argv.as_mut_ptr(),
                    this.as_mut_ptr(),
                    data.as_mut_ptr(),
                );

                // NB: this cb is leaked, should collect the box when the function is destroyed
                // restore the closure from data
                let func: &mut Box<dyn FnMut()> = std::mem::transmute(data);

                (
                    argc.assume_init(),
                    argv.assume_init(),
                    this.assume_init(),
                    func,
                )
            };

            // call the closure
            func();

            this
        }

        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_create_function(
                env.raw(),
                name.as_ref().as_ptr() as *const c_char,
                name.as_ref().len(),
                Some(trampoline),
                // pass closure to trampoline function
                Box::into_raw(func) as _,
                result.as_mut_ptr(),
            );

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(JsFunction(JsValue::from_raw(env, value)))
    }
}

impl<'a> NapiValueT for JsFunction<'a> {
    fn inner(&self) -> JsValue {
        self.0
    }
}
