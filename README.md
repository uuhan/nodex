## Nodex - Nodejs eXtension 🥳

Yet another crate to create native nodejs addons :)

This crate aims to make creating native nodejs addons very easy and comfortable.

[click here: uuhan/nodex@dev](https://github.com/uuhan/nodex) to see the most recently developments.

## Features

- [x] ergonomical api design.
- [ ] export the codebase from crates world, make it easy to call rust function from js world.
    - [ ] sweet syntax, like: #[nodex::function] fn foo()
- [ ] import the huge codebase from npm world, make it easy to call js function from rust side.
    - [ ] sweet syntax, like: let lodash = nodex::import!(lodash);
- [ ] nodejs async runtime to drive rust async code
    - [ ] async runtime for async rust
    - [ ] macros like: #[nodex::rt] async fn main(), so you can use nodejs to run any rust async-code.
        - [ ] node --require=main.node
        - [ ] rust code introspection with nodejs repl
- [ ] cargo-nodex cargo subcommand to make ease of create nodejs addons, e.g. auto generate ts typings.
    - [ ] cargo nodex build
    - [ ] cargo nodex typings
    - [ ] cargo nodex package

## Usage

```toml
[lib]
crate-type = ["cdylib"]

[dependencies.nodex-api]
version = "0.1.0-beta.1"
features = ["v8"]
```

The default napi version is set to v1, you can use other version with your need.

We have v1,v2,v3,...v8 versions.

**Currently, nodex just reexports nodex-api:**

```toml
[lib]
crate-type = ["cdylib"]

[dependencies.nodex]
version = "0.1.0-beta.1"
```

## Napi Level

### v1

* NapiValueT::wrap::\<T, Finalizer>() - Wraps a native instance, call finalizer when value is garbage-collected.
* NapiValueT::remove_wrap::\<T>() - Remove the wrapped native instance. The finalizer will not be called if the wrapped instance is removed.
* NapiValueT::unwrap::\<T>() - Access the wrapped instance.
* NapiValueT::gc::\<Finalizer>() - Hook fired when value is gabage-collected.

### v3

* NapiEnv::add_cleanup_hook() - Do the cleanup when nodejs environment exits.

### v4

* NapiThreadsafeFunction::\<Data> - Thread safe function.

### v5

* NapiValueT::finalizer() - Adds a napi_finalize callback which will be called when the JavaScript object is ready for gc.

### v6

* NapiEnv::set_instance_data::\<Data, Finalizer> - Set data to current agent.
* NapiENv::get_instance_data::\<Data> - Get Option\<&mut Data> from current agent.

### v8

* NapiEnv::add_async_cleanup_hook() - Do the cleanup when nodejs environment exits, asynchronous.

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
let func: JsFunction = env.func(move |this, [a1, a2, a3]: [JsValue; 3]| {
    let env = this.env();
    a1.as_function()?.call::<JsValue, 0>(this, [])?;
    a1.as_function()?.call(this, [env.string("I am from rust world.")?])
})?;

let func: JsFunction = env.func(move |this, [a1]: [JsFunction; 1]| {
    let env = this.env();
    a1.call(this, [env.string("I am from rust world.")?])
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

### gabage-collected hook

for napi less than 5, implement by napi_wrap, otherwise by napi_add_finalizer.

```rust
let mut obj = env.object()?;
obj.gc(move |_| {
    println!("obj garbage-collected");
    Ok(())
});
```

### Wrap native instance

```rust
let mut obj = env.object()?;
obj.wrap([1usize; 2], move |_, wrapped| {
    Ok(())
})?;
obj.unwrap::<[usize; 2]>()?; // access the wrapped instance
obj.remove_wrap::<[usize; 2]>()?; // the finalizer will not be called
```

### Thread safe function

require: napi >= 4

```rust
let tsfn = NapiThreadsafeFunction::new(
    env,
    "tsfn-task",
    env.func(|this, [a1]: [JsString; 1]| {
        println!("callback result: {}", a1.get()?);
        this.env().undefined()
    })?,
    // finalizer
    move |_| Ok(()),
    // js-callback
    move |f, data: String| {
        f.call::<JsString, 1>(env.object()?, [env.string(&data)?])?;
        Ok(())
    },
)?;

std::thread::spawn(move || {
    tsfn.call(
        "hello, world - 1".into(),
        NapiThreadsafeFunctionCallMode::Nonblocking,
    )
    .unwrap();

    tsfn.call(
        "hello, world - 2".into(),
        NapiThreadsafeFunctionCallMode::Nonblocking,
    )
    .unwrap();

    tsfn.release(NapiThreadsafeFunctionReleaseMode::Release)
        .unwrap();
});
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
