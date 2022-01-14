use nodex_api::{api, NapiPropertyAttributes, NapiStatus};
use std::{mem::MaybeUninit, os::raw::c_char};

nodex_api::init!(init);

fn init(env: api::napi_env, exports: api::napi_value) {
    unsafe {
        let name = std::ffi::CString::new("hello").unwrap();

        let value = {
            let mut result = MaybeUninit::uninit();
            let _ = api::napi_create_string_utf8(
                env,
                "world".as_ptr() as *const c_char,
                5,
                result.as_mut_ptr(),
            );

            result.assume_init()
        };

        let desc = api::napi_property_descriptor {
            utf8name: name.as_ptr(),
            name: std::ptr::null_mut(),
            method: None,
            getter: None,
            setter: None,
            value,
            attributes: NapiPropertyAttributes::Default.bits(),
            data: std::ptr::null_mut(),
        };
        let status = api::napi_define_properties(env, exports, 1, &desc);
        assert_eq!(status, NapiStatus::Ok as u32);
    }
}
