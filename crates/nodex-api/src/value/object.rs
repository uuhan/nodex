use crate::{api, prelude::*};
use std::{ffi::CString, mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsObject(pub(crate) JsValue);

impl JsObject {
    pub(crate) fn from_value(value: JsValue) -> JsObject {
        JsObject(value)
    }

    /// NB: This is a special JsObject that should only be used in napi_register_module_v1.
    pub fn napi_module_exports(env: napi_env, value: napi_value) -> JsObject {
        JsObject(JsValue(NapiEnv::from_raw(env), value))
    }

    pub fn new(env: NapiEnv) -> NapiResult<JsObject> {
        let value = napi_call!(=napi_create_object, env.raw());
        Ok(JsObject(JsValue::from_raw(env, value)))
    }

    /// This API gets the requested property from the Object passed in.
    pub fn get_property(&self, key: impl NapiValueT) -> NapiResult<JsValue> {
        let value = napi_call!(=napi_get_property, self.env().raw(), self.raw(), key.raw());
        Ok(JsValue::from_raw(self.env(), value))
    }

    /// This API set a property on the Object passed in.
    pub fn set_property(&mut self, key: impl NapiValueT, value: impl NapiValueT) -> NapiResult<()> {
        napi_call!(
            napi_set_property,
            self.env().raw(),
            self.raw(),
            key.raw(),
            value.raw(),
        );
        Ok(())
    }

    /// This method is equivalent to calling napi_get_property with a napi_value created from the
    /// string passed in as utf8Name.
    pub fn set_named_property(
        &mut self,
        key: impl AsRef<str>,
        value: impl NapiValueT,
    ) -> NapiResult<()> {
        let name = CString::new(key.as_ref()).map_err(|_| NapiStatus::StringExpected)?;
        napi_call!(
            napi_set_named_property,
            self.env().raw(),
            self.raw(),
            name.as_ptr(),
            value.raw(),
        );
        Ok(())
    }

    /// This API checks if the Object passed in has the named own property. key must be a string or
    /// a symbol, or an error will be thrown. Node-API will not perform any conversion between data
    /// types.
    pub fn has_own_property(
        &mut self,
        key: impl NapiValueT,
        value: impl NapiValueT,
    ) -> NapiResult<bool> {
        Ok(napi_call!(
            =napi_has_own_property,
            self.env().raw(),
            self.raw(),
            key.raw(),
        ))
    }

    /// This API returns if the Object passed in has an element at the requested index.
    pub fn has_element(&mut self, index: u32) -> NapiResult<bool> {
        Ok(napi_call!(
            =napi_has_element,
            self.env().raw(),
            self.raw(),
            index,
        ))
    }

    /// This API gets the element at the requested index.
    pub fn get_element(&mut self, index: u32) -> NapiResult<JsValue> {
        let result = napi_call!(
            =napi_get_element,
            self.env().raw(),
            self.raw(),
            index,
        );

        Ok(JsValue::from_raw(self.env(), result))
    }

    /// This API attempts to delete the specified index from object.
    pub fn delete_element(&mut self, index: u32) -> NapiResult<bool> {
        Ok(napi_call!(
            =napi_delete_element,
            self.env().raw(),
            self.raw(),
            index,
        ))
    }

    #[cfg(feature = "v8")]
    #[doc = "Object.freeze()"]
    pub fn freeze(&mut self) -> NapiResult<()> {
        napi_call!(napi_object_freeze, self.env().raw(), self.raw());
        Ok(())
    }

    #[cfg(feature = "v8")]
    #[doc = "Object.seal()"]
    pub fn seal(&mut self) -> NapiResult<()> {
        napi_call!(napi_object_seal, self.env().raw(), self.raw());
        Ok(())
    }
}

impl NapiValueT for JsObject {
    fn value(&self) -> JsValue {
        self.0
    }
}
