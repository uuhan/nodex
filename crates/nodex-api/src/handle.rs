use crate::{api, prelude::*};

#[derive(Copy, Clone, Debug)]
pub struct NapiHandleScope<'a> {
    env: NapiEnv<'a>,
    handle: napi_handle_scope,
}

impl<'a> NapiHandleScope<'a> {
    pub(crate) fn from_value(env: NapiEnv<'a>, handle: napi_handle_scope) -> NapiHandleScope {
        NapiHandleScope { env, handle }
    }

    pub fn env(&self) -> NapiEnv<'a> {
        self.env
    }

    pub fn raw(&self) -> napi_handle_scope {
        self.handle
    }
}
