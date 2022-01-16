use nodex_api::prelude::*;

nodex_api::init!(init);

fn init(env: NapiEnv, exports: JsValue) -> NapiResult<()> {
    let mut obj = env.object()?;
    let mut times = 0;

    let label = "func";

    // env.context("my-async-context")?;

    let name = env.string(label)?;
    obj.set_property(
        name,
        JsFunction::with(env, "func", move || {
            times += 1;
            println!("[{}] called", times);
        })?,
    )?;

    assert_eq!(label, name.get()?);

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

    exports.define_properties(&[
        DescriptorBuilder::new()
            .with_name(env.string("utils")?)
            .with_value(obj)
            .build()?,
        DescriptorBuilder::new()
            .with_name(env.string("key1")?)
            .with_value(env.double(100.)?)
            .build()?,
    ])?;

    Ok(())
}
