use crate::{api, env::Env, prelude::*};
use std::mem::MaybeUninit;

#[derive(Clone, Copy, Debug)]
pub struct JsValue<'a> {
    env: Env<'a>,
    value: napi_value,
}

impl<'a> JsValue<'a> {
    /// create `JsValue` from raw napi_value
    pub(crate) fn from_raw(env: Env<'a>, value: napi_value) -> JsValue<'a> {
        JsValue { env, value }
    }

    /// `Env` of this `JsValue`
    pub fn env(&self) -> Env<'a> {
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
}

pub trait ValueInner {
    /// downcast to inner `JsValue` type
    fn downcast(&self) -> JsValue;

    /// the `Env` of current value
    fn env(&self) -> Env {
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

pub use null::JsNull;
pub use object::JsObject;
pub use string::JsString;
pub use undefined::JsUndefined;
