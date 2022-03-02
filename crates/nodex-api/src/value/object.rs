use crate::{api, prelude::*};
use std::{ffi::CString, mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsObject(pub(crate) JsValue);

impl JsObject {
    pub(crate) fn from_value(value: JsValue) -> JsObject {
        JsObject(value)
    }

    pub fn new(env: NapiEnv) -> NapiResult<JsObject> {
        let value = napi_call!(=napi_create_object, env);
        Ok(JsObject(JsValue::from_raw(env, value)))
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
            self.env(),
            self.raw(),
            name.as_ptr(),
            value.raw(),
        )
    }

    /// This API returns the names of the enumerable properties of object as an array of strings.
    /// The properties of object whose key is a symbol will not be included.
    pub fn get_property_names(&self) -> NapiResult<JsArray> {
        let names = napi_call!(=napi_get_property_names, self.env(), self.raw());
        Ok(JsArray::from_raw(self.env(), names))
    }

    #[cfg(feature = "v6")]
    /// This API returns an array containing the names of the available properties of this object.
    pub fn get_all_property_names(
        &self,
        mode: NapiKeyCollectionMode,
        filter: NapiKeyFilter,
        conversion: NapiKeyConversion,
    ) -> NapiResult<JsArray> {
        let names = napi_call!(
            =napi_get_all_property_names,
            self.env(),
            self.raw(),
            mode as _,
            filter,
            conversion as _,
        );
        Ok(JsArray::from_raw(self.env(), names))
    }

    /// Set value by string-like key.
    pub fn set<T: NapiValueT>(&mut self, key: impl AsRef<str>, value: T) -> NapiResult<()> {
        let name = self.env().string(key.as_ref())?;
        let value = self.set_property(name, value)?;
        Ok(())
    }

    /// This API set a property on the Object passed in.
    pub fn set_property(&mut self, key: impl NapiValueT, value: impl NapiValueT) -> NapiResult<()> {
        napi_call!(
            napi_set_property,
            self.env(),
            self.raw(),
            key.raw(),
            value.raw(),
        )
    }

    /// This API gets the requested property from the Object passed in.
    pub fn get_property(&self, key: impl NapiValueT) -> NapiResult<JsValue> {
        let value = napi_call!(=napi_get_property, self.env(), self.raw(), key.raw());
        Ok(JsValue::from_raw(self.env(), value))
    }

    /// Get value by string-like key.
    pub fn get<T: NapiValueT>(&self, key: impl AsRef<str>) -> NapiResult<T> {
        let name = self.env().string(key.as_ref())?;
        let value = self.get_property(name)?;
        Ok(T::from_raw(self.env(), value.raw()))
    }

    /// This API checks if the Object passed in has the named property.
    pub fn has_property(&self, key: impl NapiValueT) -> NapiResult<bool> {
        Ok(napi_call!(=napi_has_property, self.env(), self.raw(), key.raw()))
    }

    /// This API attempts to delete the key own property from object.
    pub fn delete_property(&self, key: impl NapiValueT) -> NapiResult<bool> {
        Ok(napi_call!(=napi_delete_property, self.env(), self.raw(), key.raw()))
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
            self.env(),
            self.raw(),
            key.raw(),
        ))
    }

    /// This method is equivalent to calling napi_get_property with a napi_value created
    /// from the string passed in as utf8Name.
    pub fn get_named_property(&self, key: impl AsRef<str>) -> NapiResult<JsValue> {
        let name = napi_s!(key.as_ref())?;
        let value = napi_call!(
            =napi_get_named_property,
            self.env(),
            self.raw(),
            name.as_ptr(),
        );
        Ok(JsValue::from_raw(self.env(), value))
    }

    /// This method is equivalent to calling napi_has_property with a napi_value created from the
    /// string passed in as utf8Name.
    pub fn has_named_property(&self, key: impl AsRef<str>) -> NapiResult<bool> {
        let name = napi_s!(key.as_ref())?;
        Ok(napi_call!(
            =napi_has_named_property,
            self.env(),
            self.raw(),
            name.as_ptr(),
        ))
    }

    /// This API sets an element on the Object passed in.
    pub fn set_element(&mut self, index: u32, value: impl NapiValueT) -> NapiResult<()> {
        napi_call!(napi_set_element, self.env(), self.raw(), index, value.raw())
    }

    /// This API gets the element at the requested index.
    pub fn get_element(&mut self, index: u32) -> NapiResult<JsValue> {
        let result = napi_call!(
            =napi_get_element,
            self.env(),
            self.raw(),
            index,
        );

        Ok(JsValue::from_raw(self.env(), result))
    }

    /// This API returns if the Object passed in has an element at the requested index.
    pub fn has_element(&mut self, index: u32) -> NapiResult<bool> {
        Ok(napi_call!(
            =napi_has_element,
            self.env(),
            self.raw(),
            index,
        ))
    }

    /// This API attempts to delete the specified index from object.
    pub fn delete_element(&mut self, index: u32) -> NapiResult<bool> {
        Ok(napi_call!(
            =napi_delete_element,
            self.env(),
            self.raw(),
            index,
        ))
    }

    #[cfg(feature = "v8")]
    #[doc = "Object.freeze()"]
    pub fn freeze(&mut self) -> NapiResult<()> {
        napi_call!(napi_object_freeze, self.env(), self.raw())
    }

    #[cfg(feature = "v8")]
    #[doc = "Object.seal()"]
    pub fn seal(&mut self) -> NapiResult<()> {
        napi_call!(napi_object_seal, self.env(), self.raw())
    }
}

napi_value_t!(JsObject);

impl NapiValueCheck for JsObject {
    fn check(&self) -> NapiResult<bool> {
        Ok(self.kind()? == NapiValuetype::Object)
    }
}
