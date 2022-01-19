use nodex::prelude::*;

nodex::napi_module!(init);

fn init(mut env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
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
            let _scope = env.handle_scope()?;
            let result = a1.call(this, [env.string("I am from rust world.")?]);

            let result = match result {
                Ok(result) => result,
                Err(_) => return Ok(env.undefined()?.value()),
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

    env.add_cleanup_hook(|| {
        println!("clean hook fired");
        Ok(())
    })?;

    let hook = env.add_cleanup_hook(|| {
        println!("clean hook fired");
        Ok(())
    })?;

    hook.remove()?;

    if let Some(_hook) = env.add_async_cleanup_hook(|hook| hook.remove())? {}

    Ok(())
}
