## Nodex - Nodejs eXtension 🥳

Yet another crate to create native nodejs addons :)

This crate aims to make creating native nodejs addons very easy and comfortable.

It is in a very early stage and heavy development is making.

## Features

- [ ] good napi wrappings.
- [ ] export the codebase from crates world, make it easy to call rust function from js world.
    - [ ] sweet syntax, like: #[nodex::function] fn foo()
- [ ] import the huge codebase from npm world, make it easy to call js function from rust side.
    - [ ] sweet syntax, like: let lodash = nodex::import!(lodash);
- [ ] nodejs async runtime to drive rust async code
    - [ ] async runtime for async rust
    - [ ] macros like: #[nodex::rt] async fn main()
- [ ] cargo-nodex cargo subcommand to make ease of create nodejs addons, e.g. auto generate ts typings.
    - [ ] cargo nodex build
    - [ ] cargo nodex typings
    - [ ] cargo nodex package

## Usage

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
nodex-api = "0.1.0-alpha.3"
```

## Examples

### Init Module

```rust
// lib.rs
use nodex_api::{api, prelude::*};

nodex_api::napi_module!(init);

fn init(env: NapiEnv, exports: JsValue) -> NapiResult<()> {
    Ok(())
}
```

### Demo

[lib.rs](./examples/demo/src/lib.rs)

```rust
use nodex_api::prelude::*;

nodex_api::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    let mut obj = env.object()?;
    let mut times = 0;

    let label = "func";

    // env.context("my-async-context")?;

    let name = env.string(label)?;
    let symbol = env.symbol()?;

    obj.set_property(
        name,
        env.func(move || {
            times += 1;
            println!("[{}] called", times);
        })?,
    )?;

    obj.set_property(symbol, env.double(100.)?)?;

    assert_eq!(label, name.get()?);

    // let version = env.node_version()?;
    //
    // println!(
    //     "{}.{}.{}-{} {}",
    //     version.major,
    //     version.minor,
    //     version.patch,
    //     std::ffi::CStr::from_ptr(version.release).to_str().unwrap(),
    //     env.napi_version()?,
    // );
    exports.set_property(env.string("a")?, env.string("b")?)?;

    exports.define_properties(&[
        DescriptorBuilder::new()
            .with_name(env.string("utils")?)
            .with_value(obj)
            .build()?,
        DescriptorBuilder::new()
            .with_name(env.string("key1")?)
            .with_value(env.double(100.)?)
            .build()?,
    ])?;

    Ok(())
}
```

Run:

```bash
bash demo.sh

# output
# [1] called
# { func: [Function (anonymous)], [Symbol()]: 100 }
# [2] called
# { func: [Function (anonymous)], [Symbol()]: 100 }
# 100
```

## How to participate in

## Code of conduct

```bash
cat >> .git/hooks/pre-push << EOF
#!/bin/sh

cargo fmt || exit
cargo clippy -- -D warnings || exit
EOF
```

## License

Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
