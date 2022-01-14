use crate::{api::napi_value, env::Env};

#[derive(Clone, Copy, Debug)]
pub struct JsValue<'a> {
    env: Env<'a>,
    value: napi_value,
}

impl<'a> JsValue<'a> {
    /// create `JsValue` from raw napi_value
    pub fn from_raw(env: Env<'a>, value: napi_value) -> JsValue<'a> {
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

pub use string::JsString;
