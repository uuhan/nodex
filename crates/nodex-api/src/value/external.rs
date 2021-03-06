use crate::{api, prelude::*};
use std::{marker::PhantomData, mem::MaybeUninit, os::raw::c_void};

#[derive(Copy, Clone, Debug)]
pub struct JsExternal<T>(pub(crate) JsValue, PhantomData<T>);

impl<T> JsExternal<T> {
    pub(crate) fn from_value(value: JsValue) -> JsExternal<T> {
        JsExternal(value, PhantomData)
    }

    /// This API allocates a JavaScript value with external data attached to it. This is used to
    /// pass external data through JavaScript code, so it can be retrieved later by native code
    /// using napi_get_value_external.
    pub fn new(
        env: NapiEnv,
        value: T,
        finalizer: impl FnOnce(NapiEnv, T) -> NapiResult<()> + 'static,
    ) -> NapiResult<JsExternal<T>> {
        type FnOnceBoxed<T> = Box<dyn FnOnce(NapiEnv, T) -> NapiResult<()>>;
        // NB: first leak value.
        let value = Box::into_raw(Box::new(value));

        unsafe extern "C" fn finalize<T>(env: NapiEnv, data: DataPointer, hint: DataPointer) {
            let ext: Box<T> = Box::from_raw(data as *mut T);
            let finalizer: Box<FnOnceBoxed<T>> = Box::from_raw(hint as _);
            if let Err(e) = finalizer(env, *ext) {
                log::error!("JsExternal::<T>::finalize: {}", e);
            }
        }

        let finalizer: Box<FnOnceBoxed<T>> = Box::new(Box::new(finalizer));

        let value = napi_call!(
            =napi_create_external,
            env,
            value as *mut c_void,
            Some(finalize::<T>),
            Box::into_raw(finalizer) as DataPointer,
        );

        Ok(JsExternal(JsValue::from_raw(env, value), PhantomData))
    }

    /// Access the underlaying data.
    pub fn get(&self) -> NapiResult<&mut T> {
        let ext = napi_call!(=napi_get_value_external, self.env(), self.raw());
        unsafe { Ok(&mut *(ext as *mut T)) }
    }

    /// This API returns a Node-API value corresponding to a JavaScript ArrayBuffer. The underlying byte buffer of the ArrayBuffer is externally allocated and managed. The caller must ensure that the byte buffer remains valid until the finalize callback is called.
    /// The API adds a napi_finalize callback which will be called when the JavaScript object just created is ready for garbage collection. It is similar to napi_wrap() except that:
    /// - the native data cannot be retrieved later using napi_unwrap(),
    /// - nor can it be removed later using napi_remove_wrap(), and
    /// - the object created by the API can be used with napi_wrap().
    /// JavaScript ArrayBuffers are described in Section 24.1 of the ECMAScript Language Specification.
    pub fn arraybuffer<'a>(
        env: NapiEnv,
        value: impl AsRef<[T]>,
    ) -> NapiResult<JsExternal<&'a [T]>> {
        todo!()
    }

    /// This API allocates a node::Buffer object and initializes it with data backed by the passed in buffer. While this is still a fully-supported data structure, in most cases using a TypedArray will suffice.
    /// The API adds a napi_finalize callback which will be called when the JavaScript object just created is ready for garbage collection. It is similar to napi_wrap() except that:
    /// - the native data cannot be retrieved later using napi_unwrap(),
    /// - nor can it be removed later using napi_remove_wrap(), and
    /// - the object created by the API can be used with napi_wrap().
    /// For Node.js >=4 Buffers are Uint8Arrays.
    pub fn buffer<'a>(env: NapiEnv, value: impl AsRef<[T]>) -> NapiResult<JsExternal<&'a [T]>> {
        todo!()
    }
}

impl<T> NapiValueT for JsExternal<T> {
    fn from_raw(env: NapiEnv, raw: napi_value) -> JsExternal<T> {
        JsExternal(JsValue(env, raw), PhantomData)
    }

    fn value(&self) -> JsValue {
        self.0
    }
}

impl<T> NapiValueCheck for JsExternal<T> {
    fn check(&self) -> NapiResult<bool> {
        Ok(self.kind()? == NapiValuetype::External)
    }
}
