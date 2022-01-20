use std::mem::MaybeUninit;

use crate::{
    api::{self, napi_node_version},
    prelude::*,
};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NapiEnv(pub(crate) napi_env);

impl AsRef<napi_env> for NapiEnv {
    fn as_ref(&self) -> &napi_env {
        &self.0
    }
}

impl NapiEnv {
    /// create `NapiEnv` from raw napi_env
    pub fn from_raw(env: napi_env) -> NapiEnv {
        NapiEnv(env)
    }

    /// access raw napi_env from `NapiEnv`
    pub fn raw(&self) -> napi_env {
        self.0
    }

    /// This API returns the global object.
    pub fn global(&self) -> NapiResult<JsGlobal> {
        JsGlobal::new(*self)
    }

    /// get node version
    /// the returned buffer is statically allocated and does not need to be freed.
    pub fn node_version(&self) -> NapiResult<napi_node_version> {
        let value = napi_call!(=napi_get_node_version, *self);
        unsafe { Ok(std::ptr::read(value)) }
    }

    /// get napi version
    pub fn napi_version(&self) -> NapiResult<u32> {
        Ok(napi_call!(=napi_get_version, *self))
    }

    /// Return null object
    pub fn null(&self) -> NapiResult<JsNull> {
        JsNull::new(*self)
    }

    /// Return undefined object
    pub fn undefined(&self) -> NapiResult<JsUndefined> {
        JsUndefined::new(*self)
    }

    /// This API is used to convert from the C double type to the JavaScript number type.
    /// The JavaScript number type is described in Section 6.1.6 of the ECMAScript Language Specification.
    pub fn double(&self, value: f64) -> NapiResult<JsNumber> {
        JsNumber::double(*self, value)
    }

    /// This API creates a JavaScript string value from a UTF8-encoded C string. The native string is copied.
    /// The JavaScript string type is described in Section 6.1.4 of the ECMAScript Language Specification.
    pub fn string(&self, s: impl AsRef<str>) -> NapiResult<JsString> {
        JsString::new(*self, s)
    }

    /// This API creates a JavaScript symbol value from a UTF8-encoded C string.
    /// The JavaScript symbol type is described in Section 19.4 of the ECMAScript Language Specification.
    pub fn symbol(&self) -> NapiResult<JsSymbol> {
        JsSymbol::new(*self)
    }

    /// Symbol with description.
    pub fn symbol_description(&self, desc: JsString) -> NapiResult<JsSymbol> {
        JsSymbol::description(*self, desc)
    }

    /// This API allocates a default JavaScript Object. It is the equivalent of doing new Object() in JavaScript.
    /// The JavaScript Object type is described in Section 6.1.7 of the ECMAScript Language Specification.
    pub fn object(&self) -> NapiResult<JsObject> {
        JsObject::new(*self)
    }

    /// The async context
    pub fn context(&self, name: impl AsRef<str>) -> NapiResult<NapiAsyncContext> {
        NapiAsyncContext::new(*self, name)
    }

    /// Create a named js function with a rust closure.
    pub fn func_named<Func, T, R, const N: usize>(
        &self,
        name: impl AsRef<str>,
        func: Func,
    ) -> NapiResult<JsFunction>
    where
        T: NapiValueT,
        R: NapiValueT,
        Func: FnMut(JsObject, [T; N]) -> NapiResult<R>,
    {
        JsFunction::new(*self, Some(name), func)
    }

    // Create a js function with a rust closure.
    pub fn func<Func, T, R, const N: usize>(&self, func: Func) -> NapiResult<JsFunction>
    where
        T: NapiValueT,
        R: NapiValueT,
        Func: FnMut(JsObject, [T; N]) -> NapiResult<R>,
    {
        JsFunction::new(*self, Option::<String>::None, func)
    }

    /// Create a named js function with a rust function
    pub fn function_named(
        &self,
        name: impl AsRef<str>,
        func: extern "C" fn(env: NapiEnv, info: napi_callback_info) -> napi_value,
    ) -> NapiResult<JsFunction> {
        let value = napi_call!(
            =napi_create_function,
            *self,
            name.as_ref().as_ptr() as CharPointer,
            name.as_ref().len(),
            Some(func),
            std::ptr::null_mut(),
        );

        Ok(JsFunction(JsValue::from_raw(*self, value)))
    }

    /// Create a js function with a rust function
    pub fn function(
        &self,
        func: extern "C" fn(env: NapiEnv, info: napi_callback_info) -> napi_value,
    ) -> NapiResult<JsFunction> {
        let value = napi_call!(
            =napi_create_function,
            *self,
            std::ptr::null(),
            0,
            Some(func),
            std::ptr::null_mut(),
        );

        Ok(JsFunction(JsValue::from_raw(*self, value)))
    }

