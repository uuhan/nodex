use nodex_api::{api, prelude::*};

nodex_api::init!(init);

fn init(env: api::napi_env, exports: api::napi_value) {
    unsafe {
        let env = Env::from_raw(env);
        let name = std::ffi::CString::new("hello").unwrap();

        let value = JsString::new(env, "world").unwrap();

        let desc = api::napi_property_descriptor {
            utf8name: name.as_ptr(),
            name: std::ptr::null_mut(),
            method: None,
            getter: None,
            setter: None,
            value: value.raw(),
            attributes: NapiPropertyAttributes::Default.bits(),
            data: std::ptr::null_mut(),
        };
        let status = api::napi_define_properties(env.raw(), exports, 1, &desc);
        assert_eq!(status, NapiStatus::Ok);
    }
}
