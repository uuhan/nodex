/// This addon shows how to use `nodex::NapiThreadsafeFunction`.
use nodex::prelude::*;
nodex::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    exports.set_named_property(
        "create",
        env.func(|this, [callback]: ArgsT<Function<JsString>, 1>| {
            let env = this.env();
            let tsfn: NapiTsfn<&str, 0> = env.tsfn(
                "thread-safe-function",
                callback,
                move |_| Ok(()),
                move |callback, data| {
                    let env = callback.env();
                    callback.call(env.object()?, [env.string(data)?])?;
                    Ok(())
                },
            )?;

            std::thread::spawn(move || {
                for i in 1..=3 {
                    println!("[{}] Doing...", i);
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                tsfn.blocking("NapiThreadsafeFunction Calls Back.").unwrap();
                tsfn.release().unwrap();
            })
            .join()
            .unwrap();

            this.env().undefined()
        })?,
    )?;

    Ok(())
}
