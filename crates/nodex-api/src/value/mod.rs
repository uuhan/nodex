use crate::{api, env::NapiEnv, prelude::*};
use std::mem::MaybeUninit;

#[derive(Clone, Copy, Debug)]
pub struct JsValue(pub(crate) NapiEnv, pub(crate) napi_value);

impl JsValue {
    /// `NapiEnv` of this `JsValue`
    pub fn env(&self) -> NapiEnv {
        self.0
    }

    /// raw napi_value of this `JsValue`
    pub fn raw(&self) -> napi_value {
        self.1
    }

    /// check if it is an object
    pub fn is_object(&self) -> NapiResult<bool> {
        Ok(self.kind()? == NapiValuetype::Object)
    }

    /// view it as an object, may fail if it is not an object value
    pub fn as_object(&self) -> NapiResult<JsObject> {
        if self.is_object()? {
            Ok(JsObject::from_value(*self))
        } else {
            Err(NapiStatus::ObjectExpected)
        }
    }

    /// check if it is a string
    pub fn is_string(&self) -> NapiResult<bool> {
        Ok(self.kind()? == NapiValuetype::String)
    }

    /// view it as a string, may fail if it is not a string value
    pub fn as_string(&self) -> NapiResult<JsString> {
        if self.is_string()? {
            Ok(JsString::from_value(*self))
        } else {
            Err(NapiStatus::StringExpected)
        }
    }

    /// check if it is a symbol
    pub fn is_symbol(&self) -> NapiResult<bool> {
        Ok(self.kind()? == NapiValuetype::Symbol)
    }

    /// view it as a symbol, may fail if it is not a symbol value
    pub fn as_symbol(&self) -> NapiResult<JsSymbol> {
        if self.is_symbol()? {
            Ok(JsSymbol::from_value(*self))
        } else {
            Err(NapiStatus::GenericFailure)
        }
    }

    /// view it as an array, may fail if it is not an array value
    pub fn as_array(&self) -> NapiResult<JsArray> {
        if self.is_array()? {
            Ok(JsArray::from_value(*self))
        } else {
            Err(NapiStatus::ArrayExpected)
        }
    }

    /// view it as a typed_array, may fail if it is not a typed_array value
    pub fn as_typedarray(&self) -> NapiResult<JsTypedArray> {
        if self.is_typedarray()? {
            Ok(JsTypedArray::from_value(*self))
        } else {
            Err(NapiStatus::GenericFailure)
        }
    }

    /// view it as an array_buffer, may faile if it is not an array_buffer value
    pub fn as_arraybuffer(&self) -> NapiResult<JsArrayBuffer> {
        if self.is_arraybuffer()? {
            Ok(JsArrayBuffer::from_value(*self))
        } else {
            Err(NapiStatus::ArraybufferExpected)
        }
    }

    /// view it as a buffer, may faile if it is not a buffer value
    pub fn as_buffer<const N: usize>(&self) -> NapiResult<JsBuffer<N>> {
        if self.is_buffer()? {
            Ok(JsBuffer::from_value(*self))
        } else {
            Err(NapiStatus::GenericFailure)
        }
    }

    /// view it as a dataview, may faile if it is not a dataview value
    pub fn as_dataview(&self) -> NapiResult<JsDataView> {
        if self.is_buffer()? {
            Ok(JsDataView::from_value(*self))
        } else {
            Err(NapiStatus::GenericFailure)
        }
    }

    /// check if it is an external
    pub fn is_external(&self) -> NapiResult<bool> {
        Ok(self.kind()? == NapiValuetype::External)
    }

    /// view it as an external, may fail if it is not an external value
    pub fn as_external<T>(&self) -> NapiResult<JsExternal<T>> {
        if self.is_external()? {
            Ok(JsExternal::from_value(*self))
        } else {
            Err(NapiStatus::GenericFailure)
        }
    }

    /// check if it is a function
    pub fn is_function(&self) -> NapiResult<bool> {
        Ok(self.kind()? == NapiValuetype::Function)
    }

    /// view it as a number, may fail if it is not a number value
    pub fn as_function(&self) -> NapiResult<JsFunction> {
        if self.is_function()? {
            Ok(JsFunction::from_value(*self))
        } else {
            Err(NapiStatus::FunctionExpected)
        }
    }

