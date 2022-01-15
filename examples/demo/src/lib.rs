use nodex_api::{api, prelude::*};

nodex_api::init!(init);

fn init(env: NapiEnv, exports: JsValue) -> NapiResult<()> {
    let name = std::ffi::CString::new("hello").unwrap();

    let mut obj = JsObject::new(env)?;
    let _value = JsString::new(env, "world")?;

    obj.set(
        JsString::new(env, "a")?,
        JsFunction::with(env, "b", || println!("called"))?,
    )?;

    // let version = env.node_version()?;
    //
    // println!(
    //     "{}.{}.{}-{} {}",
    //     version.major,
    //     version.minor,
    //     version.patch,
    //     std::ffi::CStr::from_ptr(version.release).to_str().unwrap(),
    //     env.napi_version()?,
    // );

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

    let status = unsafe { api::napi_define_properties(env.raw(), exports.raw(), 1, &desc) };
    assert_eq!(status, NapiStatus::Ok);

    Ok(())
}
