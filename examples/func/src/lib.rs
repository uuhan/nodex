use nodex::prelude::*;
nodex::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    exports.set_named_property(
        "func",
        JsFunction::new(
            env,
            Option::<String>::None,
            |this, (a, _): (JsString, JsNumber)| {
                println!("{}", a.get()?);
                this.undefined()
            },
        )?,
    )?;

    Ok(())
}
