use crate::{api, prelude::*};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct NapiPropertyDescriptor(napi_property_descriptor);

impl AsRef<napi_property_descriptor> for NapiPropertyDescriptor {
    fn as_ref(&self) -> &napi_property_descriptor {
        &self.0
    }
}

impl std::ops::Deref for NapiPropertyDescriptor {
    type Target = napi_property_descriptor;
    fn deref(&self) -> &napi_property_descriptor {
        &self.0
    }
}

impl NapiPropertyDescriptor {
    pub fn raw(&self) -> &napi_property_descriptor {
        &self.0
    }
}

#[derive(Debug)]
pub struct DescriptorBuilder {
    pub utf8name: Option<String>,
    pub name: napi_value,
    pub method: napi_callback,
    pub getter: napi_callback,
    pub setter: napi_callback,
    pub value: napi_value,
    pub attributes: NapiPropertyAttributes,
    pub data: *mut ::std::os::raw::c_void,
}

impl DescriptorBuilder {
    pub fn new() -> DescriptorBuilder {
        DescriptorBuilder {
            utf8name: None,
            name: std::ptr::null_mut(),
            method: None,
            getter: None,
            setter: None,
            value: std::ptr::null_mut(),
            attributes: NapiPropertyAttributes::Default,
            data: std::ptr::null_mut(),
        }
    }

    /// Optional string describing the key for the property, encoded as UTF8. One of utf8name or
    /// name must be provided for the property.
    pub fn with_utf8name(mut self, name: impl Into<String>) -> Self {
        self.utf8name.replace(name.into());
        self
    }

    /// Optional napi_value that points to a JavaScript string or symbol to be used as the key for
    /// the property. One of utf8name or name must be provided for the property.
    pub fn with_name(mut self, name: impl ValueInner) -> Self {
        let name = name.downcast();
        if let (Ok(name_string), Ok(name_symbol)) = (name.is_string(), name.is_symbol()) {
            if name_string || name_symbol {
                self.name = name.raw();
            }
        }
        self
    }

    /// The value that's retrieved by a get access of the property if the property is a data
    /// property. If this is passed in, set getter, setter, method and data to NULL (since these
    /// members won't be used).
    pub fn with_value(mut self, value: impl ValueInner) -> Self {
        self.value = value.raw();
        self
    }

    /// Set this to make the property descriptor object's value property to be a JavaScript
    /// function represented by method. If this is passed in, set value, getter and setter to NULL
    /// (since these members won't be used). napi_callback provides more details.
    pub fn with_method(mut self, method: napi_callback) -> Self {
        self.method = method;
        self
    }

    ///  A function to call when a get access of the property is performed. If this is passed in,
    ///  set value and method to NULL (since these members won't be used). The given function is
    ///  called implicitly by the runtime when the property is accessed from JavaScript code (or if
    ///  a get on the property is performed using a Node-API call). napi_callback provides more
    ///  details.
    pub fn with_getter(mut self, getter: napi_callback) -> Self {
        self.getter = getter;
        self
    }

    /// A function to call when a set access of the property is performed. If this is passed in,
    /// set value and method to NULL (since these members won't be used). The given function is
    /// called implicitly by the runtime when the property is set from JavaScript code (or if a set
    /// on the property is performed using a Node-API call). napi_callback provides more details.
    pub fn with_setter(mut self, setter: napi_callback) -> Self {
        self.setter = setter;
        self
    }

    /// The attributes associated with the particular property. See napi_property_attributes.
    pub fn with_attribute(mut self, attribute: NapiPropertyAttributes) -> Self {
        self.attributes |= attribute;
        self
    }

    /// build finale `NapiPropertyDescriptor`
    pub fn build(mut self) -> NapiResult<NapiPropertyDescriptor> {
        let utf8name = if let Some(name) = self.utf8name {
            name.as_ptr() as *const std::os::raw::c_char
        } else {
            std::ptr::null()
        };

        let name = self.name;

        // NB: panic if utf8name and name is both null
        if (utf8name.is_null() && name.is_null()) {
            return Err(NapiStatus::InvalidArg);
        }

        let method = self.method.take();
        let getter = self.getter.take();
        let setter = self.setter.take();
        let value = self.value;
        let attributes = self.attributes.bits();
        let mut data = self.data;

        Ok(NapiPropertyDescriptor(napi_property_descriptor {
            utf8name,
            name,
            method,
            getter,
            setter,
            value,
            attributes,
            data,
        }))
    }
}

impl Default for DescriptorBuilder {
    fn default() -> Self {
        Self::new()
    }
}
