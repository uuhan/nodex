use nodex::prelude::*;
nodex::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    exports.set_named_property(
        "create",
        env.func(|this, [resolve]: ArgsT<JsBoolean, 1>| {
            let env = this.env();
            let resolve = resolve.get()?;
            let promise: JsPromise<JsString, JsError> = JsPromise::new(env)?;

            env.async_work(
                "test-async-work",
                (),
                move |_| {
                    for i in 1..=3 {
                        std::thread::sleep(std::time::Duration::from_secs(1));
                        println!("[{}] Doing...", i);
                    }
                },
                move |env, _, _| {
                    if resolve {
                        promise.resolve(env.string("the promise is resolved.")?)?;
                    } else {
                        promise.reject(env.error("the promise is rejected.")?)?;
                    }
                    Ok(())
                },
            )?
            .queue()?;

            Ok(promise.value())
        })?,
    )?;

    Ok(())
}
