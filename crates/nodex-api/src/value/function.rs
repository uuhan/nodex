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
        value: extern "C" fn(env: NapiEnv, info: napi_callback_info) -> napi_value,
    ) -> NapiResult<JsFunction> {
        let (name, len) = if let Some(name) = name {
            (name.as_ref().as_ptr() as *const c_char, name.as_ref().len())
        } else {
            (std::ptr::null(), 0)
        };

        let value = napi_call!(
            =napi_create_function,
            env,
            name,
            len,
            Some(value),
            std::ptr::null_mut(),
        );

        Ok(JsFunction(JsValue::from_raw(env, value)))
    }

    /// Create a js function with rust closure
    #[allow(clippy::type_complexity)]
    pub fn with<Func, T, R, const N: usize>(
        env: NapiEnv,
        name: Option<impl AsRef<str>>,
        func: Func,
    ) -> NapiResult<JsFunction>
    where
        T: NapiValueT,
        R: NapiValueT,
        Func: FnMut(JsObject, [T; N]) -> NapiResult<R>,
    {
        let (name, len) = if let Some(name) = name {
            (name.as_ref().as_ptr() as *const c_char, name.as_ref().len())
        } else {
            (std::ptr::null(), 0)
        };

        // NB: leak the func closure
        let func: Box<Box<dyn FnMut(JsObject, [T; N]) -> NapiResult<R>>> = Box::new(Box::new(func));

        // TODO: it just works but not very useful by current design
        // use the trampoline function to call into the closure
        extern "C" fn trampoline<T: NapiValueT, R: NapiValueT, const N: usize>(
            env: NapiEnv,
            info: napi_callback_info,
        ) -> napi_value {
            let mut argc = N;
            let mut argv = [std::ptr::null_mut(); N];
            let mut data = MaybeUninit::uninit();
            let mut this = MaybeUninit::uninit();

            let (argc, argv, this, mut func) = unsafe {
                let status = api::napi_get_cb_info(
                    env,
                    info,
                    &mut argc,
                    argv.as_mut_ptr(),
                    this.as_mut_ptr(),
                    data.as_mut_ptr(),
                );

                // NB: the JsFunction maybe called multiple times, so we can should leak the
                // closure memory here.
                //
                // With napi >= 5, we can add a finalizer to this function.
                let func: &mut Box<dyn FnMut(JsObject, [T; N]) -> NapiResult<R>> =
                    std::mem::transmute(data);

                (argc, argv, this.assume_init(), func)
            };

            let args = unsafe { argv.map(|arg| T::from_raw(env, arg)) };
            let this = JsObject::from_raw(env, this);

            if let Ok(result) = func(this, args) {
                result.raw()
            } else {
                env.undefined().unwrap().raw()
            }
        }

        let fn_pointer = Box::into_raw(func) as DataPointer;
        let fn_pointer_1 = fn_pointer.clone();

        let value = napi_call!(
            =napi_create_function,
            env,
            name,
            len,
            Some(trampoline::<T, R, N>),
            // pass closure to trampoline function
            fn_pointer,
        );

        let func = JsFunction(JsValue::from_raw(env, value));

        #[cfg(feature = "v5")]
        func.finalizer(move |_| unsafe {
            // NB: the leaked data is collected here.
            let _: Box<Box<dyn FnMut(JsObject, [T; N]) -> NapiResult<R>>> =
                Box::from_raw(fn_pointer as _);
            Ok(())
        })?;

        Ok(func)
    }

    /// This method allows a JavaScript function object to be called from a native add-on. This is
    /// the primary mechanism of calling back from the add-on's native code into JavaScript. For
    /// the special case of calling into JavaScript after an async operation, see
    /// napi_make_callback.
    pub fn call<T, const N: usize>(&self, this: JsObject, argv: [T; N]) -> NapiResult<JsValue>
    where
        T: NapiValueT,
    {
        let value = napi_call!(
            =napi_call_function,
            self.env(),
            this.raw(),
            self.raw(),
            argv.len(),
            argv.map(|arg| arg.raw()).as_ptr(),
        );
        Ok(JsValue::from_raw(self.env(), value))
    }
}

napi_value_t!(JsFunction);
