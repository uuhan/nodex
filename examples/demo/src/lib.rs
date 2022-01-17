use nodex_api::prelude::*;

nodex_api::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    nodex_api::napi_guard!(env.napi_version()?);

    let mut obj = env.object()?;
    let mut times = 0;

    let label = "func";

    // env.context("my-async-context")?;

    let name = env.string(label)?;
    let symbol = env.symbol()?;

    obj.set_property(
        name,
        env.func(move |this| {
            times += 1;
            println!("[{}] called", times);
            this.value()
        })?,
    )?;

    obj.set_property(symbol, env.double(100.)?)?;

    assert_eq!(label, name.get()?);

    let version = env.node_version()?;
    println!(
        "{}.{}.{}-{} {}",
        version.major,
        version.minor,
        version.patch,
        unsafe { std::ffi::CStr::from_ptr(version.release).to_str().unwrap() },
        env.napi_version()?,
    );

    exports.set_named_property("a", env.string("b")?)?;

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
