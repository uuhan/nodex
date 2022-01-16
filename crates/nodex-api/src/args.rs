use crate::{api, env::NapiEnv, prelude::*};

/// Js args
#[derive(Debug, Clone)]
pub struct JsArgs(pub(crate) Vec<JsValue>);

/// Trait for types convertible to any number of Js values.
pub trait ToJsArgs {
    fn to_js_args(self) -> NapiResult<JsArgs>;
}

/// Trait for types that can be created from an arbitrary number of Js values.
pub trait FromJsArgs: Sized {
    fn from_js_args(args: JsArgs) -> NapiResult<Self>;
}

impl ToJsArgs for bool {
    fn to_js_args(self) -> NapiResult<JsArgs> {
        todo!()
    }
}
