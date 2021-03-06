use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsClass(pub(crate) JsValue);

impl JsClass {
    pub(crate) fn from_value(value: JsValue) -> JsClass {
        JsClass(value)
    }

    /// Defines a JavaScript class, including:
    ///
    /// * A JavaScript constructor function that has the class name. When wrapping a corresponding C++ class, the callback passed via constructor can be used to instantiate a new C++ class instance, which can then be placed inside the JavaScript object instance being constructed using napi_wrap.
    /// * Properties on the constructor function whose implementation can call corresponding static data properties, accessors, and methods of the C++ class (defined by property descriptors with the napi_static attribute).
    /// * Properties on the constructor function's prototype object. When wrapping a C++ class, non-static data properties, accessors, and methods of the C++ class can be called from the static functions given in the property descriptors without the napi_static attribute after retrieving the C++ class instance placed inside the JavaScript object instance by using napi_unwrap.
    ///
    /// When wrapping a C++ class, the C++ constructor callback passed via constructor should
    /// be a static method on the class that calls the actual class constructor, then wraps the
    /// new C++ instance in a JavaScript object, and returns the wrapper object. See napi_wrap
    /// for details.
    ///
    /// The JavaScript constructor function returned from napi_define_class is often saved and
    /// used later to construct new instances of the class from native code, and/or to check
    /// whether provided values are instances of the class. In that case, to prevent the function
    /// value from being garbage-collected, a strong persistent reference to it can be created
    /// using napi_create_reference, ensuring that the reference count is kept >= 1.
    ///
    /// Any non-NULL data which is passed to this API via the data parameter or via the data field
    /// of the napi_property_descriptor array items can be associated with the resulting JavaScript
    /// constructor (which is returned in the result parameter) and freed whenever the class is
    /// garbage-collected by passing both the JavaScript function and the data to napi_add_finalizer.
    #[allow(clippy::type_complexity)]
    pub fn new<F, P, T, R>(
        env: NapiEnv,
        name: impl AsRef<str>,
        func: F,
        properties: P,
    ) -> NapiResult<JsClass>
    where
        T: FromJsArgs,
        R: NapiValueT,
        F: FnMut(JsObject, T) -> NapiResult<R>,
        P: AsRef<[NapiPropertyDescriptor]>,
    {
        // NB: leak the func closure
        let func: Box<Box<dyn FnMut(JsObject, T) -> NapiResult<R>>> = Box::new(Box::new(func));

        // TODO: it just works but not very useful by current design
        // use the trampoline function to call into the closure
        extern "C" fn trampoline<T: FromJsArgs, R: NapiValueT>(
            env: NapiEnv,
            info: napi_callback_info,
        ) -> napi_value {
            let mut data = MaybeUninit::uninit();
            let mut this = MaybeUninit::uninit();

            let (argc, argv, this, mut func) = unsafe {
                let mut argc = T::len();
                let mut argv = vec![std::ptr::null_mut(); T::len()];

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
                env.throw_error("wrong argument type!");
                env.undefined().unwrap().raw()
            }
        }

        let fn_pointer = Box::into_raw(func) as DataPointer;
        let value = napi_call!(
            =napi_define_class,
            env,
            name.as_ref().as_ptr() as CharPointer,
            name.as_ref().len(),
            Some(trampoline::<T, R>),
            fn_pointer,
            properties.as_ref().len(),
            properties.as_ref().as_ptr() as *const _,
        );

        let mut class = JsClass(JsValue(env, value));

        class.gc(move |_| unsafe {
            // NB: the leaked data is collected here.
            let _: Box<Box<dyn FnMut(JsObject, T) -> NapiResult<R>>> =
                Box::from_raw(fn_pointer as _);
            Ok(())
        })?;

        Ok(class)
    }

    /// This method is used to instantiate a new JavaScript value using a given napi_value
    /// that represents the constructor for the object.
    pub fn new_instance<T>(&self, args: T) -> NapiResult<JsObject>
    where
        T: ToJsArgs,
    {
        let args = args
            .to_js_args(self.env())?
            .0
            .into_iter()
            .map(|value| value.raw())
            .collect::<Vec<_>>();

        let instance = napi_call!(
            =napi_new_instance,
            self.env(),
            self.raw(),
            T::len(),
            args.as_ptr(),
        );
        Ok(JsObject::from_raw(self.env(), instance))
    }
}

napi_value_t!(JsClass);

impl NapiValueCheck for JsClass {
    fn check(&self) -> NapiResult<bool> {
        Ok(self.kind()? == NapiValuetype::Function)
    }
}
