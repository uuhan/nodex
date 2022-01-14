use nodex_api::{api, prelude::*};

nodex_api::init!(init);

fn init(env: Env, exports: JsValue) -> NapiResult<()> {
    unsafe {
        let name = std::ffi::CString::new("hello").unwrap();

        let mut obj = JsObject::new(env)?;
        let _value = JsString::new(env, "world")?;

        obj.set(JsString::new(env, "a")?, JsString::new(env, "b")?)?;

        let desc = api::napi_property_descriptor {
            utf8name: name.as_ptr(),
            name: std::ptr::null_mut(),
            method: None,
            getter: None,
            setter: None,
            value: obj.raw(),
            attributes: NapiPropertyAttributes::Default.bits(),
            data: std::ptr::null_mut(),
        };

        let status = api::napi_define_properties(env.raw(), exports.raw(), 1, &desc);
        assert_eq!(status, NapiStatus::Ok);

        Ok(())
    }
}
