use nodex::prelude::*;

nodex::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
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
        env.func(move |this, a1: JsFunction| {
            let env = this.env();
            let _scope = env.handle_scope()?;
            a1.call(this, ())?;

            env.async_work(
                "my-test-async-task",
                0,
                move |idx| {
                    *idx += 1;
                },
                move |_, status, idx| {
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

            a1.call(this, env.string("I am from rust world.")?)
        })?,
    )?;
    obj.set_property(symbol, env.double(100.)?)?;
    assert_eq!(label, name.get()?);

    let class = env.class(
        "myclass",
        |mut this, a1: JsNumber| {
            this.set_named_property("a1", a1)?;
            Ok(this)
        },
        [DescriptorValueBuilder::new()
            .with_utf8name("prop1")
            .with_value(env.double(10.)?)
            .build()?],
    )?;

    obj.set_named_property("myclass", class)?;

    obj.set_named_property("instance", class.new_instance(env.double(100.)?)?)?;

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
        DescriptorValueBuilder::new()
            .with_utf8name("utils")
            .with_value(obj)
            .build()?,
        DescriptorValueBuilder::new()
            .with_utf8name("key1")
            .with_value(env.double(100.)?)
            .build()?,
    ])?;

    exports.set_named_property(
        "names",
        env.func(move |_, a1: JsObject| {
            let names = a1.get_property_names()?;
            println!("len: {}", names.len()?);
            Ok(names)
        })?,
    )?;

    let label = "my-task-async-work";

    env.async_work(
        label,
        (),
        move |_| {},
        move |env, status, _| {
            assert!(status.ok());
            let _: JsUndefined = env.run_script("console.log('async work complete')")?;
            Ok(())
        },
    )?
    .queue()?;

    exports.set_named_property(
        "delay",
        env.func(move |_, cb: Function<JsUndefined>| {
            let tsfn: NapiTsfn<()> = env.tsfn(
                "delay-callback",
                cb,
                move |_| Ok(()),
                move |cb, _| {
                    cb.call(env.object()?, ())?;
                    Ok(())
                },
            )?;
            env.async_work(
                "delay-async-work",
                (),
                move |_| {
                    println!("delay async work executing...");
                    std::thread::sleep(std::time::Duration::from_secs(1));
                },
                move |_, _, _| {
                    println!("delay async work complete");
                    tsfn.blocking(())?;
                    tsfn.release()?;
                    Ok(())
                },
            )?
            .queue()?;
            env.undefined()
        })?,
    )?;

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
    let _callback = context.make_callback(exports, env.func(move |this, ()| Ok(this))?, ())?;

    if let Some(_hook) = env.add_async_cleanup_hook(|hook| hook.remove())? {}

    exports.set_named_property(
        "thread",
        env.func(move |this, a1: JsFunction| {
            let env = this.env();
            let tsfn = NapiTsfn::<_>::new(
                env,
                "tsfn-context",
                a1,
                move |_| Ok(()),
                move |f, data: String| {
                    f.call(env.object()?, env.string(&data)?)?;
                    Ok(())
                },
            )?;

            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs(1));
                tsfn.non_blocking("hello, world - 1".into()).unwrap();

                std::thread::sleep(std::time::Duration::from_secs(1));
                tsfn.non_blocking("hello, world - 2".into()).unwrap();

                tsfn.release().unwrap();
            });

            this.undefined()
        })?,
    )?;

    env.set_instance_data(100usize, |_, value| {
        println!("drop instance data: {}", value);
        Ok(())
    })?;

    let value = env.get_instance_data::<usize>()?;
    println!("get instance data: {:?}", value);
    if let Ok(Some(data)) = env.get_instance_data::<usize>() {
        *data = 200;
    }
    let value = env.get_instance_data::<usize>()?;
    println!("get instance data: {:?}", value);

    exports.set_named_property(
        "buffer_index",
        env.func(|this, a1: JsValue| {
            let a1 = a1.as_buffer::<5>()?;
            this.env().double(a1[0] as f64)
        })?,
    )?;

    let external = env.external("ext data".into(), |_, _: String| Ok(()))?;
    assert_eq!("ext data", external.get()?);

    let ext_buffer: JsBuffer<10> = env.buffer_copy([10; 10])?;
    exports.set_named_property("buffer", ext_buffer)?;

    Ok(())
}
