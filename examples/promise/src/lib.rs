use nodex::prelude::*;
nodex::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    exports.set_named_property(
        "create",
        env.func(|this, []: Args::<0>| {
            let env = this.env();
            let promise: JsPromise<JsString, JsError> = JsPromise::new(env)?;

            env.async_work(
                "test-async-work",
                (),
                move |_| {
                    for i in 1..=5 {
                        std::thread::sleep(std::time::Duration::from_secs(1));
                        println!("[{}] Doing...", i);
                    }
                },
                move |env, _, _| {
                    promise.resolve(env.string("the promise is resolved.")?)?;
                    Ok(())
                }
            )?.queue()?;

            Ok(promise.value())
        })?,
    )?;

    Ok(())
}
