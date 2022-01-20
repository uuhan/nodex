use crate::{api, prelude::*};

#[derive(Clone, Debug)]
pub struct NapiRef(NapiEnv, napi_ref);

impl NapiRef {
    pub(crate) fn from_raw(env: NapiEnv, reference: napi_ref) -> NapiRef {
        NapiRef(env, reference)
    }

    /// This API creates a new reference with the specified reference count to the Object passed
    /// in.
    pub fn new<T: NapiValueT>(value: T, count: u32) -> NapiResult<NapiRef> {
        let reference = napi_call!(
            =napi_create_reference,
            value.env(),
            value.raw(),
            count,
        );

        Ok(NapiRef(value.env(), reference))
    }

    /// This API increments the reference count for the reference passed in and returns the
    /// resulting reference count.
    pub fn inc(&mut self) -> NapiResult<u32> {
        Ok(napi_call!(=napi_reference_ref, self.0, self.1))
    }

    /// This API decrements the reference count for the reference passed in and returns the
    /// resulting reference count.
    pub fn dec(&mut self) -> NapiResult<u32> {
        Ok(napi_call!(=napi_reference_unref, self.0, self.1))
    }

    /// If still valid, this API returns the napi_value representing the JavaScript Object
    /// associated with the napi_ref. Otherwise, result will be NULL.
    pub fn deref<T: NapiValueT>(&self) -> NapiResult<T> {
        let value = napi_call!(=napi_get_reference_value, self.0, self.1);
        Ok(T::from_raw(self.0, value))
    }
}

impl Drop for NapiRef {
    fn drop(&mut self) {
        unsafe {
            api::napi_delete_reference(self.0, self.1);
        }
    }
}
