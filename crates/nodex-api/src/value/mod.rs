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

    /// check if it is an arraybuffer
    pub fn is_arraybuffer(&self) -> NapiResult<bool> {
        todo!()
    }

    /// view it as an array_buffer, may faile if it is not an array_buffer value
    pub fn as_arraybuffer(&self) -> NapiResult<JsArrayBuffer> {
        todo!()
    }

    pub fn is_null(&self) -> NapiResult<bool> {
        Ok(self.value_type()? == NapiValuetype::Null)
    }

    pub fn is_undefined(&self) -> NapiResult<bool> {
        Ok(self.value_type()? == NapiValuetype::Undefined)
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
pub use null::JsNull;
pub use object::JsObject;
pub use string::JsString;
pub use undefined::JsUndefined;
