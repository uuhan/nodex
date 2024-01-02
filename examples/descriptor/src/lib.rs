use nodex::prelude::*;
use std::sync::{Arc, Mutex};
nodex::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    let obj = env.object()?;

    obj.define_properties([DescriptorValueBuilder::new()
        .with_utf8name("myvalue")
        .with_value(env.string("myvalue")?)
        .build()?])?;

    obj.define_properties([DescriptorMethodBuilder::new()
        .with_utf8name("mymethod")
        .with_method(move |this, ()| this.env().double(200.))
        .build()?])?;

    let value = Arc::new(Mutex::new(0.));
    let value2 = value.clone();

    obj.define_properties([DescriptorAccessorBuilder::new()
        .with_utf8name("myaccessor")
        .with_getter(move |this| this.env().double(*value.lock().unwrap()))
        .with_setter(move |_this: JsObject, n: JsNumber| {
            let mut value = value2.lock().unwrap();
            *value = n.get_value_double()?;
            Ok(())
        })
        .build()?])?;

    exports.set("obj", obj)?;

    Ok(())
}
