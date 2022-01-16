use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsFunction<'a>(pub(crate) JsValue<'a>);

impl<'a> JsFunction<'a> {
    pub(crate) fn from_value(value: JsValue) -> JsFunction {
        JsFunction(value)
    }

    /// This API allows an add-on author to create a function object in native code.
    /// This is the primary mechanism to allow calling into the add-on's native code
    /// from JavaScript.
    /// The newly created function is not automatically visible from script after this call.
    /// Instead, a property must be explicitly set on any object that is visible to JavaScript,
    /// in order for the function to be accessible from script.
    /// In order to expose a function as part of the add-on's module exports, set the newly
    /// created function on the exports object. A sample module might look as follows:
    ///
    /// ```c
    /// napi_value SayHello(napi_env env, napi_callback_info info) {
    ///  printf("Hello\n");
    ///  return NULL;
    /// }
    ///
    /// napi_value Init(napi_env env, napi_value exports) {
    ///  napi_status status;
    ///  napi_value fn;
    ///  status = napi_create_function(env, NULL, 0, SayHello, NULL, &fn);
    ///  if (status != napi_ok) return NULL;
    ///  status = napi_set_named_property(env, exports, "sayHello", fn);
    ///  if (status != napi_ok) return NULL;
    ///  return exports;
    /// }
    ///
    /// NAPI_MODULE(NODE_GYP_MODULE_NAME, Init)
    /// ```
    ///
    /// Given the above code, the add-on can be used from JavaScript as follows:
    ///
    /// ```c
    /// const myaddon = require('./addon');
    /// myaddon.sayHello();
    /// ```
    ///
    /// The string passed to require() is the name of the target in binding.gyp responsible
    /// for creating the .node file.
    ///
    /// Any non-NULL data which is passed to this API via the data parameter can be associated
    /// with the resulting JavaScript function (which is returned in the result parameter)
    /// and freed whenever the function is garbage-collected by passing both the JavaScript
    /// function and the data to napi_add_finalizer.
    ///
    /// JavaScript Functions are described in Section 19.2 of the ECMAScript Language Specification.
    pub fn new(
        env: NapiEnv<'a>,
        name: impl AsRef<str>,
        value: unsafe extern "C" fn(env: napi_env, info: napi_callback_info) -> napi_value,
    ) -> NapiResult<JsFunction<'a>> {
        let value = napi_call!(
            =napi_create_function,
            env.raw(),
            name.as_ref().as_ptr() as *const c_char,
            name.as_ref().len(),
            Some(value),
            std::ptr::null_mut(),
        );

        Ok(JsFunction(JsValue::from_raw(env, value)))
    }

    /// Create a JsFunction by passing a rust closure.
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

        let value = napi_call!(
            =napi_create_function,
            env.raw(),
            name.as_ref().as_ptr() as *const c_char,
            name.as_ref().len(),
            Some(trampoline),
            // pass closure to trampoline function
            Box::into_raw(func) as _,
        );

        Ok(JsFunction(JsValue::from_raw(env, value)))
    }
}

impl<'a> NapiValueT for JsFunction<'a> {
    fn inner(&self) -> JsValue {
        self.0
    }
}
