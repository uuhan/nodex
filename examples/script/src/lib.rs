use nodex::prelude::*;
nodex::napi_module!(init);

fn init(env: NapiEnv, _: JsObject) -> NapiResult<()> {
    let func: Function<JsUndefined> = env.run_script(
        r#"
            function hello() {
                console.log(this);
            }

            hello
        "#,
    )?;

    func.call::<JsValue, 0>(env.global()?.cast(), [])?;

    Ok(())
}
