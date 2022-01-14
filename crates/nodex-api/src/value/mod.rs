use crate::{api::napi_value, env::Env};

#[derive(Clone, Copy, Debug)]
pub struct Value<'a> {
    env: Env<'a>,
    value: napi_value,
}

impl<'a> Value<'a> {
    /// create `Value` from raw napi_value
    pub fn from_raw(env: Env<'a>, value: napi_value) -> Value<'a> {
        Value { env, value }
    }

    /// `Env` of this `Value`
    pub fn env(&self) -> Env<'a> {
        self.env
    }

    /// raw napi_value of this `Value`
    pub fn raw(&self) -> napi_value {
        self.value
    }
}
