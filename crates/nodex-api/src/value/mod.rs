use crate::{api, env::NapiEnv, prelude::*};
use std::mem::MaybeUninit;

#[derive(Clone, Copy, Debug)]
pub struct JsValue<'a> {
    env: NapiEnv<'a>,
    value: napi_value,
}

impl<'a> JsValue<'a> {
    /// create `JsValue` from raw napi_value
    pub fn from_raw(env: NapiEnv<'a>, value: napi_value) -> JsValue<'a> {
        JsValue { env, value }
    }

    /// `NapiEnv` of this `JsValue`
    pub fn env(&self) -> NapiEnv<'a> {
        self.env
    }

    /// raw napi_value of this `JsValue`
    pub fn raw(&self) -> napi_value {
        self.value
    }

    /// get the value type of the js value
    pub fn value_type(&self) -> NapiResult<NapiValuetype> {
        let value = unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_typeof(self.env().raw(), self.raw(), result.as_mut_ptr());
            if status.err() {
                return Err(status);
            }

            result.assume_init()
        };

        Ok(value)
    }

    /// check if it is an object
    pub fn is_object(&self) -> NapiResult<bool> {
        Ok(self.value_type()? == NapiValuetype::Object)
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
        Ok(self.value_type()? == NapiValuetype::String)
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
        Ok(self.value_type()? == NapiValuetype::Symbol)
    }

    /// view it as a symbol, may fail if it is not a symbol value
    pub fn as_symbol(&self) -> NapiResult<JsSymbol> {
        if self.is_symbol()? {
            Ok(JsSymbol::from_value(*self))
        } else {
            Err(NapiStatus::GenericFailure)
        }
    }

    /// check if it is an array
    pub fn is_array(&self) -> NapiResult<bool> {
        unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_is_array(self.env().raw(), self.raw(), result.as_mut_ptr());
            if status.err() {
                return Err(NapiStatus::ArrayExpected);
            }

            Ok(result.assume_init())
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

    /// check if it is a typed_array
    pub fn is_typedarray(&self) -> NapiResult<bool> {
        unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_is_typedarray(self.env().raw(), self.raw(), result.as_mut_ptr());
            if status.err() {
                return Err(NapiStatus::GenericFailure);
            }

            Ok(result.assume_init())
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

    /// check if it is an arraybuffer
    pub fn is_arraybuffer(&self) -> NapiResult<bool> {
        unsafe {
            let mut result = MaybeUninit::uninit();
            let status =
                api::napi_is_arraybuffer(self.env().raw(), self.raw(), result.as_mut_ptr());
            if status.err() {
                return Err(NapiStatus::ArraybufferExpected);
            }

            Ok(result.assume_init())
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

    /// check if it is a buffer
    pub fn is_buffer(&self) -> NapiResult<bool> {
        unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_is_buffer(self.env().raw(), self.raw(), result.as_mut_ptr());
            if status.err() {
                return Err(NapiStatus::GenericFailure);
            }

            Ok(result.assume_init())
        }
    }

    /// view it as a buffer, may faile if it is not a buffer value
    pub fn as_buffer(&self) -> NapiResult<JsBuffer> {
        if self.is_buffer()? {
            Ok(JsBuffer::from_value(*self))
        } else {
            Err(NapiStatus::GenericFailure)
        }
    }

    /// check if it is a dataview
    pub fn is_dataview(&self) -> NapiResult<bool> {
        unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_is_dataview(self.env().raw(), self.raw(), result.as_mut_ptr());
            if status.err() {
                return Err(NapiStatus::GenericFailure);
            }

            Ok(result.assume_init())
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
        Ok(self.value_type()? == NapiValuetype::External)
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
        Ok(self.value_type()? == NapiValuetype::Function)
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
        Ok(self.value_type()? == NapiValuetype::Number)
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
        Ok(self.value_type()? == NapiValuetype::Bigint)
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
        Ok(self.value_type()? == NapiValuetype::Boolean)
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
    /// check if it is a date
    pub fn is_date(&self) -> NapiResult<bool> {
        unsafe {
            let mut result = MaybeUninit::uninit();
            let status = api::napi_is_date(self.env().raw(), self.raw(), result.as_mut_ptr());
            if status.err() {
                return Err(NapiStatus::DateExpected);
            }

            Ok(result.assume_init())
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
        Ok(self.value_type()? == NapiValuetype::Null)
    }

    pub fn is_undefined(&self) -> NapiResult<bool> {
        Ok(self.value_type()? == NapiValuetype::Undefined)
    }
}

impl<'a> ValueInner for JsValue<'a> {
    fn downcast(&self) -> JsValue {
        *self
    }
}

pub trait ValueInner {
    /// downcast to inner `JsValue` type
    fn downcast(&self) -> JsValue;

    /// the `NapiEnv` of current value
    fn env(&self) -> NapiEnv {
        self.downcast().env()
    }

    /// the raw-handle of current value
    fn raw(&self) -> napi_value {
        self.downcast().raw()
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
mod null;
mod number;
mod object;
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
pub use null::JsNull;
pub use number::JsNumber;
pub use object::JsObject;
pub use string::JsString;
pub use symbol::JsSymbol;
pub use typedarray::JsTypedArray;
pub use undefined::JsUndefined;
