#!/bin/bash
cargo build -p demo && \
cp target/debug/deps/libdemo.dylib demo.node && \
cat << EOF | node --napi-modules -
  const demo = require("./demo.node")
  const async_hooks = require("async_hooks")

  console.log(demo.utils.func())
  console.log(demo.utils.func())

  console.log(async_hooks.executionAsyncResource())
EOF
