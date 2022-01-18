use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsFunction(pub(crate) JsValue);

impl JsFunction {
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
        env: NapiEnv,
        name: Option<impl AsRef<str>>,
        value: extern "C" fn(env: napi_env, info: napi_callback_info) -> napi_value,
    ) -> NapiResult<JsFunction> {
        let (name, len) = if let Some(name) = name {
            (name.as_ref().as_ptr() as *const c_char, name.as_ref().len())
        } else {
            (std::ptr::null(), 0)
        };

        let value = napi_call!(
            =napi_create_function,
            env.raw(),
            name,
            len,
            Some(value),
            std::ptr::null_mut(),
        );

        Ok(JsFunction(JsValue::from_raw(env, value)))
    }

    /// Create a js function with rust closure
    pub fn with<Func, const N: usize>(
        env: NapiEnv,
        name: Option<impl AsRef<str>>,
        func: Func,
    ) -> NapiResult<JsFunction>
    where
        Func: FnMut(JsObject, [JsValue; N]) -> NapiResult<JsValue>,
    {
        let (name, len) = if let Some(name) = name {
            (name.as_ref().as_ptr() as *const c_char, name.as_ref().len())
        } else {
            (std::ptr::null(), 0)
        };

        // NB: leak the func closure
        let func: Box<Box<dyn FnMut(JsObject, [JsValue; N]) -> NapiResult<JsValue>>> =
            Box::new(Box::new(func));

        // TODO: it just works but not very useful by current design
        // use the trampoline function to call into the closure
        extern "C" fn trampoline<const N: usize>(
            env: napi_env,
            info: napi_callback_info,
        ) -> napi_value {
            let mut argc = N;
            let mut argv = [std::ptr::null_mut(); N];
            let mut data = MaybeUninit::uninit();
            let mut this = MaybeUninit::uninit();

            let env = NapiEnv::from_raw(env);

            let (argc, argv, this, mut func) = unsafe {
                let status = api::napi_get_cb_info(
                    env.raw(),
                    info,
                    &mut argc,
                    argv.as_mut_ptr(),
                    this.as_mut_ptr(),
                    data.as_mut_ptr(),
                );

                // NB: this cb is leaked, should collect the box when the function is destroyed
                // restore the closure from data
                let func: &mut Box<dyn FnMut(JsObject, [JsValue; N]) -> NapiResult<JsValue>> =
                    std::mem::transmute(data);

                (argc, argv, this.assume_init(), func)
            };

            let args = unsafe { argv.map(|arg| JsValue::from_raw(env, arg)) };

            let this = JsObject::from_value(JsValue::from_raw(env, this));

            let result = if let Ok(result) = func(this, args) {
                result
            } else {
                env.undefined().unwrap().value()
            };

            result.raw()
        }

        let value = napi_call!(
            =napi_create_function,
            env.raw(),
            name,
            len,
            Some(trampoline::<N>),
            // pass closure to trampoline function
            Box::into_raw(func) as _,
        );

        Ok(JsFunction(JsValue::from_raw(env, value)))
    }
}

napi_value_t!(JsFunction);
