use nodex::prelude::*;
nodex::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    exports.set_named_property(
        "func",
        env.func(|_, [obj, cb, n, m]: [JsValue; 4]| {
            let context = env.context("async-context")?;
            cb.as_function()?.call(obj.as_object()?, [n, m])?;
            context.make_callback(obj.cast(), cb.as_function()?, [n, m])
        })?,
    )?;

    Ok(())
}
