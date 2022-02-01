use crate::{api, prelude::*};
use std::mem::MaybeUninit;

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

/// The DescriptorBuild for value.
pub struct DescriptorValueBuilder {
    pub utf8name: Option<String>,
    pub name: napi_value,
    pub value: napi_value,
    pub attributes: NapiPropertyAttributes,
}

/// The DescriptorBuild for method.
/// NB: there seems no way to reclaim the napi_property_descriptor.data, so it is leaked.
#[allow(clippy::type_complexity)]
pub struct DescriptorMethodBuilder<T: NapiValueT, R: NapiValueT, const N: usize> {
    pub utf8name: Option<String>,
    pub name: napi_value,
    pub method: Option<Box<dyn FnMut(JsObject, [T; N]) -> NapiResult<R> + 'static>>,
    pub attributes: NapiPropertyAttributes,
}

/// The DescriptorBuild for accessor.
/// NB: there seems no way to reclaim the napi_property_descriptor.data, so it is leaked.
#[allow(clippy::type_complexity)]
pub struct DescriptorAccessorBuilder<T: NapiValueT, R: NapiValueT> {
    pub utf8name: Option<String>,
    pub name: napi_value,
    pub getter: Option<Box<dyn FnMut(JsObject) -> NapiResult<R> + 'static>>,
    pub setter: Option<Box<dyn FnMut(JsObject, [T; 1]) -> NapiResult<()> + 'static>>,
    pub attributes: NapiPropertyAttributes,
}

