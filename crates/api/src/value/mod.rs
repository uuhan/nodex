use crate::api::napi_value;

#[derive(Clone, Debug)]
pub struct Value(napi_value);

impl AsRef<napi_value> for Value {
    fn as_ref(&self) -> &napi_value {
        &self.0
    }
}

