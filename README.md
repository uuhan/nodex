## Nodex - Nodejs eXtension 🥳

Yet another crate to create native nodejs addons :)

This crate aims to make creating native nodejs addons very easy and comfortable.

It is in a very early stage and heavy development is making.

## Features

- [ ] ergonomical api design.
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
nodex-api = "0.1.0-alpha.6"
```

The default napi version is set to v1, you can use other version with your need.

## Examples

### Init Module

simply define you module by:

```rust
use nodex_api::prelude::*;
nodex_api::napi_module!(init);
fn init(env: NapiEnv, exports: JsObject) -> NapiResult<()> {
    Ok(())
}
```

### Version Guard

check if the node api version is not less than your addon:

```rust
nodex_api::napi_guard!(env.napi_version()?);
```

### Nodejs Version & Napi Version

get the runtime version:

```rust
let node_version = env.node_version()?;
let napi_version = env.napi_version()?;
```

### Define Js Variable

```rust
// String & Symbol
let label: JsSymbol = env.symbol()?;
let name: JsString = env.string("")?;

// Object
let mut obj: JsObject = env.object()?;
obj.set_property(name, env.null()?)?;

// Function
let func: JsFunction = env.func(move |this| {
    this.undefined()
})?;
```

### Set Property Descriptor

```rust
let mut obj: JsObject = env.object()?;
obj.define_properties(&[
    DescriptorBuilder::new()
        .with_name(env.string("utils")?)
        .with_value(env.double(100.)?)
        .build()?,
])?;
```

### Create An Async Work

```rust
// without shared state
env.async_work(
    env,
    "my-test-async-task",
    move |_| {
        println!("execute async task");
    },
    move |_, status| {
        println!("[{}] complete async task", status);
    },
)?
.queue()?;

// with shared state
env.async_work_state(
    "my-test-async-task",
    0,
    move |_, idx| {
        *idx += 1;
        println!("execute async task");
    },
    move |_, status, idx| {
        println!("[{}] complete async task: {}", status, idx);
    },
)?
.queue()?;
```

### More

[examples/demo](./examples/demo)

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