impl DescriptorValueBuilder {
    pub fn new() -> DescriptorValueBuilder {
        DescriptorValueBuilder {
            utf8name: None,
            name: std::ptr::null_mut(),
            value: std::ptr::null_mut(),
            attributes: NapiPropertyAttributes::Default,
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
    pub fn with_name(mut self, name: impl NapiValueT) -> Self {
        let name = name.value();
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
    pub fn with_value(mut self, value: impl NapiValueT) -> Self {
        self.value = value.raw();
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
            std::ffi::CString::new(name)
                .map_err(|_| NapiStatus::StringExpected)?
                .into_raw()
        } else {
            std::ptr::null()
        };

        let name = self.name;

        // NB: panic if utf8name and name is both null
        if (utf8name.is_null() && name.is_null()) {
            return Err(NapiStatus::InvalidArg);
        }

        let method = None;
        let getter = None;
        let setter = None;

        let value = self.value;
        let attributes = self.attributes.bits();

        Ok(NapiPropertyDescriptor(napi_property_descriptor {
            utf8name,
            name,
            method,
            getter,
            setter,
            value,
            attributes,
            data: std::ptr::null_mut(),
        }))
    }
}

impl<T: NapiValueT, R: NapiValueT, const N: usize> DescriptorMethodBuilder<T, R, N> {
    pub fn new() -> Self {
        Self {
            utf8name: None,
            name: std::ptr::null_mut(),
            method: None,
            attributes: NapiPropertyAttributes::Default,
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
    pub fn with_name(mut self, name: impl NapiValueT) -> Self {
        let name = name.value();
        if let (Ok(name_string), Ok(name_symbol)) = (name.is_string(), name.is_symbol()) {
            if name_string || name_symbol {
                self.name = name.raw();
            }
        }
        self
    }

    /// Set this to make the property descriptor object's value property to be a JavaScript
    /// function represented by method. If this is passed in, set value, getter and setter to NULL
    /// (since these members won't be used). napi_callback provides more details.
    pub fn with_method(
        mut self,
        method: impl FnMut(JsObject, [T; N]) -> NapiResult<R> + 'static,
    ) -> Self {
        self.method = Some(Box::new(method));
        self
    }

    /// The attributes associated with the particular property. See napi_property_attributes.
    pub fn with_attribute(mut self, attribute: NapiPropertyAttributes) -> Self {
        self.attributes |= attribute;
        self
    }

    /// build finale `NapiPropertyDescriptor`
    #[allow(clippy::type_complexity)]
    pub fn build(mut self) -> NapiResult<NapiPropertyDescriptor> {
        let utf8name = if let Some(name) = self.utf8name {
            std::ffi::CString::new(name)
                .map_err(|_| NapiStatus::StringExpected)?
                .into_raw()
        } else {
            std::ptr::null()
        };

        let name = self.name;

        // NB: panic if utf8name and name is both null
        if (utf8name.is_null() && name.is_null()) {
            return Err(NapiStatus::InvalidArg);
        }

        extern "C" fn method_trampoline<T: NapiValueT, R: NapiValueT, const N: usize>(
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

                let func: &mut Box<dyn FnMut(JsObject, [T; N]) -> NapiResult<R>> =
                    std::mem::transmute(data);

                (argc, argv, this.assume_init(), func)
            };

            let args = unsafe { argv.map(|arg| T::from_raw(env, arg)) };
            let this = JsObject::from_raw(env, this);

            napi_r!(env, =func(this, args))
        }

        let method = Some(method_trampoline::<T, R, N> as _);
        let data = if let Some(method) = self.method.take() {
            Box::into_raw(Box::new(method)) as _
        } else {
            return Err(NapiStatus::InvalidArg);
        };

        let getter = None;
        let setter = None;
        let value = std::ptr::null_mut();

        let attributes = self.attributes.bits();

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

impl<T: NapiValueT, R: NapiValueT> DescriptorAccessorBuilder<T, R> {
    pub fn new() -> Self {
        Self {
            utf8name: None,
            name: std::ptr::null_mut(),
            getter: None,
            setter: None,
            attributes: NapiPropertyAttributes::Default,
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
    pub fn with_name(mut self, name: impl NapiValueT) -> Self {
        let name = name.value();
        if let (Ok(name_string), Ok(name_symbol)) = (name.is_string(), name.is_symbol()) {
            if name_string || name_symbol {
                self.name = name.raw();
            }
        }
        self
    }

    ///  A function to call when a get access of the property is performed. If this is passed in,
    ///  set value and method to NULL (since these members won't be used). The given function is
    ///  called implicitly by the runtime when the property is accessed from JavaScript code (or if
    ///  a get on the property is performed using a Node-API call). napi_callback provides more
    ///  details.
    pub fn with_getter(mut self, getter: impl FnMut(JsObject) -> NapiResult<R> + 'static) -> Self {
        self.getter = Some(Box::new(getter));
        self
    }

    /// A function to call when a set access of the property is performed. If this is passed in,
    /// set value and method to NULL (since these members won't be used). The given function is
    /// called implicitly by the runtime when the property is set from JavaScript code (or if a set
    /// on the property is performed using a Node-API call). napi_callback provides more details.
    pub fn with_setter(
        mut self,
        setter: impl FnMut(JsObject, [T; 1]) -> NapiResult<()> + 'static,
    ) -> Self {
        self.setter = Some(Box::new(setter));
        self
    }

    /// The attributes associated with the particular property. See napi_property_attributes.
    pub fn with_attribute(mut self, attribute: NapiPropertyAttributes) -> Self {
        self.attributes |= attribute;
        self
    }

    /// build finale `NapiPropertyDescriptor`
    #[allow(clippy::type_complexity)]
    pub fn build(mut self) -> NapiResult<NapiPropertyDescriptor> {
        let utf8name = if let Some(name) = self.utf8name {
            std::ffi::CString::new(name)
                .map_err(|_| NapiStatus::StringExpected)?
                .into_raw()
        } else {
            std::ptr::null()
        };

        let name = self.name;

        // NB: panic if utf8name and name is both null
        if (utf8name.is_null() && name.is_null()) {
            return Err(NapiStatus::InvalidArg);
        }

        extern "C" fn getter_trampoline<T: NapiValueT, R: NapiValueT>(
            env: NapiEnv,
            info: napi_callback_info,
        ) -> napi_value {
            let mut argc = 0;
            let mut argv = [std::ptr::null_mut(); 0];
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

                // NB: the Function maybe called multiple times, so we can shoud leak the
                // closure memory here.
                //
                // With napi >= 5, we can add a finalizer to this function.
                let func: &mut (
                    Option<Box<dyn FnMut(JsObject) -> NapiResult<R>>>,
                    Option<Box<dyn FnMut(JsObject, [T; 1]) -> NapiResult<()>>>,
                ) = std::mem::transmute(data);

                (argc, argv, this.assume_init(), func)
            };

            let this = JsObject::from_raw(env, this);

            napi_r!(env, =func.0.as_mut().unwrap()(this))
        }

        let mut data = (None, None);

        let getter = if let Some(getter) = self.getter {
            data.0 = Some(getter);
            Some(getter_trampoline::<T, R> as _)
        } else {
            None
        };

        extern "C" fn setter_trampoline<T: NapiValueT, R: NapiValueT>(
            env: NapiEnv,
            info: napi_callback_info,
        ) -> napi_value {
            let mut argc = 1;
            let mut argv = [std::ptr::null_mut(); 1];
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

                let func: &mut (
                    Option<Box<dyn FnMut(JsObject) -> NapiResult<R>>>,
                    Option<Box<dyn FnMut(JsObject, [T; 1]) -> NapiResult<()>>>,
                ) = std::mem::transmute(data);

                (argc, argv, this.assume_init(), func)
            };

            let args = unsafe { argv.map(|arg| T::from_raw(env, arg)) };
            let this = JsObject::from_raw(env, this);

            napi_r!(env, func.1.as_mut().unwrap()(this, args))
        }

        let setter = if let Some(setter) = self.setter {
            data.1 = Some(setter);
            Some(setter_trampoline::<T, R> as _)
        } else {
            None
        };

        let attributes = self.attributes.bits();

        let method = None;
        let value = std::ptr::null_mut();

        Ok(NapiPropertyDescriptor(napi_property_descriptor {
            utf8name,
            name,
            method,
            getter,
            setter,
            value,
            attributes,
            data: Box::into_raw(Box::new(data)) as _,
        }))
    }
}

impl Default for DescriptorValueBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: NapiValueT, R: NapiValueT, const N: usize> Default for DescriptorMethodBuilder<T, R, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: NapiValueT, R: NapiValueT> Default for DescriptorAccessorBuilder<T, R> {
    fn default() -> Self {
        Self::new()
    }
}
