## It is an example to show how to write a nodex-plugin

Just add nodex as the dependency:

```toml
[package]
name = "nodex-plugin-helloworld"
version = "0.1.1"
edition = "2021"

[dependencies]
nodex = "^0.2"
```

Then export your own function with the signature:

[lib.rs](./src/lib.rs)

```rust
use nodex::prelude::*;

pub fn init(env: NapiEnv, mut object: JsObject) -> NapiResult<()> {
    object.set_named_property(
        "hello_world",
        env.func(|this, ()| {
            let env = this.env();
            let res: JsValue = env.run_script(
                r#"
                    console.log("hello, nodex!");
                "#
            )?;
            Ok(res)
        })?,
    )?;

    Ok(())
}
```

So you can use this crate as the dependency of your nodex project:

[plugin example](https://github.com/uuhan/nodex/tree/master/examples)

```toml
[package]
name = "plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies.nodex]
features = ["v8"]
version = "^0.1"

[dependencies.nodex-plugin-helloworld]
version = "0.1"
```

lib.rs

```rust
use nodex::prelude::*;
nodex::napi_module!(init);

fn init(env: NapiEnv, exports: JsObject) -> NapiResult<()> {
    nodex_plugin_helloworld::init(env, exports)?;

    Ok(())
}
```