    /// Create a js class with a rust closure
    pub fn class<F, P, T, R, const N: usize>(
        &self,
        name: impl AsRef<str>,
        func: F,
        properties: P,
    ) -> NapiResult<JsClass>
    where
        T: NapiValueT,
        R: NapiValueT,
        F: FnMut(JsObject, [T; N]) -> NapiResult<R>,
        P: AsRef<[NapiPropertyDescriptor]>,
    {
        JsClass::new(*self, name, func, properties)
    }

    /// Create an async work
    pub fn async_work(
        &self,
        name: impl AsRef<str>,
        execute: impl FnMut(),
        complete: impl FnMut(NapiEnv, NapiStatus) -> NapiResult<()>,
    ) -> NapiResult<NapiAsyncWork> {
        NapiAsyncWork::new(*self, name, execute, complete)
    }

    /// Create an async work with shared state
    pub fn async_work_state<T>(
        &self,
        name: impl AsRef<str>,
        state: T,
        execute: impl FnMut(&mut T),
        complete: impl FnMut(NapiEnv, NapiStatus, &mut T) -> NapiResult<()>,
    ) -> NapiResult<NapiAsyncWork> {
        NapiAsyncWork::state(*self, name, state, execute, complete)
    }

    /// This method allows the efficient definition of multiple properties on a given object. The
    /// properties are defined using property descriptors (see napi_property_descriptor). Given an
    /// array of such property descriptors, this API will set the properties on the object one at a
    /// time, as defined by DefineOwnProperty() (described in Section 9.1.6 of the ECMA-262
    /// specification).
    pub fn define_properties(
        &self,
        object: impl NapiValueT,
        properties: impl AsRef<[NapiPropertyDescriptor]>,
    ) -> NapiResult<()> {
        napi_call!(
            napi_define_properties,
            *self,
            object.raw(),
            properties.as_ref().len(),
            properties.as_ref().as_ptr() as *const _,
        );
        Ok(())
    }

    /// This API throws a JavaScript Error with the text provided.
    #[inline]
    pub fn throw_error(
        &self,
        message: impl AsRef<str>,
        code: Option<impl AsRef<str>>,
    ) -> NapiResult<()> {
        use std::ffi::CString;
        let msg = CString::new(message.as_ref()).map_err(|_| NapiStatus::StringExpected)?;
        let code = if let Some(code) = code {
            CString::new(code.as_ref())
                .map_err(|_| NapiStatus::StringExpected)?
                .as_ptr()
        } else {
            std::ptr::null()
        };
        napi_call!(napi_throw_error, *self, code, msg.as_ptr());
        Ok(())
    }

    /// This API throws a JavaScript TypeError with the text provided.
    #[inline]
    pub fn throw_type_error(
        &self,
        message: impl AsRef<str>,
        code: Option<impl AsRef<str>>,
    ) -> NapiResult<()> {
        let msg = napi_s!(message.as_ref()).map_err(|_| NapiStatus::StringExpected)?;
        let code = if let Some(code) = code {
            napi_s!(code.as_ref())?.as_ptr()
        } else {
            std::ptr::null()
        };
        napi_call!(napi_throw_type_error, *self, code, msg.as_ptr());
        Ok(())
    }

    /// This API throws a JavaScript TypeError with the text provided.
    #[inline]
    pub fn throw_range_error(
        &self,
        message: impl AsRef<str>,
        code: Option<impl AsRef<str>>,
    ) -> NapiResult<()> {
        use std::ffi::CString;
        let msg = CString::new(message.as_ref()).map_err(|_| NapiStatus::StringExpected)?;
        let code = if let Some(code) = code {
            CString::new(code.as_ref())
                .map_err(|_| NapiStatus::StringExpected)?
                .as_ptr()
        } else {
            std::ptr::null()
        };
        napi_call!(napi_throw_range_error, *self, code, msg.as_ptr());
        Ok(())
    }

    #[inline]
    pub fn fatal_error(&self, msg: impl AsRef<str>) {
        crate::fatal_error(msg, Option::<String>::None);
    }

    /// Get and clear last exception
    /// This API can be called even if there is a pending JavaScript exception.
    #[inline]
    pub fn get_and_clear_last_exception(&self) -> NapiResult<Option<JsError>> {
        let err = napi_call!(=napi_get_and_clear_last_exception, *self);
        if err.is_null() {
            Ok(None)
        } else {
            Ok(Some(JsError(JsValue(*self, err))))
        }
    }

