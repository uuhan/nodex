use nodex::prelude::*;
nodex::napi_module!(init);

fn init(env: NapiEnv, exports: JsObject) -> NapiResult<()> {
    nodex_plugin_helloworld::init(env, exports)?;

    Ok(())
}
