use nodex::prelude::*;

nodex::napi_module!(init);

fn init(mut env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    nodex::napi_guard!(env.napi_version()?);

    let mut obj = env.object()?;
    let mut times = 0;

    obj.gc(move |_| {
        println!("obj garbage-collected");
        Ok(())
    })?;

    obj.wrap([1usize, 2], move |_, v| {
        println!("wrap: {:?}", v);
        Ok(())
    })?;

    println!("unwrap: {:?}", obj.unwrap::<[usize; 2]>());
    println!("remove wrap: {:?}", obj.remove_wrap::<[usize; 2]>());

    let label = "func";
    let name = env.string(label)?;
    let symbol = env.symbol()?;

    obj.set_property(
        name,
        env.func(move |this, [a1]: [JsFunction; 1]| {
            let env = this.env();
            let _scope = env.handle_scope()?;
            a1.call::<JsValue, 0>(this, [])?;

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

            a1.call(this, [env.string("I am from rust world.")?])
        })?,
    )?;
    obj.set_property(symbol, env.double(100.)?)?;
    assert_eq!(label, name.get()?);

    let class = env.class(
        "myclass",
        |mut this, [a1]: [JsNumber; 1]| {
            this.set_named_property("a1", a1)?;
            Ok(this)
        },
        [DescriptorBuilder::new()
            .with_utf8name("prop1")
            .with_value(env.double(10.)?)
            .build()?],
    )?;

    obj.set_named_property("myclass", class)?;

    obj.set_named_property("instance", class.new_instance::<JsValue>(&[])?)?;

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

    exports.set_named_property(
        "names",
        env.func(move |_, [a1]: [JsObject; 1]| {
            let names = a1.get_property_names()?;
            println!("len: {}", names.len()?);
            Ok(names)
        })?,
    )?;

    let label = "my-task-async-work";

    env.async_work(
        label,
        move || {
            for i in 1..=5 {
                println!("async work executing: {}", i);
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        },
        move |env, status| {
            assert!(status.ok());
            println!("async work complete");
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

    let context = NapiAsyncContext::new(env, "my-async-context")?;
    let _callback = context.callback(
        exports,
        env.func(move |this, []: [JsValue; 0]| Ok(this))?,
        [env.undefined()?],
    )?;

    if let Some(_hook) = env.add_async_cleanup_hook(|hook| hook.remove())? {}

    let tsfn = NapiThreadsafeFunction::new(
        env,
        env.func(|this, []: [JsValue; 0]| {
            println!("tsfn called");
            this.env().undefined()
        })?,
        (),
        move |_| {
            println!("destroyed");
            Ok(())
        },
    )?;

    tsfn.call(
        std::ptr::null_mut(),
        NapiThreadsafeFunctionCallMode::Nonblocking,
    )?;

    Ok(())
}
