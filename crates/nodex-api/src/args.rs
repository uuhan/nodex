use crate::{api, env::NapiEnv, prelude::*};

/// Js args
#[derive(Debug, Clone)]
pub struct JsArgs(pub Vec<JsValue>);

/// Trait for types convertible to any number of Js values.
pub trait ToJsArgs {
    fn to_js_args(self, env: NapiEnv) -> NapiResult<JsArgs>;
    fn len() -> usize;
}

/// Trait for types that can be created from an arbitrary number of Js values.
pub trait FromJsArgs: Sized {
    fn from_js_args(args: JsArgs) -> NapiResult<Self>;
    fn len() -> usize;
}

impl FromJsArgs for () {
    fn from_js_args(args: JsArgs) -> NapiResult<()> {
        Ok(())
    }

    fn len() -> usize {
        0
    }
}

impl ToJsArgs for () {
    fn to_js_args(self, env: NapiEnv) -> NapiResult<JsArgs> {
        Ok(JsArgs(vec![]))
    }

    fn len() -> usize {
        0
    }
}

impl<T: NapiValueT> FromJsArgs for T {
    fn from_js_args(args: JsArgs) -> NapiResult<T> {
        // It's safe here
        let arg = unsafe { args.0.get_unchecked(0) };
        let casted = arg.cast::<T>();
        match casted.check() {
            Ok(true) => Ok(casted),
            Ok(false) => Err(NapiStatus::InvalidArg),
            Err(e) => Err(e),
        }
    }

    fn len() -> usize {
        1
    }
}

impl<T: NapiValueT> ToJsArgs for T {
    fn to_js_args(self, env: NapiEnv) -> NapiResult<JsArgs> {
        Ok(JsArgs(vec![self.value()]))
    }

    fn len() -> usize {
        1
    }
}

macro_rules! count {
    () => (0_usize);
    ($x:tt $($xs:tt)*) => (1_usize + count!($($xs)*));
}

#[macro_export]
macro_rules! from_js_args_tuple {
    () => ();

    ($($name:ident),+; $($idx:tt),+) => (
        #[doc(hidden)]
        impl<$($name: NapiValueT + NapiValueCheck),*> FromJsArgs for ($($name,)*) {
            fn from_js_args(args: JsArgs) -> NapiResult<Self> {
                Ok(($({
                    // It's safe here
                    let arg = unsafe { args.0.get_unchecked($idx) };
                    let casted = arg.cast::<$name>();
                    match casted.check() {
                        Ok(true) => {
                            casted
                        }
                        Ok(false) => {
                            return Err(NapiStatus::InvalidArg)
                        }
                        Err(e) => {
                            return Err(e)
                        }
                    }
                },)*))
            }

            fn len() -> usize {
                count!($($name)*)
            }
        }
    )
}

from_js_args_tuple!(T0; 0);
from_js_args_tuple!(T0, T1; 0, 1);
from_js_args_tuple!(T0, T1, T2; 0, 1, 2);
from_js_args_tuple!(T0, T1, T2, T3; 0, 1, 2, 3);
from_js_args_tuple!(T0, T1, T2, T3, T4; 0, 1, 2, 3, 4);
from_js_args_tuple!(T0, T1, T2, T3, T4, T5; 0, 1, 2, 3, 4, 5);
from_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6; 0, 1, 2, 3, 4, 5, 6);
from_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7; 0, 1, 2, 3, 4, 5, 6, 6);
from_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8; 0, 1, 2, 3, 4, 5, 6, 7, 8);
from_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
from_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
from_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
from_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
from_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
from_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
from_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

#[macro_export]
macro_rules! to_js_args_tuple {
    () => ();

    ($($name:ident),+; $($idx:tt),+) => (
        #[doc(hidden)]
        impl<$($name: NapiValueT + NapiValueCheck),+> ToJsArgs for ($($name,)+) {
            fn to_js_args(self, env: NapiEnv) -> NapiResult<JsArgs> {
                let mut args = vec![];
                let mut idx = 0;

                $({
                    args.push(self.$idx.value());
                })+

                Ok(JsArgs(args))
            }

            fn len() -> usize {
                count!($($name)*)
            }
        }
    )
}

to_js_args_tuple!(T0; 0);
to_js_args_tuple!(T0, T1; 0, 1);
to_js_args_tuple!(T0, T1, T2; 0, 1, 2);
to_js_args_tuple!(T0, T1, T2, T3; 0, 1, 2, 3);
to_js_args_tuple!(T0, T1, T2, T3, T4; 0, 1, 2, 3, 4);
to_js_args_tuple!(T0, T1, T2, T3, T4, T5; 0, 1, 2, 3, 4, 5);
to_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6; 0, 1, 2, 3, 4, 5, 6);
to_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7; 0, 1, 2, 3, 4, 5, 6, 6);
to_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8; 0, 1, 2, 3, 4, 5, 6, 7, 8);
to_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
to_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
to_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
to_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
to_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
to_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
to_js_args_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
