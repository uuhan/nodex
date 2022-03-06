use crate::{api, env::NapiEnv, prelude::*};
use std::mem::MaybeUninit;

#[repr(C)]
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

    pub fn is_object(&self) -> NapiResult<bool> {
        napi_is!(self, JsObject)
    }

    /// view it as an object, may fail if it is not an object value
    pub fn as_object(&self) -> NapiResult<JsObject> {
        napi_as!(self, JsObject, NapiStatus::ObjectExpected)
    }

    pub fn is_string(&self) -> NapiResult<bool> {
        napi_is!(self, JsString)
    }

    /// view it as a string, may fail if it is not a string value
    pub fn as_string(&self) -> NapiResult<JsString> {
        napi_as!(self, JsString, NapiStatus::StringExpected)
    }

    pub fn is_symbol(&self) -> NapiResult<bool> {
        napi_is!(self, JsSymbol)
    }

    /// view it as a symbol, may fail if it is not a symbol value
    pub fn as_symbol(&self) -> NapiResult<JsSymbol> {
        napi_as!(self, JsSymbol, NapiStatus::InvalidArg)
    }

    pub fn is_array(&self) -> NapiResult<bool> {
        napi_is!(self, JsArray)
    }

    /// view it as an array, may fail if it is not an array value
    pub fn as_array(&self) -> NapiResult<JsArray> {
        napi_as!(self, JsArray, NapiStatus::ArrayExpected)
    }

    pub fn is_typedarray(&self) -> NapiResult<bool> {
        napi_is!(self, JsTypedArray)
    }
    /// view it as a typed_array, may fail if it is not a typed_array value
    pub fn as_typedarray(&self) -> NapiResult<JsTypedArray> {
        napi_as!(self, JsTypedArray, NapiStatus::InvalidArg)
    }

    pub fn is_arraybuffer(&self) -> NapiResult<bool> {
        napi_is!(self, JsArrayBuffer)
    }

    /// view it as an array_buffer, may fail if it is not an array_buffer value
    pub fn as_arraybuffer(&self) -> NapiResult<JsArrayBuffer> {
        napi_as!(self, JsArrayBuffer, NapiStatus::ArraybufferExpected)
    }

    pub fn is_buffer<const N: usize>(&self) -> NapiResult<bool> {
        napi_is!(self, JsBuffer<N>)
    }

    /// view it as a buffer, may fail if it is not a buffer value
    pub fn as_buffer<const N: usize>(&self) -> NapiResult<JsBuffer<N>> {
        napi_as!(self, JsBuffer<N>, NapiStatus::InvalidArg)
    }

    pub fn is_dataview(&self) -> NapiResult<bool> {
        napi_is!(self, JsDataView)
    }

    /// view it as a dataview, may fail if it is not a dataview value
    pub fn as_dataview(&self) -> NapiResult<JsDataView> {
        napi_as!(self, JsDataView, NapiStatus::InvalidArg)
    }

    pub fn is_external<T>(&self) -> NapiResult<bool> {
        napi_is!(self, JsExternal<T>)
    }

    /// view it as an external, may fail if it is not an external value
    pub fn as_external<T>(&self) -> NapiResult<JsExternal<T>> {
        napi_as!(self, JsExternal<T>, NapiStatus::InvalidArg)
    }

    pub fn is_function(&self) -> NapiResult<bool> {
        napi_is!(self, JsFunction)
    }

    /// view it as a number, may fail if it is not a number value
    pub fn as_function(&self) -> NapiResult<JsFunction> {
        napi_as!(self, JsFunction, NapiStatus::FunctionExpected)
    }

    pub fn is_number(&self) -> NapiResult<bool> {
        napi_is!(self, JsNumber)
    }

    /// view it as a number, may fail if it is not a number value
    pub fn as_number(&self) -> NapiResult<JsNumber> {
        napi_as!(self, JsNumber, NapiStatus::NumberExpected)
    }

    pub fn is_bigint<T>(&self) -> NapiResult<bool> {
        napi_is!(self, JsBigInt<T>)
    }

    /// view it as a bigint, may fail if it is not a bigint value
    pub fn as_bigint<T>(&self) -> NapiResult<JsBigInt<T>> {
        napi_as!(self, JsBigInt<T>, NapiStatus::BigintExpected)
    }

    pub fn is_boolean(&self) -> NapiResult<bool> {
        napi_is!(self, JsBoolean)
    }

    /// view it as a boolean, may fail if it is not a boolean value
    pub fn as_boolean(&self) -> NapiResult<JsBoolean> {
        napi_as!(self, JsBoolean, NapiStatus::BooleanExpected)
    }

    #[cfg(feature = "v5")]
    pub fn is_date(&self) -> NapiResult<bool> {
        napi_is!(self, JsDate)
    }

    #[cfg(feature = "v5")]
    /// view it as a date, may fail if it is not a date value
    pub fn as_date(&self) -> NapiResult<JsDate> {
        napi_as!(self, JsDate, NapiStatus::DateExpected)
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

impl NapiValueCheck for JsValue {
    fn check(&self) -> NapiResult<bool> {
        Ok(true)
    }
}

pub trait NapiValueCheck {
    fn check(&self) -> NapiResult<bool>;
}

/// The trait for js value, which just store napi_value raw pointer.
pub trait NapiValueT: NapiValueCheck + Sized {
    /// construct value from raw pointer
    fn from_raw(env: NapiEnv, value: napi_value) -> Self;

    /// inner value
    fn value(&self) -> JsValue;

    /// napi_value type cast
    ///
    /// ## Safety
    ///
    /// It just put the handle in new type, does not check the real type.
    #[inline]
    unsafe fn cast<T: NapiValueT>(&self) -> T {
        T::from_raw(self.env(), self.raw())
    }

    /// Upcast to specified value
    #[inline]
    fn cast_checked<T: NapiValueT>(&self) -> NapiResult<T> {
        if unsafe { self.cast::<T>() }.check()? {
            Ok(T::from_raw(self.env(), self.raw()))
        } else {
            Err(NapiStatus::InvalidArg)
        }
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

    /// This API represents the invocation of the Strict Equality algorithm as defined in
    /// Section 7.2.14 of the ECMAScript Language Specification.
    fn equals(&self, rhs: impl NapiValueT) -> NapiResult<bool> {
        Ok(napi_call!(=napi_strict_equals, self.env(), self.raw(), rhs.raw()))
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
        napi_call!(napi_throw, self.env(), self.raw())
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
        )
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
    fn unwrap<T>(&self) -> NapiResult<Option<&mut T>> {
        let (status, value) = napi_call!(?napi_unwrap, self.env(), self.raw());
        match status {
            NapiStatus::Ok => unsafe { Ok(Some(&mut *(value as *mut T))) },
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
        napi_call!(napi_type_tag_object, self.env(), self.raw(), tag)
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
