#!/bin/bash
cargo build -p demo && \
cp target/debug/deps/libdemo.dylib demo.node && \
cat << EOF | node --napi-modules -
  const demo = require("./demo.node")

  console.log(demo.utils.func())
  console.log(demo.utils.func())

  console.log(demo.key1)
EOF
