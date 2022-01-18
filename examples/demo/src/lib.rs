use nodex::prelude::*;

nodex::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    nodex::napi_guard!(env.napi_version()?);

    let mut obj = env.object()?;
    let mut times = 0;

    let label = "func";
    let name = env.string(label)?;
    let symbol = env.symbol()?;

    obj.set_property(
        name,
        env.func(move |this, [a1]: [JsFunction; 1]| {
            let env = this.env();

            let r = a1.call(this, [env.string("I am from rust world.")?]);
            let result = match r {
                Ok(result) => result,
                Err(_) => env.undefined()?.value(),
            };

            env.async_work_state(
                "my-test-async-task",
                0,
                move |idx| {
                    *idx += 1;
                },
                move |_, status, &mut idx| {
                    if status == NapiStatus::Cancelled {
                        println!("[{}] task cancelled", status);
                    } else {
                        assert_eq!(idx, 1);
                    }
                    Ok(())
                },
            )?
            .queue()?;

            times += 1;
            println!("[{}] called", times);
            Ok(result)
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
        move || {},
        move |env, status| {
            assert!(status.ok());
            env.async_work(
                label,
                move || {},
                move |_, status| {
                    assert!(status.ok());
                    Ok(())
                },
            )?
            .queue()?;
            Ok(())
        },
    )?
    .queue()?;

    Ok(())
}
