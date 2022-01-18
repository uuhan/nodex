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
        let value = napi_call!(=napi_get_node_version, self.raw());
        unsafe { Ok(std::ptr::read(value)) }
    }

    /// get napi version
    pub fn napi_version(&self) -> NapiResult<u32> {
        Ok(napi_call!(=napi_get_version, self.raw()))
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
    pub fn func_named<Func, const N: usize>(
        &self,
        name: impl AsRef<str>,
        func: Func,
    ) -> NapiResult<JsFunction>
    where
        Func: FnMut(JsObject, [JsValue; N]) -> NapiResult<JsValue>,
    {
        JsFunction::with(*self, Some(name), func)
    }

    /// Create a js function with a rust closure.
    pub fn func<Func, const N: usize>(&self, func: Func) -> NapiResult<JsFunction>
    where
        Func: FnMut(JsObject, [JsValue; N]) -> NapiResult<JsValue>,
    {
        JsFunction::with(*self, Option::<String>::None, func)
    }

    /// Create a named js function with a rust function
    pub fn function_named(
        &self,
        name: impl AsRef<str>,
        func: extern "C" fn(env: napi_env, info: napi_callback_info) -> napi_value,
    ) -> NapiResult<JsFunction> {
        JsFunction::new(*self, Some(name), func)
    }

    /// Create a js function with a rust function
    pub fn function(
        &self,
        func: extern "C" fn(env: napi_env, info: napi_callback_info) -> napi_value,
    ) -> NapiResult<JsFunction> {
        JsFunction::new(*self, Option::<String>::None, func)
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
            self.raw(),
            object.raw(),
            properties.as_ref().len(),
            properties.as_ref().as_ptr() as *const _,
        );
        Ok(())
    }
}
