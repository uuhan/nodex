use crate::{api, prelude::*};
use std::{mem::MaybeUninit, os::raw::c_char};

#[derive(Copy, Clone, Debug)]
pub struct JsBuffer<const N: usize>(pub(crate) JsValue);

impl<const N: usize> JsBuffer<N> {
    pub(crate) fn from_value(value: JsValue) -> JsBuffer<N> {
        JsBuffer(value)
    }

    /// This API allocates a node::Buffer object. While this is still a fully-supported data
    /// structure, in most cases using a TypedArray will suffice.
    pub fn create(env: NapiEnv) -> NapiResult<JsBuffer<N>> {
        let buffer = napi_call!(=napi_create_buffer, env, N, std::ptr::null_mut());
        Ok(JsBuffer::<N>::from_raw(env, buffer))
    }

    /// This API allocates a node::Buffer object and initializes it with data copied from the
    /// passed-in buffer. While this is still a fully-supported data structure, in most cases using
    /// a TypedArray will suffice.
    pub fn create_copy(env: NapiEnv, data: impl AsRef<[u8]>) -> NapiResult<JsBuffer<N>> {
        let buffer = napi_call!(
            =napi_create_buffer_copy,
            env,
            N,
            data.as_ref().as_ptr() as _,
            std::ptr::null_mut(),
        );
        Ok(JsBuffer::from_raw(env, buffer))
    }

    // pub fn create_external(
    //     env: NapiEnv,
    //     data: impl AsRef<[u8]>,
    //     finalizer: impl FnOnce(NapiEnv, [u8; N]) -> NapiResult<()> + 'static,
    // ) -> NapiResult<JsBuffer<N>>
    // {
    //     type FnOnceBoxed<const N: usize> = Box<dyn FnOnce(NapiEnv, [u8; N]) -> NapiResult<()>>;
    //
    //     unsafe extern "C" fn finalize<const N: usize>(
    //         env: NapiEnv,
    //         data: DataPointer,
    //         hint: DataPointer,
    //     ) {
    //         let ext: [u8; N] = *(data as *const [u8; N]);
    //         let finalizer: Box<FnOnceBoxed::<N>> = Box::from_raw(hint as _);
    //         if let Err(e) = finalizer(env, ext) {
    //             log::error!("JsExternal::<T>::finalize: {}", e);
    //         }
    //         std::ptr::drop_in_place(data as *mut [u8; N]);
    //     }
    //
    //     let finalizer: Box<FnOnceBoxed::<N>> = Box::new(Box::new(finalizer));
    //     let data = std::mem::ManuallyDrop::new([10; 10]);
    //
    //     let buffer = napi_call!(
    //         =napi_create_external_buffer,
    //         env,
    //         N,
    //         data.as_ptr() as DataPointer,
    //         Some(finalize::<N>),
    //         Box::into_raw(finalizer) as DataPointer,
    //     );
    //
    //     Ok(JsBuffer::from_raw(env, buffer))
    // }

    /// Get the underlaying array
    pub fn get(&self) -> NapiResult<&[u8]> {
        let mut data = MaybeUninit::uninit();
        let length = napi_call!(=napi_get_buffer_info, self.env(), self.raw(), data.as_mut_ptr());
        if length != N {
            return Err(NapiStatus::InvalidArg);
        }

        unsafe {
            let data = data.assume_init();
            Ok(std::slice::from_raw_parts(data as *mut u8, N))
        }
    }

    /// Get the underlaying array (mut)
    pub fn get_mut(&mut self) -> NapiResult<&mut [u8]> {
        let mut data = MaybeUninit::uninit();
        let length = napi_call!(=napi_get_buffer_info, self.env(), self.raw(), data.as_mut_ptr());
        if length != N {
            return Err(NapiStatus::InvalidArg);
        }

        unsafe {
            let data = data.assume_init();
            Ok(std::slice::from_raw_parts_mut(data as *mut u8, N))
        }
    }

    /// The length of current buffer.
    pub const fn len(&self) -> usize {
        N
    }

    /// The buffer is empty
    pub const fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<const N: usize> NapiValueT for JsBuffer<N> {
    fn from_raw(env: NapiEnv, raw: napi_value) -> JsBuffer<N> {
        JsBuffer(JsValue(env, raw))
    }

    fn value(&self) -> JsValue {
        self.0
    }
}

impl<const N: usize> std::ops::Index<usize> for JsBuffer<N> {
    type Output = u8;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.get().unwrap()[idx]
    }
}
