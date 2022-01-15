## nodex

Yet another crate to create native nodejs addons :)

This crate aims to make creating native nodejs addons very easy and comfortable.

It is in a very early stage and heavy development is making.

## Examples

### Init Module

```rust
// lib.rs
use nodex_api::{api, prelude::*};

nodex_api::init!(init);

fn init(env: NapiEnv, exports: JsValue) -> NapiResult<()> {
    Ok(())
}
```

### Demo

```rust
use nodex_api::{api, prelude::*};

nodex_api::init!(init);

fn init(env: NapiEnv, exports: JsValue) -> NapiResult<()> {
    let name = std::ffi::CString::new("utils").unwrap();

    let mut obj = JsObject::new(env)?;
    let mut times = 0;

    obj.set(
        JsString::new(env, "func")?,
        JsFunction::with(env, "func", move || {
            times += 1;
            println!("[{}] called", times);
        })?,
    )?;

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

    let desc = api::napi_property_descriptor {
        utf8name: name.as_ptr(),
        name: std::ptr::null_mut(),
        method: None,
        getter: None,
        setter: None,
        value: obj.raw(),
        attributes: NapiPropertyAttributes::Default.bits(),
        data: std::ptr::null_mut(),
    };

    let status = unsafe { api::napi_define_properties(env.raw(), exports.raw(), 1, &desc) };
    assert_eq!(status, NapiStatus::Ok);

    Ok(())
}
```

Run:

```bash
bash demo.sh

# output
# [1] called
# { func: [Function: func] }
# [2] called
# { func: [Function: func] }
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
