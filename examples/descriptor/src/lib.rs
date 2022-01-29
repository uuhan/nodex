use nodex::prelude::*;
nodex::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    let obj = env.object()?;

    obj.define_properties(&[DescriptorValueBuilder::new()
        .with_utf8name("myvalue")
        .with_value(env.string("myvalue")?)
        .build()?])?;

    obj.define_properties(&[DescriptorMethodBuilder::new()
        .with_utf8name("mymethod")
        .with_method(move |this, []: [JsValue; 0]| this.env().double(200.))
        .build()?])?;

    obj.define_properties(&[DescriptorAccessorBuilder::new()
        .with_utf8name("myaccessor")
        .with_getter(|this| this.env().double(100.))
        .with_setter(|_this: JsObject, [n]: [JsNumber; 1]| {
            println!("setter: {}", n.get_value_int32()?);
            Ok(())
        })
        .build()?])?;

    exports.set("obj", obj)?;

    Ok(())
}
