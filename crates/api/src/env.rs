use crate::api::napi_env;

#[derive(Clone, Copy)]
pub struct Env(napi_env);

impl AsRef<napi_env> for Env {
    fn as_ref(&self) -> &napi_env {
        &self.0
    }
}
