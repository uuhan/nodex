use nodex_api::prelude::*;

nodex_api::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    nodex_api::napi_guard!(env.napi_version()?);

    let mut obj = env.object()?;
    let mut times = 0;

    let label = "func";
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
            .with_utf8name("utils")
            .with_value(obj)
            .build()?,
        DescriptorBuilder::new()
            .with_utf8name("key1")
            .with_value(env.double(100.)?)
            .build()?,
    ])?;

    let label = "my-task-async-work";

    env.async_work(
        label,
        move || {
            println!("execute async task1: {:?}", env.undefined().unwrap());
        },
        move |env, status| {
            env.async_work(
                label,
                move || {
                    println!("execute async task2");
                },
                move |env2, status| {
                    println!("[{}] complete async task2: {:?} {:?}", status, env, env2);
                },
            )
            .unwrap()
            .queue()
            .unwrap();
            println!("[{}] complete async task1", status);
        },
    )?
    .queue()?;

    let mut work = env.async_work_state(
        "my-test-async-task",
        0,
        move |idx| {
            *idx += 1;
            println!("execute async task");
        },
        move |_, status, idx| {
            if status == NapiStatus::Cancelled {
                println!("[{}] task cancelled", status);
            } else {
                println!("[{}] complete async task: {}", status, idx);
            }
        },
    )?;

    work.queue()?;

    Ok(())
}
