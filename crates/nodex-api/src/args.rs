use crate::{api, env::NapiEnv, prelude::*};

/// Js args
#[derive(Debug, Clone)]
pub struct JsArgs<'a>(pub(crate) Vec<JsValue<'a>>);

/// Trait for types convertible to any number of Js values.
pub trait ToJsArgs<'a> {
    fn to_js_args(self) -> NapiResult<JsArgs<'a>>;
}

/// Trait for types that can be created from an arbitrary number of Js values.
pub trait FromJsArgs<'a>: Sized {
    fn from_js_args(args: JsArgs<'a>) -> NapiResult<Self>;
}

impl<'a> ToJsArgs<'a> for bool {
    fn to_js_args(self) -> NapiResult<JsArgs<'a>> {
        todo!()
    }
}
