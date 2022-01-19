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

    /// This API represents invoking the IsArray operation on the object as defined in Section
    /// 7.2.2 of the ECMAScript Language Specification.
    pub fn is_array(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_array, self.env(), self.raw()))
    }

    /// view it as an array, may fail if it is not an array value
    pub fn as_array(&self) -> NapiResult<JsArray> {
        if self.is_array()? {
            Ok(JsArray::from_value(*self))
        } else {
            Err(NapiStatus::ArrayExpected)
        }
    }

    /// This API checks if the Object passed in is a typed array.
    pub fn is_typedarray(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_typedarray, self.env(), self.raw()))
    }

    /// view it as a typed_array, may fail if it is not a typed_array value
    pub fn as_typedarray(&self) -> NapiResult<JsTypedArray> {
        if self.is_typedarray()? {
            Ok(JsTypedArray::from_value(*self))
        } else {
            Err(NapiStatus::GenericFailure)
        }
    }

    /// This API checks if the Object passed in is an array buffer.
    pub fn is_arraybuffer(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_arraybuffer, self.env(), self.raw()))
    }

    /// view it as an array_buffer, may faile if it is not an array_buffer value
    pub fn as_arraybuffer(&self) -> NapiResult<JsArrayBuffer> {
        if self.is_arraybuffer()? {
            Ok(JsArrayBuffer::from_value(*self))
        } else {
            Err(NapiStatus::ArraybufferExpected)
        }
    }

    /// This API checks if the Object passed in is a buffer.
    pub fn is_buffer(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_buffer, self.env(), self.raw()))
    }

    /// view it as a buffer, may faile if it is not a buffer value
    pub fn as_buffer(&self) -> NapiResult<JsBuffer> {
        if self.is_buffer()? {
            Ok(JsBuffer::from_value(*self))
        } else {
            Err(NapiStatus::GenericFailure)
        }
    }

    /// This API checks if the Object passed in is a DataView.
    pub fn is_dataview(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_dataview, self.env(), self.raw()))
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
    /// This API checks if the Object passed in is a date.
    pub fn is_date(&self) -> NapiResult<bool> {
        Ok(napi_call!(=napi_is_date, self.env(), self.raw()))
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

    /// Returns napi_ok if the API succeeded.
    /// - `napi_invalid_arg` if the type of value is not a known ECMAScript type and value is not an External value.
    /// This API represents behavior similar to invoking the typeof Operator on the object as defined in Section 12.5.5 of the ECMAScript Language Specification. However, there are some differences:
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
    fn define_properties(
        &self,
        properties: impl AsRef<[NapiPropertyDescriptor]>,
    ) -> NapiResult<()> {
        napi_call!(
            napi_define_properties,
            self.env(),
            self.raw(),
            properties.as_ref().len(),
            properties.as_ref().as_ptr() as *const _,
        );

        Ok(())
    }
}

mod array;
mod arraybuffer;
mod bigint;
mod boolean;
mod buffer;
mod dataview;
mod date;
mod external;
mod function;
mod global;
mod null;
mod number;
mod object;
mod promise;
mod string;
mod symbol;
mod typedarray;
mod undefined;

pub use array::JsArray;
pub use arraybuffer::JsArrayBuffer;
pub use bigint::JsBigInt;
pub use boolean::JsBoolean;
pub use buffer::JsBuffer;
pub use dataview::JsDataView;
pub use date::JsDate;
pub use external::JsExternal;
pub use function::JsFunction;
pub use global::JsGlobal;
pub use null::JsNull;
pub use number::JsNumber;
pub use object::JsObject;
pub use promise::JsPromise;
pub use string::JsString;
pub use symbol::JsSymbol;
pub use typedarray::JsTypedArray;
pub use undefined::JsUndefined;