    /// check if it is a number
    pub fn is_number(&self) -> NapiResult<bool> {
        Ok(self.kind()? == NapiValuetype::Number)
    }

    /// view it as a number, may fail if it is not a number value
    pub fn as_number(&self) -> NapiResult<JsNumber> {
        if self.is_number()? {
            Ok(JsNumber::from_value(*self))
        } else {
            Err(NapiStatus::NumberExpected)
        }
    }

    /// check if it is a bigint
    pub fn is_bigint(&self) -> NapiResult<bool> {
        Ok(self.kind()? == NapiValuetype::Bigint)
    }

    /// view it as a bigint, may fail if it is not a bigint value
    pub fn as_bigint(&self) -> NapiResult<JsBigInt> {
        if self.is_bigint()? {
            Ok(JsBigInt::from_value(*self))
        } else {
            Err(NapiStatus::BigintExpected)
        }
    }

    /// check if it is a boolean
    pub fn is_boolean(&self) -> NapiResult<bool> {
        Ok(self.kind()? == NapiValuetype::Boolean)
    }

    /// view it as a boolean, may fail if it is not a boolean value
    pub fn as_boolean(&self) -> NapiResult<JsBoolean> {
        if self.is_boolean()? {
            Ok(JsBoolean::from_value(*self))
        } else {
            Err(NapiStatus::BooleanExpected)
        }
    }

    #[cfg(feature = "v5")]
    /// view it as a date, may fail if it is not a date value
    pub fn as_date(&self) -> NapiResult<JsDate> {
        if self.is_date()? {
            Ok(JsDate::from_value(*self))
        } else {
            Err(NapiStatus::DateExpected)
        }
    }

    pub fn is_null(&self) -> NapiResult<bool> {
        Ok(self.kind()? == NapiValuetype::Null)
    }

    pub fn is_undefined(&self) -> NapiResult<bool> {
        Ok(self.kind()? == NapiValuetype::Undefined)
    }

    /// This API checks if the Object passed in is a promise.
    pub fn is_promise(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_promise, self.env(), self.raw()))
    }
}

impl NapiValueT for JsValue {
    fn from_raw(env: NapiEnv, value: napi_value) -> JsValue {
        JsValue(env, value)
    }

    fn value(&self) -> JsValue {
        *self
    }
}

pub trait NapiValueT {
    /// construct value from raw pointer
    fn from_raw(env: NapiEnv, value: napi_value) -> Self;

    /// inner value
    fn value(&self) -> JsValue;

    /// napi_value type cast
    fn cast<T: NapiValueT>(&self) -> T {
        T::from_raw(self.env(), self.raw())
    }

    /// This API implements the abstract operation ToBoolean() as defined in Section 7.1.2 of the
    /// ECMAScript Language Specification.
    #[inline]
    fn coerce_to_bool(&self) -> NapiResult<JsBoolean> {
        Ok(JsBoolean::from_raw(
            self.env(),
            napi_call!(=napi_coerce_to_bool, self.env(), self.raw()),
        ))
    }

    /// This API implements the abstract operation ToNumber() as defined in Section 7.1.3 of the
    /// ECMAScript Language Specification. This function potentially runs JS code if the passed-in
    /// value is an object.
    #[inline]
    fn coerce_coerce_to_number(&self) -> NapiResult<JsNumber> {
        Ok(JsNumber::from_raw(
            self.env(),
            napi_call!(=napi_coerce_to_number, self.env(), self.raw()),
        ))
    }

    /// This API implements the abstract operation ToObject() as defined in Section 7.1.13 of the
    /// ECMAScript Language Specification.
    #[inline]
    fn coerce_to_object(&self) -> NapiResult<JsObject> {
        Ok(JsObject::from_raw(
            self.env(),
            napi_call!(=napi_coerce_to_object, self.env(), self.raw()),
        ))
    }

