## Nodex - Nodejs eXtension 🥳

Yet another crate to create native nodejs addons :)

This crate aims to make creating native nodejs addons very easy and comfortable.

It is in a very early stage and heavy development is making.

## Features

- [x] ergonomical api design.
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

[dependencies.nodex-api]
version = "0.1.0-alpha.10"
features = ["v8"]
```

The default napi version is set to v1, you can use other version with your need.

We have v1,v2,v3,...v8 versions.

**Currently, nodex just exports nodex-api:**

```toml
[lib]
crate-type = ["cdylib"]

[dependencies.nodex]
version = "0.1.0-alpha.10"
```

## Examples

### Init Module

simply define your module by:

```rust
use nodex_api::prelude::*;
nodex_api::napi_module!(init);
fn init(env: NapiEnv, exports: JsObject) -> NapiResult<()> {
    Ok(())
}
```

### Version Guard

make sure the node api version is large or equal than your compiled addon's.

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
let func: JsFunction = env.func(move |this, [a1, a2, a3]: [JsValue; _]| {
    let env = this.env();
    let r = a1.as_function()?.call(this, [env.string("I am from rust world.")?.value()])?;
    Ok(r)
})?;

let func: JsFunction = env.func(move |this, [a1]: [JsFunction; _]| {
    let env = this.env();
    let r = a1.call(this, [env.string("I am from rust world.")?.value()])?;
    Ok(r)
})?;

// Error
let error: JsError = JsError::error("error", None)?;

```

### Napi handle scope

```rust
// napi handle scope
let _scope: NapiHandleScope = env.handle_scope()?;
let _escapable_scope: NapiEscapableHandleScope = env.escapable_handle_scope()?;
```

### Napi cleanup hook

#### sync

```rust
env.add_cleanup_hook(|| {
    println!("clean hook fired");
    Ok(())
})?;

let hook_to_remove = env.add_cleanup_hook(|| {
    println!("clean hook fired");
    Ok(())
})?;

hook_to_remove.remove()?;
```

#### aync

```rust
match env.add_async_cleanup_hook(|hook| {
    // DO SOME CLEANUP
    // NB: should call remove after done
    hook.remove()
})? {
    Some(hook) => {
        // NB: also the hook can be removed before it is fired.
        hook.remove()?;
    }
    None => {}
}
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
    move || {
        // you can do the hard work in the thread-pool context.
        // NB: js work is not allowed here.
        println!("execute async task");
    },
    move |_, status| {
        // you can do some js work in this context
        println!("[{}] complete async task", status);
        Ok(())
    },
)?
.queue()?;

// with shared state
env.async_work_state(
    "my-test-async-task",
    0,
    move |idx| {
        *idx += 1;
        println!("execute async task");
    },
    move |_, status, idx| {
        println!("[{}] complete async task: {}", status, idx);
        Ok(())
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
