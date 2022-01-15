use nodex_api::{api, prelude::*};

nodex_api::init!(init);

fn init(env: NapiEnv, exports: JsValue) -> NapiResult<()> {
    let mut obj = JsObject::new(env)?;
    let mut times = 0;

    obj.set(
        JsString::new(env, "func")?,
        JsFunction::with(env, "func", move || {
            times += 1;
            println!("[{}] called", times);
        })?,
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

    let desc = DescriptorBuilder::new()
        .with_name(JsString::new(env, "utils")?)
        .with_value(obj)
        .build()
        .unwrap();

    let status = unsafe { api::napi_define_properties(env.raw(), exports.raw(), 1, desc.raw()) };
    assert_eq!(status, NapiStatus::Ok);

    Ok(())
}
