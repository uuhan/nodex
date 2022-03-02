use nodex::prelude::*;
nodex::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    exports.set_named_property(
        "create",
        env.func(|this, resolve: JsBoolean| {
            let env = this.env();
            let resolve = resolve.get()?;
            let promise: JsPromise<JsString, JsError> = env.promise(
                move |result| {
                    for i in 1..=3 {
                        std::thread::sleep(std::time::Duration::from_secs(1));
                        println!("[{}] Doing...", i);
                    }

                    *result = resolve;
                },
                move |promise, _, result| {
                    let env = promise.env();
                    if result {
                        promise.resolve(env.string("the promise is resolved.")?)?;
                    } else {
                        promise.reject(env.error("the promise is rejected.")?)?;
                    }
                    Ok(())
                },
            )?;

            Ok(promise.value())
        })?,
    )?;

    Ok(())
}
