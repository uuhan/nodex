use std::{marker::PhantomData, mem::MaybeUninit};

use crate::{
    api::{self, napi_node_version},
    prelude::*,
};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NapiEnv<'a>(napi_env, PhantomData<&'a napi_env>);

impl<'a> AsRef<napi_env> for NapiEnv<'a> {
    fn as_ref(&self) -> &napi_env {
        &self.0
    }
}

impl<'a> NapiEnv<'a> {
    /// create `NapiEnv` from raw napi_env
    pub fn from_raw(env: napi_env) -> NapiEnv<'a> {
        NapiEnv(env, PhantomData)
    }

    /// access raw napi_env from `NapiEnv`
    pub fn raw(&self) -> napi_env {
        self.0
    }

    /// get current global object
    pub fn global(&self) -> NapiResult<JsValue> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_get_global(self.raw(), result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(JsValue::from_raw(*self, value))
    }

    /// get node version
    /// the returned buffer is statically allocated and does not need to be freed.
    pub fn node_version(&self) -> NapiResult<napi_node_version> {
        unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_get_node_version(self.raw(), result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            Ok(std::ptr::read(result.assume_init()))
        }
    }

    /// get napi version
    pub fn napi_version(&self) -> NapiResult<u32> {
        unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_get_version(self.raw(), result.as_mut_ptr());

            if status.err() {
                return Err(status);
            }

            Ok(result.assume_init())
        }
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
    pub fn number(&self, value: f64) -> NapiResult<JsNumber> {
        JsNumber::new(*self, value)
    }

    /// This API creates a JavaScript string value from a UTF8-encoded C string. The native string is copied.
    /// The JavaScript string type is described in Section 6.1.4 of the ECMAScript Language Specification.
    pub fn string(&self, s: impl AsRef<str>) -> NapiResult<JsString> {
        JsString::new(*self, s)
    }

    /// This API allocates a default JavaScript Object. It is the equivalent of doing new Object() in JavaScript.
    /// The JavaScript Object type is described in Section 6.1.7 of the ECMAScript Language Specification.
    pub fn object(&self) -> NapiResult<JsObject> {
        JsObject::new(*self)
    }

    /// The async context
    pub fn context(&self, name: impl AsRef<str>) -> NapiResult<NapiAsyncContext<'a>> {
        NapiAsyncContext::new(*self, name)
    }

    /// This method allows the efficient definition of multiple properties on a given object. The
    /// properties are defined using property descriptors (see napi_property_descriptor). Given an
    /// array of such property descriptors, this API will set the properties on the object one at a
    /// time, as defined by DefineOwnProperty() (described in Section 9.1.6 of the ECMA-262
    /// specification).
    pub fn define_properties(
        &self,
        object: impl ValueInner,
        properties: impl AsRef<[NapiPropertyDescriptor]>,
    ) -> NapiResult<()> {
        unsafe {
            let status = api::napi_define_properties(
                self.raw(),
                object.raw(),
                properties.as_ref().len(),
                properties.as_ref().as_ptr() as *const _,
            );

            if status.err() {
                return Err(status);
            }

            Ok(())
        }
    }
}