    /// This API implements the abstract operation ToString() as defined in Section 7.1.13 of the
    /// ECMAScript Language Specification. This function potentially runs JS code if the passed-in
    /// value is an object.
    #[inline]
    fn coerce_to_string(&self) -> NapiResult<JsString> {
        Ok(JsString::from_raw(
            self.env(),
            napi_call!(=napi_coerce_to_string, self.env(), self.raw()),
        ))
    }

    /// This API represents invoking the instanceof Operator on the object as defined in Section
    /// 12.10.4 of the ECMAScript Language Specification.
    fn instance_of(&self, constructor: JsFunction) -> NapiResult<bool> {
        Ok(napi_call!(=napi_instanceof, self.env(), self.raw(), constructor.raw()))
    }

    /// This API represents invoking the IsArray operation on the object as defined in Section
    /// 7.2.2 of the ECMAScript Language Specification.
    fn is_array(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_array, self.env(), self.raw()))
    }

    /// This API checks if the Object passed in is an array buffer.
    fn is_arraybuffer(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_arraybuffer, self.env(), self.raw()))
    }

    /// This API checks if the Object passed in is a buffer.
    fn is_buffer(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_buffer, self.env(), self.raw()))
    }

    #[cfg(feature = "v5")]
    /// This API checks if the Object passed in is a date.
    fn is_date(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_date, self.env(), self.raw()))
    }

    /// This API checks if the Object passed in is a error.
    fn is_error(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_error, self.env(), self.raw()))
    }

    /// This API checks if the Object passed in is a typed array.
    fn is_typedarray(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_typedarray, self.env(), self.raw()))
    }

    /// This API checks if the Object passed in is a DataView.
    fn is_dataview(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_dataview, self.env(), self.raw()))
    }

    /// This API represents the invocation of the Strict Equality algorithm as defined in
    /// Section 7.2.14 of the ECMAScript Language Specification.
    fn equals(&self, rhs: impl NapiValueT) -> NapiResult<bool> {
        Ok(napi_call!(=napi_strict_equals, self.env(), self.raw(), rhs.raw()))
    }

    /// Returns napi_ok if the API succeeded.
    /// - `napi_invalid_arg` if the type of value is not a known ECMAScript type and value is not an External value.
    /// This API represents behavior similar to invoking the typeof Operator on the object as defined in
    /// Section 12.5.5 of the ECMAScript Language Specification. However, there are some differences:
    /// It has support for detecting an External value.
    /// It detects null as a separate type, while ECMAScript typeof would detect object.
    /// If value has a type that is invalid, an error is returned.
    #[inline]
    fn kind(&self) -> NapiResult<NapiValuetype> {
        Ok(napi_call!(=napi_typeof, self.env(), self.raw()))
    }

    /// the `NapiEnv` of current value
    fn env(&self) -> NapiEnv {
        self.value().env()
    }

    /// the raw-handle of current value
    fn raw(&self) -> napi_value {
        self.value().raw()
    }

    /// get null singleton
    fn null(&self) -> NapiResult<JsNull> {
        JsNull::new(self.env())
    }

    /// get undefined singleton
    fn undefined(&self) -> NapiResult<JsUndefined> {
        JsUndefined::new(self.env())
    }

    /// get global singleton
    fn global(&self) -> NapiResult<JsGlobal> {
        JsGlobal::new(self.env())
    }

    /// value is throwable
    #[inline]
    fn throw(&self) -> NapiResult<()> {
        napi_call!(napi_throw, self.env(), self.raw());
        Ok(())
    }

    /// This method allows the efficient definition of multiple properties on a given object. The
    /// properties are defined using property descriptors (see napi_property_descriptor). Given an
    /// array of such property descriptors, this API will set the properties on the object one at a
    /// time, as defined by DefineOwnProperty() (described in Section 9.1.6 of the ECMA-262
    /// specification).
    fn define_properties<P>(&self, properties: P) -> NapiResult<()>
    where
        P: AsRef<[NapiPropertyDescriptor]>,
    {
        napi_call!(
            napi_define_properties,
            self.env(),
            self.raw(),
            properties.as_ref().len(),
            properties.as_ref().as_ptr() as *const _,
        );

        Ok(())
    }

    /// This is a hook which is fired when the value is gabage-collected.
    /// For napi >= 5, we use napi_add_finalizer,
    /// For napi < 5, we use napi_wrap.
    fn gc<Finalizer>(&mut self, finalizer: Finalizer) -> NapiResult<NapiRef>
    where
        Finalizer: FnOnce(NapiEnv) -> NapiResult<()>,
    {
        #[cfg(feature = "v5")]
        return self.finalizer(finalizer);
        #[cfg(not(feature = "v5"))]
        return self.wrap((), move |env, _| finalizer(env));
    }

    #[cfg(feature = "v5")]
    /// Adds a napi_finalize callback which will be called when the JavaScript object in
    /// js_object is ready for garbage collection. This API is similar to napi_wrap() except
    /// that:
    ///
    /// * the native data cannot be retrieved later using napi_unwrap(),
    /// * nor can it be removed later using napi_remove_wrap(), and
    /// * the API can be called multiple times with different data items in order to attach
    /// each of them to the JavaScript object, and
    /// * the object manipulated by the API can be used with napi_wrap().
    ///
    /// Caution: The optional returned reference (if obtained) should be deleted via
    /// napi_delete_reference ONLY in response to the finalize callback invocation.
    /// If it is deleted before then, then the finalize callback may never be invoked.
    /// herefore, when obtaining a reference a finalize callback is also required in order
    /// to enable correct disposal of the reference.
    fn finalizer<Finalizer>(&self, finalizer: Finalizer) -> NapiResult<NapiRef>
    where
        Finalizer: FnOnce(NapiEnv) -> NapiResult<()>,
    {
        // NB: Because we add a closure to the napi finalizer, it's better
        // to **CAPTURE** the leaked data from rust side, so here we just
        // ignore the passed in native data pointer.
        unsafe extern "C" fn finalizer_trampoline(
            env: NapiEnv,
            _: DataPointer,
            finalizer: DataPointer,
        ) {
            // NB: here we collect the memory of finalizer closure
            let finalizer: Box<Box<dyn FnOnce(NapiEnv) -> NapiResult<()>>> =
                Box::from_raw(finalizer as _);
            if let Err(err) = finalizer(env) {
                log::error!("NapiValueT::finalizer(): {}", err);
            }
        }

        let finalizer: Box<Box<dyn FnOnce(NapiEnv) -> NapiResult<()>>> =
            Box::new(Box::new(finalizer));
        let reference = napi_call!(
            =napi_add_finalizer,
            self.env(),
            self.raw(),
            std::ptr::null_mut(),
            Some(finalizer_trampoline),
            Box::into_raw(finalizer) as DataPointer,
        );

        Ok(NapiRef::from_raw(self.env(), reference))
    }

    #[allow(clippy::type_complexity)]
    /// Wraps a native instance in a JavaScript object. The native instance can be retrieved
    /// later using napi_unwrap().
    ///
    /// When JavaScript code invokes a constructor for a class that was defined using napi_define_class(),
    /// the napi_callback for the constructor is invoked. After constructing an instance of the native class,
    /// the callback must then call napi_wrap() to wrap the newly constructed instance in the already-created
    /// JavaScript object that is the this argument to the constructor callback. (That this object was
    /// created from the constructor function's prototype, so it already has definitions of all the instance
    /// properties and methods.)
    ///
    /// Typically when wrapping a class instance, a finalize callback should be provided that simply
    /// deletes the native instance that is received as the data argument to the finalize callback.
    ///
    /// The optional returned reference is initially a weak reference, meaning it has a reference
    /// count of 0. Typically this reference count would be incremented temporarily during async
    /// operations that require the instance to remain valid.
    ///
    /// Caution: The optional returned reference (if obtained) should be deleted via napi_delete_reference
    /// ONLY in response to the finalize callback invocation. If it is deleted before then, then
    /// the finalize callback may never be invoked. Therefore, when obtaining a reference a finalize
    /// callback is also required in order to enable correct disposal of the reference.
    ///
    /// Calling napi_wrap() a second time on an object will return an error. To associate another
    /// native instance with the object, use napi_remove_wrap() first.
    fn wrap<T, Finalizer>(&mut self, data: T, finalizer: Finalizer) -> NapiResult<NapiRef>
    where
        Finalizer: FnOnce(NapiEnv, T) -> NapiResult<()>,
    {
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
            let data = Box::<T>::from_raw(data as _);
            if let Err(err) = finalizer(env, *data) {
                log::error!("NapiValueT::wrap(): {}", err);
            }
        }

        let finalizer: Box<Box<dyn FnOnce(NapiEnv, T) -> NapiResult<()>>> =
            Box::new(Box::new(finalizer));
        let reference = napi_call!(
            =napi_wrap,
            self.env(),
            self.raw(),
            Box::into_raw(Box::new(data)) as DataPointer,
            Some(finalizer_trampoline::<T>),
            Box::into_raw(finalizer) as DataPointer,
        );

        Ok(NapiRef::from_raw(self.env(), reference))
    }

    /// Retrieves a native instance that was previously wrapped in a JavaScript object using
    /// napi_wrap().
    ///
    /// When JavaScript code invokes a method or property accessor on the class, the corresponding
    /// napi_callback is invoked. If the callback is for an instance method or accessor, then the
    /// this argument to the callback is the wrapper object; the wrapped C++ instance that is the
    /// target of the call can be obtained then by calling napi_unwrap() on the wrapper object.
    ///
    /// NB: if a there is no wrap or the wrap is just removed by NapiValue::remove_wrap, return
    /// None.
    fn unwrap<T>(&self) -> NapiResult<Option<&T>> {
        let (status, value) = napi_call!(?napi_unwrap, self.env(), self.raw());
        match status {
            NapiStatus::Ok => unsafe { Ok(Some(&*(value as *const T))) },
            NapiStatus::InvalidArg => Ok(None),
            err => Err(err),
        }
    }

    /// Retrieves a native instance that was previously wrapped in the JavaScript object js_object
    /// using napi_wrap() and removes the wrapping. If a finalize callback was associated with the
    /// wrapping, it will no longer be called when the JavaScript object becomes garbage-collected.
    fn remove_wrap<T>(&mut self) -> NapiResult<T> {
        let value = napi_call!(=napi_remove_wrap, self.env(), self.raw());
        unsafe {
            let value: Box<T> = Box::from_raw(value as *mut _);
            Ok(*value)
        }
    }

    #[cfg(feature = "v8")]
    /// Associates the value of the type_tag pointer with the JavaScript object.
    /// napi_check_object_type_tag() can then be used to compare the tag that was attached to the
    /// object with one owned by the addon to ensure that the object has the right type.
    /// If the object already has an associated type tag, this API will return napi_invalid_arg.
    fn type_tag_object(&self, tag: &NapiTypeTag) -> NapiResult<()> {
        napi_call!(napi_type_tag_object, self.env(), self.raw(), tag);
        Ok(())
    }

    #[cfg(feature = "v8")]
    /// Compares the pointer given as type_tag with any that can be found on js_object. If no tag
    /// is found on js_object or, if a tag is found but it does not match type_tag, then result is
    /// set to false. If a tag is found and it matches type_tag, then result is set to true.
    fn check_object_type_tag(&self, tag: &NapiTypeTag) -> NapiResult<bool> {
        Ok(napi_call!(=napi_check_object_type_tag, self.env(), self.raw(), tag))
    }
}

mod array;
mod arraybuffer;
mod bigint;
mod boolean;
mod buffer;
mod class;
mod dataview;
mod date;
mod error;
mod external;
mod function;
mod global;
mod name;
mod null;
mod number;
mod object;
mod promise;
mod typedarray;
mod undefined;

pub use array::JsArray;
pub use arraybuffer::JsArrayBuffer;
pub use bigint::JsBigInt;
pub use boolean::JsBoolean;
pub use buffer::JsBuffer;
pub use class::JsClass;
pub use dataview::JsDataView;
pub use date::JsDate;
pub use error::JsError;
pub use external::JsExternal;
pub use function::{Function, JsFunction};
pub use global::JsGlobal;
pub use name::{JsString, JsSymbol};
pub use null::JsNull;
pub use number::JsNumber;
pub use object::JsObject;
pub use promise::JsPromise;
pub use typedarray::JsTypedArray;
pub use undefined::JsUndefined;
