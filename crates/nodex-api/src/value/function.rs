use crate::{api, prelude::*};
use std::{marker::PhantomData, mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct Function<F: NapiValueT>(pub(crate) JsValue, PhantomData<F>);

impl<F: NapiValueT> Function<F> {
    pub(crate) fn from_value(value: JsValue) -> Function<F> {
        Function::<F>(value, PhantomData)
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
    #[allow(clippy::type_complexity)]
    pub fn new<T, R>(
        env: NapiEnv,
        name: Option<impl AsRef<str>>,
        func: impl FnMut(JsObject, T) -> NapiResult<R>,
    ) -> NapiResult<Function<R>>
    where
        T: FromJsArgs,
        R: NapiValueT,
    {
        let (name, len) = if let Some(name) = name {
            (name.as_ref().as_ptr() as *const c_char, name.as_ref().len())
        } else {
            (std::ptr::null(), 0)
        };

        // NB: leak the func closure
        let func: Box<Box<dyn FnMut(JsObject, T) -> NapiResult<R>>> = Box::new(Box::new(func));

        extern "C" fn trampoline<T: FromJsArgs, R: NapiValueT>(
            env: NapiEnv,
            info: napi_callback_info,
        ) -> napi_value {
            let mut data = MaybeUninit::uninit();
            let mut this = MaybeUninit::uninit();

            let (argc, argv, this, mut func) = unsafe {
                // NB: use this to get the number of arguments
                // let mut argc = 0;
                // let mut argv = [std::ptr::null_mut(); 0];
                // api::napi_get_cb_info(
                //     env,
                //     info,
                //     &mut argc,
                //     argv.as_mut_ptr(),
                //     std::ptr::null_mut(),
                //     std::ptr::null_mut(),
                // );

                let mut argc = T::len();
                let mut argv = vec![std::ptr::null_mut(); T::len()];
                api::napi_get_cb_info(
                    env,
                    info,
                    &mut argc,
                    argv.as_mut_ptr(),
                    this.as_mut_ptr(),
                    data.as_mut_ptr(),
                );

                // NB: the Function maybe called multiple times, so we can should leak the
                // closure memory here.
                //
                // With napi >= 5, we can add a finalizer to this function.
                let func: &mut Box<dyn FnMut(JsObject, T) -> NapiResult<R>> =
                    std::mem::transmute(data);

                (argc, argv, this.assume_init(), func)
            };

            let args = argv
                .into_iter()
                .map(|arg| JsValue::from_raw(env, arg))
                .collect();
            let this = JsObject::from_raw(env, this);

            if let Ok(args) = T::from_js_args(JsArgs(args)) {
                napi_r!(env, =func(this, args))
            } else {
                env.throw_error("wrong argument type!").unwrap();
                env.undefined().unwrap().raw()
            }
        }

        let fn_pointer = Box::into_raw(func) as DataPointer;
        let value = napi_call!(
            =napi_create_function,
            env,
            name,
            len,
            Some(trampoline::<T, R>),
            // pass closure to trampoline function
            fn_pointer,
        );

        let mut func = Function::<R>(JsValue::from_raw(env, value), PhantomData);
        func.gc(move |_| unsafe {
            // NB: the leaked data is collected here.
            let _: Box<Box<dyn FnMut(JsObject, T) -> NapiResult<R>>> =
                Box::from_raw(fn_pointer as _);
            Ok(())
        })?;

        Ok(func)
    }

    /// This method allows a JavaScript function object to be called from a native add-on. This is
    /// the primary mechanism of calling back from the add-on's native code into JavaScript. For
    /// the special case of calling into JavaScript after an async operation, see
    /// napi_make_callback.
    pub fn call<T>(&self, this: JsObject, args: T) -> NapiResult<F>
    where
        T: ToJsArgs,
    {
        let args = args
            .to_js_args(this.env())?
            .0
            .into_iter()
            .map(|value| value.raw())
            .collect::<Vec<_>>();

        let value = napi_call!(
            =napi_call_function,
            self.env(),
            this.raw(),
            self.raw(),
            T::len(),
            args.as_ptr(),
        );

        Ok(F::from_raw(self.env(), value))
    }

    /// This method is used to instantiate a new JavaScript value using a given napi_value
    /// that represents the constructor for the object.
    pub fn new_instance<T, Args>(&self, args: Args) -> NapiResult<JsObject>
    where
        T: NapiValueT,
        Args: AsRef<[T]>,
    {
        let instance = napi_call!(
            =napi_new_instance,
            self.env(),
            self.raw(),
            args.as_ref().len(),
            args.as_ref().as_ptr() as _,
        );
        Ok(JsObject::from_raw(self.env(), instance))
    }
}

impl<F: NapiValueT> NapiValueT for Function<F> {
    fn from_raw(env: NapiEnv, raw: napi_value) -> Function<F> {
        Function::<F>(JsValue(env, raw), PhantomData)
    }

    fn value(&self) -> JsValue {
        self.0
    }
}

pub type JsFunction = Function<JsValue>;

impl<F: NapiValueT> NapiValueCheck for Function<F> {
    fn check(&self) -> NapiResult<bool> {
        Ok(self.kind()? == NapiValuetype::Function)
    }
}
