use nodex::prelude::*;

pub fn init(env: NapiEnv, mut object: JsObject) -> NapiResult<()> {
    object.set_named_property(
        "hello_world",
        env.func(|this, []: Args<0>| {
            let env = this.env();
            let res: JsValue = env.run_script(
                r#"
            console.log("hello, nodex!");
        "#,
            )?;
            Ok(res)
        })?,
    )?;

    Ok(())
}