    /// Return true if an exception is pending.
    /// This API can be called even if there is a pending JavaScript exception.
    #[inline]
    pub fn is_exception_pending(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_exception_pending, *self))
    }

    /// Trigger an 'uncaughtException' in JavaScript. Useful if an async callback throws an
    /// exception with no way to recover.
    #[inline]
    #[cfg(feature = "v3")]
    pub fn fatal_exception(&self, err: JsError) -> NapiResult<()> {
        napi_call!(napi_fatal_exception, *self, err.raw());
        Ok(())
    }

    /// Create a handle scope
    pub fn handle_scope(&self) -> NapiResult<NapiHandleScope> {
        NapiHandleScope::open(*self)
    }

    /// Create a escapable handle scope
    pub fn escapable_handle_scope(&self) -> NapiResult<NapiEscapableHandleScope> {
        NapiEscapableHandleScope::open(*self)
    }

    /// Registers fun as a function to be run with the arg parameter once the current
    /// Node.js environment exits.
    ///
    /// A function can safely be specified multiple times with different arg values.
    ///
    /// In that case, it will be called multiple times as well. Providing the same fun
    /// and arg values multiple times is not allowed and will lead the process to abort.
    ///
    /// The hooks will be called in reverse order, i.e. the most recently added one
    /// will be called first.
    ///
    /// Removing this hook can be done by using napi_remove_env_cleanup_hook. Typically,
    /// that happens when the resource for which this hook was added is being torn down anyway.
    /// For asynchronous cleanup, napi_add_async_cleanup_hook is available.
    #[cfg(feature = "v3")]
    pub fn add_cleanup_hook<Hook>(&mut self, hook: Hook) -> NapiResult<CleanupHookHandler>
    where
        Hook: FnOnce() -> NapiResult<()>,
    {
        let hook: Box<Box<dyn FnOnce() -> NapiResult<()>>> = Box::new(Box::new(hook));

        unsafe extern "C" fn cleanup_hook(data: *mut std::os::raw::c_void) {
            unsafe {
                let hook: Box<Box<dyn FnOnce() -> NapiResult<()>>> = Box::from_raw(data as _);
                if let Err(e) = hook() {
                    log::error!("[{}] cleanup hook error.", e);
                }
            }
        }

        let args = Box::into_raw(hook) as _;

        napi_call!(napi_add_env_cleanup_hook, *self, Some(cleanup_hook), args,);

        Ok(CleanupHookHandler {
            env: *self,
            hook: Some(cleanup_hook),
            args,
        })
    }

    #[cfg(feature = "v8")]
    /// Registers hook, which is a function of type napi_async_cleanup_hook, as a function
    /// to be run with the remove_handle and arg parameters once the current Node.js
    /// environment exits.
    ///
    /// Unlike napi_add_env_cleanup_hook, the hook is allowed to be asynchronous.
    ///
    /// Otherwise, behavior generally matches that of napi_add_env_cleanup_hook.
    ///
    /// If remove_handle is not NULL, an opaque value will be stored in it that must later
    /// be passed to napi_remove_async_cleanup_hook, regardless of whether the hook has
    /// already been invoked. Typically, that happens when the resource for which this hook
    /// was added is being torn down anyway.
    pub fn add_async_cleanup_hook<Hook>(
        &mut self,
        hook: Hook,
    ) -> NapiResult<Option<AsyncCleanupHookHandler>>
    where
        Hook: FnOnce(AsyncCleanupHookHandler) -> NapiResult<()>,
    {
        let hook: Box<Box<dyn FnOnce(AsyncCleanupHookHandler) -> NapiResult<()>>> =
            Box::new(Box::new(hook));

        // The body of the function should initiate the asynchronous cleanup actions at the end of
        // which handle must be passed in a call to napi_remove_async_cleanup_hook.
        unsafe extern "C" fn async_cleanup_hook(
            handle: napi_async_cleanup_hook_handle,
            data: *mut std::os::raw::c_void,
        ) {
            unsafe {
                let hook: Box<Box<dyn FnOnce(AsyncCleanupHookHandler) -> NapiResult<()>>> =
                    Box::from_raw(data as _);
                if let Err(e) = hook(AsyncCleanupHookHandler(handle)) {
                    log::error!("[{}] cleanup hook error.", e);
                }
            }
        }

        let maybe_handler = napi_call!(
            =napi_add_async_cleanup_hook,
            *self,
            Some(async_cleanup_hook),
            Box::into_raw(hook) as _,
        );

        if maybe_handler.is_null() {
            return Ok(None);
        }

        Ok(Some(AsyncCleanupHookHandler(maybe_handler)))
    }
}

#[cfg(feature = "v3")]
pub struct CleanupHookHandler {
    env: NapiEnv,
    hook: Option<unsafe extern "C" fn(data: *mut std::os::raw::c_void)>,
    args: *mut std::os::raw::c_void,
}

#[cfg(feature = "v3")]
impl CleanupHookHandler {
    pub fn remove(self) -> NapiResult<()> {
        napi_call!(napi_remove_env_cleanup_hook, self.env, self.hook, self.args,);
        Ok(())
    }
}

#[cfg(feature = "v8")]
#[derive(Debug)]
pub struct AsyncCleanupHookHandler(napi_async_cleanup_hook_handle);

#[cfg(feature = "v8")]
impl AsyncCleanupHookHandler {
    pub fn remove(self) -> NapiResult<()> {
        napi_call!(napi_remove_async_cleanup_hook, self.0);
        Ok(())
    }
}
