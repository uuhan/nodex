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
    #[inline]
    pub fn from_raw(env: napi_env) -> NapiEnv {
        NapiEnv(env)
    }

    /// access raw napi_env from `NapiEnv`
    #[inline]
    pub fn raw(&self) -> napi_env {
        self.0
    }

    /// This API returns the global object.
    #[inline]
    pub fn global(&self) -> NapiResult<JsGlobal> {
        JsGlobal::new(*self)
    }

    /// get node version
    /// the returned buffer is statically allocated and does not need to be freed.
    #[inline]
    pub fn node_version(&self) -> NapiResult<napi_node_version> {
        let value = napi_call!(=napi_get_node_version, *self);
        unsafe { Ok(std::ptr::read(value)) }
    }

    /// get napi version
    #[inline]
    pub fn napi_version(&self) -> NapiResult<u32> {
        Ok(napi_call!(=napi_get_version, *self))
    }

    /// Return null object
    #[inline]
    pub fn null(&self) -> NapiResult<JsNull> {
        JsNull::new(*self)
    }

    /// Return undefined object
    #[inline]
    pub fn undefined(&self) -> NapiResult<JsUndefined> {
        JsUndefined::new(*self)
    }

    /// This API is used to convert from the C int32_t type to the JavaScript number type.
    /// The JavaScript number type is described in Section 6.1.6 of the ECMAScript Language Specification.
    #[inline]
    pub fn int32(&self, value: i32) -> NapiResult<JsNumber> {
        JsNumber::int32(*self, value)
    }

    /// This API is used to convert from the C uint32_t type to the JavaScript number type.
    /// The JavaScript number type is described in Section 6.1.6 of the ECMAScript Language Specification.
    #[inline]
    pub fn uint32(&self, value: u32) -> NapiResult<JsNumber> {
        JsNumber::uint32(*self, value)
    }

    /// This API is used to convert from the C int64_t type to the JavaScript number type.
    /// The JavaScript number type is described in Section 6.1.6 of the ECMAScript Language Specification. Note the complete range of int64_t cannot be represented with full precision in JavaScript. Integer values outside the range of Number.MIN_SAFE_INTEGER -(2**53 - 1) - Number.MAX_SAFE_INTEGER (2**53 - 1) will lose precision.
    #[inline]
    pub fn int64(&self, value: i64) -> NapiResult<JsNumber> {
        JsNumber::int64(*self, value)
    }

    /// This API is used to convert from the C double type to the JavaScript number type.
    /// The JavaScript number type is described in Section 6.1.6 of the ECMAScript Language Specification.
    #[inline]
    pub fn double(&self, value: f64) -> NapiResult<JsNumber> {
        JsNumber::double(*self, value)
    }

    /// This API creates a JavaScript string value from a UTF8-encoded C string. The native string is copied.
    /// The JavaScript string type is described in Section 6.1.4 of the ECMAScript Language Specification.
    #[inline]
    pub fn string(&self, s: impl AsRef<str>) -> NapiResult<JsString> {
        JsString::new(*self, s)
    }

    /// Create an empty js array.
    #[inline]
    pub fn array(&self) -> NapiResult<JsArray> {
        JsArray::empty(*self)
    }

    #[cfg(feature = "v6")]
    /// Create a bigint_int64.
    #[inline]
    pub fn bigint_i64(&self, value: i64) -> NapiResult<JsBigInt<i64>> {
        JsBigInt::<i64>::new_i64(*self, value)
    }

    #[cfg(feature = "v6")]
    /// Create a bigint_unt64.
    #[inline]
    pub fn bigint_u64(&self, value: u64) -> NapiResult<JsBigInt<u64>> {
        JsBigInt::<u64>::new_u64(*self, value)
    }

    /// Create a boolean.
    #[inline]
    pub fn boolean(&self, boolean: bool) -> NapiResult<JsBoolean> {
        JsBoolean::new(*self, boolean)
    }

    /// Create a Buffer<N>
    #[inline]
    pub fn buffer<const N: usize>(&self) -> NapiResult<JsBuffer<N>> {
        JsBuffer::<N>::create(*self)
    }

    /// Create a Buffer<N> from [u8]
    #[inline]
    pub fn buffer_copy<const N: usize>(&self, data: [u8; N]) -> NapiResult<JsBuffer<N>> {
        JsBuffer::<N>::create_copy(*self, data)
    }

    /// Create an ArrayBuffer
    #[inline]
    pub fn arraybuffer(&self, buffer: impl AsRef<[u8]>) -> NapiResult<JsArrayBuffer> {
        JsArrayBuffer::new(*self, buffer)
    }

    #[cfg(feature = "v5")]
    /// Create a Date.
    pub fn date(&self, time: f64) -> NapiResult<JsDate> {
        JsDate::new(*self, time)
    }

    /// This API creates a JavaScript symbol value from a UTF8-encoded C string.
    /// The JavaScript symbol type is described in Section 19.4 of the ECMAScript Language Specification.
    #[inline]
    pub fn symbol(&self) -> NapiResult<JsSymbol> {
        JsSymbol::new(*self)
    }

    /// Symbol with description.
    #[inline]
    pub fn symbol_description(&self, desc: JsString) -> NapiResult<JsSymbol> {
        JsSymbol::description(*self, desc)
    }

    /// This API allocates a default JavaScript Object. It is the equivalent of doing new Object() in JavaScript.
    /// The JavaScript Object type is described in Section 6.1.7 of the ECMAScript Language Specification.
    #[inline]
    pub fn object(&self) -> NapiResult<JsObject> {
        JsObject::new(*self)
    }

    /// The async context
    #[inline]
    pub fn context(&self, name: impl AsRef<str>) -> NapiResult<NapiAsyncContext> {
        NapiAsyncContext::new(*self, name)
    }

    /// Create an external data.
    #[inline]
    pub fn external<T>(
        &self,
        value: T,
        finalizer: impl FnOnce(NapiEnv, T) -> NapiResult<()> + 'static,
    ) -> NapiResult<JsExternal<T>> {
        JsExternal::<T>::new(*self, value, finalizer)
    }

    /// Create a js function with a rust closure.
    #[inline]
    pub fn func<T: FromJsArgs, R>(
        &self,
        func: impl FnMut(JsObject, T) -> NapiResult<R> + 'static,
    ) -> NapiResult<Function<R>>
    where
        T: FromJsArgs,
        R: NapiValueT,
    {
        Function::<R>::new(*self, Option::<String>::None, func)
    }

    /// Create a named js function with a rust closure.
    #[inline]
    pub fn func_named<T: FromJsArgs, R>(
        &self,
        name: impl AsRef<str>,
        func: impl FnMut(JsObject, T) -> NapiResult<R> + 'static,
    ) -> NapiResult<Function<R>>
    where
        T: FromJsArgs,
        R: NapiValueT,
    {
        Function::<R>::new(*self, Some(name), func)
    }

    /// Create a named js function with a rust function
    #[inline]
    pub fn function_named(
        &self,
        name: impl AsRef<str>,
        func: extern "C" fn(env: NapiEnv, info: napi_callback_info) -> napi_value,
    ) -> NapiResult<Function<JsValue>> {
        let value = napi_call!(
            =napi_create_function,
            *self,
            name.as_ref().as_ptr() as CharPointer,
            name.as_ref().len(),
            Some(func),
            std::ptr::null_mut(),
        );

        Ok(Function::<JsValue>::from_value(JsValue::from_raw(
            *self, value,
        )))
    }

    /// Create a js function with a rust function
    #[inline]
    pub fn function(
        &self,
        func: extern "C" fn(env: NapiEnv, info: napi_callback_info) -> napi_value,
    ) -> NapiResult<Function<JsValue>> {
        let value = napi_call!(
            =napi_create_function,
            *self,
            std::ptr::null(),
            0,
            Some(func),
            std::ptr::null_mut(),
        );

        Ok(Function::<JsValue>::from_value(JsValue::from_raw(
            *self, value,
        )))
    }

    /// Create a js class with a rust closure
    #[inline]
    pub fn class<T, R>(
        &self,
        name: impl AsRef<str>,
        func: impl FnMut(JsObject, T) -> NapiResult<R> + 'static,
        properties: impl AsRef<[NapiPropertyDescriptor]>,
    ) -> NapiResult<JsClass>
    where
        T: FromJsArgs,
        R: NapiValueT,
    {
        JsClass::new(*self, name, func, properties)
    }

    /// Create an async work with shared state
    #[inline]
    pub fn async_work<T>(
        &self,
        name: impl AsRef<str>,
        state: T,
        execute: impl FnMut(&mut T) + Send + 'static,
        complete: impl FnMut(NapiEnv, NapiStatus, T) -> NapiResult<()> + 'static,
    ) -> NapiResult<NapiAsyncWork<T>> {
        NapiAsyncWork::new(*self, name, state, execute, complete)
    }

    /// Create a promise with a work & complete closure.
    #[inline]
    pub fn promise<T, L: NapiValueT + Copy + 'static, R: NapiValueT + Copy + 'static>(
        &self,
        mut work: impl FnMut(&mut T) + Send + 'static,
        mut complete: impl FnMut(JsPromise<L, R>, NapiStatus, T) -> NapiResult<()> + 'static,
    ) -> NapiResult<JsPromise<L, R>>
    where
        T: Default,
    {
        JsPromise::<L, R>::spawn(*self, work, complete)
    }

    #[cfg(feature = "v4")]
    /// Create a NapiThreadsafeFunction.
    #[inline]
    pub fn tsfn<Data, R, const N: usize>(
        &self,
        name: impl AsRef<str>,
        func: Function<R>,
        finalizer: impl FnOnce(NapiEnv) -> NapiResult<()> + 'static,
        callback: impl FnMut(Function<R>, Data) -> NapiResult<()> + 'static,
    ) -> NapiResult<NapiThreadsafeFunction<Data, N>>
    where
        R: NapiValueT,
    {
        NapiThreadsafeFunction::<Data, N>::new(*self, name, func, finalizer, callback)
    }

    /// This method allows the efficient definition of multiple properties on a given object. The
    /// properties are defined using property descriptors (see napi_property_descriptor). Given an
    /// array of such property descriptors, this API will set the properties on the object one at a
    /// time, as defined by DefineOwnProperty() (described in Section 9.1.6 of the ECMA-262
    /// specification).
    #[inline]
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
        )
    }

    /// This API throws the JavaScript value provided.
    #[inline]
    pub fn throw<T: NapiValueT>(&self, to_throw: T) -> NapiResult<()> {
        napi_call!(napi_throw, *self, to_throw.raw())
    }

    /// This API throws a JavaScript Error with the text provided.
    #[inline]
    pub fn throw_error(&self, msg: impl AsRef<str>) -> NapiResult<()> {
        use std::ffi::CString;
        let msg = napi_s!(msg.as_ref())?;
        napi_call!(napi_throw_error, *self, std::ptr::null(), msg.as_ptr())
    }

    /// This API throws a JavaScript Error with the text provided.
    #[inline]
    pub fn throw_error_code(&self, msg: impl AsRef<str>, code: impl AsRef<str>) -> NapiResult<()> {
        use std::ffi::CString;
        let msg = napi_s!(msg.as_ref())?;
        let code = napi_s!(code.as_ref())?;
        napi_call!(napi_throw_error, *self, code.as_ptr(), msg.as_ptr())
    }

    /// This API throws a JavaScript TypeError with the text provided.
    #[inline]
    pub fn throw_type_error(&self, msg: impl AsRef<str>) -> NapiResult<()> {
        let msg = napi_s!(msg.as_ref()).map_err(|_| NapiStatus::StringExpected)?;
        napi_call!(napi_throw_type_error, *self, std::ptr::null(), msg.as_ptr())
    }

    /// This API throws a JavaScript TypeError with the text provided.
    #[inline]
    pub fn throw_type_error_code(
        &self,
        msg: impl AsRef<str>,
        code: impl AsRef<str>,
    ) -> NapiResult<()> {
        let msg = napi_s!(msg.as_ref()).map_err(|_| NapiStatus::StringExpected)?;
        let code = napi_s!(code.as_ref())?;
        napi_call!(napi_throw_type_error, *self, code.as_ptr(), msg.as_ptr())
    }

    /// This API throws a JavaScript TypeError with the text provided.
    #[inline]
    pub fn throw_range_error(
        &self,
        msg: impl AsRef<str>,
        code: Option<impl AsRef<str>>,
    ) -> NapiResult<()> {
        use std::ffi::CString;
        let msg = napi_s!(msg.as_ref())?;
        napi_call!(
            napi_throw_range_error,
            *self,
            std::ptr::null(),
            msg.as_ptr()
        )
    }

    /// This API throws a JavaScript TypeError with the text provided.
    #[inline]
    pub fn throw_range_error_code(
        &self,
        msg: impl AsRef<str>,
        code: impl AsRef<str>,
    ) -> NapiResult<()> {
        use std::ffi::CString;
        let msg = napi_s!(msg.as_ref())?;
        let code = napi_s!(code.as_ref())?;
        napi_call!(napi_throw_range_error, *self, code.as_ptr(), msg.as_ptr())
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

    /// This API retrieves a napi_extended_error_info structure with information about the last
    /// error that occurred.
    ///
    /// The content of the napi_extended_error_info returned is only valid up until a Node-API
    /// function is called on the same env. This includes a call to napi_is_exception_pending
    /// so it may often be necessary to make a copy of the information so that it can be used
    /// later. The pointer returned in error_message points to a statically-defined string so
    /// it is safe to use that pointer if you have copied it out of the error_message field
    /// (which will be overwritten) before another Node-API function was called.
    ///
    /// Do not rely on the content or format of any of the extended information as it is not
    /// subject to SemVer and may change at any time. It is intended only for logging purposes.
    ///
    /// This API can be called even if there is a pending JavaScript exception.
    #[inline]
    pub fn get_last_error_info(&self) -> NapiResult<NapiExtendedErrorInfo> {
        let info = napi_call!(=napi_get_last_error_info, *self);
        unsafe { Ok(std::ptr::read(info)) }
    }

    /// Return true if an exception is pending.
    /// This API can be called even if there is a pending JavaScript exception.
    #[inline]
    pub fn is_exception_pending(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_exception_pending, *self))
    }

    /// Create a js Error.
    pub fn error(&self, msg: impl AsRef<str>) -> NapiResult<JsError> {
        JsError::error(*self, msg, Option::<String>::None)
    }

    /// Trigger an 'uncaughtException' in JavaScript. Useful if an async callback throws an
    /// exception with no way to recover.
    #[inline]
    #[cfg(feature = "v3")]
    pub fn fatal_exception(&self, err: JsError) -> NapiResult<()> {
        napi_call!(napi_fatal_exception, *self, err.raw())
    }

    /// Create a handle scope
    #[inline]
    pub fn handle_scope(&self) -> NapiResult<NapiHandleScope> {
        NapiHandleScope::open(*self)
    }

    /// Run in a scope.
    #[inline]
    pub fn scope<T>(&self, task: impl Fn(NapiHandleScope) -> T) -> NapiResult<T> {
        Ok(task(self.handle_scope()?))
    }

    /// Create a escapable handle scope
    #[inline]
    pub fn escapable_handle_scope(&self) -> NapiResult<NapiEscapableHandleScope> {
        NapiEscapableHandleScope::open(*self)
    }

    /// Run in an escapable scope.
    pub fn escapable_scope<T>(
        &self,
        task: impl Fn(NapiEscapableHandleScope) -> T,
    ) -> NapiResult<T> {
        Ok(task(self.escapable_handle_scope()?))
    }

    #[cfg(feature = "v3")]
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
    #[inline]
    pub fn add_cleanup_hook<Hook>(&self, hook: Hook) -> NapiResult<CleanupHookHandler>
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

        napi_call!(napi_add_env_cleanup_hook, *self, Some(cleanup_hook), args);

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
    #[inline]
    pub fn add_async_cleanup_hook<Hook>(
        &self,
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

    /// This function gives V8 an indication of the amount of externally allocated memory that is
    /// kept alive by JavaScript objects (i.e. a JavaScript object that points to its own memory
    /// allocated by a native module). Registering externally allocated memory will trigger global
    /// garbage collections more often than it would otherwise.
    #[inline]
    pub fn adjust_external_memory(&self, changes: i64) -> NapiResult<i64> {
        Ok(napi_call!(=napi_adjust_external_memory, *self, changes))
    }

    /// This function executes a string of JavaScript code and returns its result with the
    /// following caveats:
    ///
    /// * Unlike eval, this function does not allow the script to access the current lexical
    /// scope, and therefore also does not allow to access the module scope, meaning that
    /// pseudo-globals such as require will not be available.
    /// * The script can access the global scope. Function and var declarations in the script
    /// will be added to the global object. Variable declarations made using let and const will
    /// be visible globally, but will not be added to the global object.
    /// * The value of this is global within the script.
    #[inline]
    pub fn run_script<R: NapiValueT>(&self, script: impl AsRef<str>) -> NapiResult<R> {
        let result = napi_call!(
            =napi_run_script,
            *self,
            JsString::new(*self, script)?.raw(),
        );
        Ok(R::from_raw(*self, result))
    }

    #[cfg(feature = "v2")]
    #[inline]
    pub fn get_uv_event_loop(&self) -> NapiResult<uv_loop_s> {
        unsafe { Ok(*napi_call!(=napi_get_uv_event_loop, *self)) }
    }

    #[cfg(feature = "v6")]
    #[allow(clippy::type_complexity)]
    #[inline]
    /// This API associates data with the currently running Agent. data can later be retrieved
    /// using napi_get_instance_data(). Any existing data associated with the currently running
    /// Agent which was set by means of a previous call to napi_set_instance_data() will be
    /// overwritten. If a finalize_cb was provided by the previous call, it will not be called.
    pub fn set_instance_data<T, F>(&self, data: T, finalizer: F) -> NapiResult<()>
    where
        F: FnOnce(NapiEnv, T) -> NapiResult<()>,
    {
        let data = Box::into_raw(Box::new(data)) as DataPointer;

        // NB: Because we add a closure to the napi finalizer, it's better
        // to **CAPTURE** the leaked data from rust side, so here we just
        // ignore the passed in native data pointer.
        unsafe extern "C" fn finalizer_trampoline<T>(
            env: NapiEnv,
            data: DataPointer,
            finalizer: DataPointer,
        ) {
            // NB: here we collect the memory of finalizer closure
            let finalizer: Box<Box<dyn FnOnce(NapiEnv, T) -> NapiResult<()>>> =
                Box::from_raw(finalizer as _);

            let data: Box<T> = Box::from_raw(data as _);

            if let Err(err) = finalizer(env, *data) {
                log::error!("NapiValueT::finalizer(): {}", err);
            }
        }

        let finalizer: Box<Box<dyn FnOnce(NapiEnv, T) -> NapiResult<()>>> =
            Box::new(Box::new(finalizer));

        napi_call!(
            napi_set_instance_data,
            *self,
            data,
            Some(finalizer_trampoline::<T>),
            Box::into_raw(finalizer) as _,
        )
    }

    #[cfg(feature = "v6")]
    /// This API retrieves data that was previously associated with the currently running Agent via
    /// napi_set_instance_data(). If no data is set, the call will succeed and data will be set to
    /// NULL.
    #[inline]
    pub fn get_instance_data<T>(&self) -> NapiResult<Option<&mut T>> {
        let data = napi_call!(=napi_get_instance_data, *self) as *mut T;
        if data.is_null() {
            Ok(None)
        } else {
            unsafe { Ok(Some(&mut *data)) }
        }
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
        napi_call!(napi_remove_env_cleanup_hook, self.env, self.hook, self.args)
    }
}

#[cfg(feature = "v8")]
#[derive(Debug)]
pub struct AsyncCleanupHookHandler(napi_async_cleanup_hook_handle);

#[cfg(feature = "v8")]
impl AsyncCleanupHookHandler {
    pub fn remove(self) -> NapiResult<()> {
        napi_call!(napi_remove_async_cleanup_hook, self.0)
    }
}
