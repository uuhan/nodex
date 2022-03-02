use nodex::prelude::*;

nodex::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    exports.set("string", env.string("napi string")?)?;
    exports.set("symbol", env.symbol()?)?;
    exports.set("array", env.array()?)?;
    exports.set("arraybuffer", env.arraybuffer([1, 2, 3, 4, 5])?)?;
    exports.set("bigint_int64", env.bigint_i64(i64::MAX)?)?;
    exports.set("bigint_unt64", env.bigint_u64(u64::MAX)?)?;
    exports.set("boolean_true", env.boolean(true)?)?;
    exports.set("boolean_false", env.boolean(false)?)?;
    exports.set("buffer_10", env.buffer::<10>()?)?;
    exports.set("buffer_copy_10", env.buffer_copy([0; 10])?)?;

    let arraybuffer = env.arraybuffer([0; 10])?;
    exports.set_named_property("arraybuffer_10", arraybuffer)?;
    exports.set_named_property("dataview_3_5", arraybuffer.view(3, 5)?)?;

    exports.set_named_property("date", env.date(1000000000.)?)?;
    exports.set_named_property("error", env.error("error")?)?;
    exports.set_named_property("external", env.external(100, move |_, _| Ok(()))?)?;
    exports.set_named_property(
        "function",
        env.func(move |this, ()| this.env().undefined())?,
    )?;
    exports.set_named_property("global", env.global()?)?;
    exports.set_named_property("null", env.null()?)?;
    exports.set_named_property("undefined", env.undefined()?)?;
    exports.set_named_property("int32", env.int32(100)?)?;
    exports.set_named_property("uint32", env.uint32(100)?)?;
    exports.set_named_property("int64", env.int64(100)?)?;
    exports.set_named_property("double", env.double(100.)?)?;
    exports.set_named_property("object", env.object()?)?;
    exports.set_named_property(
        "promise",
        env.promise(|_| {}, |_: JsPromise<JsValue, JsValue>, _, _: ()| Ok(()))?
            .value(),
    )?;
    exports.set_named_property(
        "typedarray",
        arraybuffer.typedarray(NapiTypedarrayType::Uint8Array, 0, 5)?,
    )?;

    Ok(())
}
