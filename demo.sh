#!/bin/bash
cargo build -p demo && \
cp target/debug/deps/libdemo.dylib demo.node && \
cat << EOF | node --napi-modules -
  const demo = require("./demo.node")
  console.log(demo.hello)
EOF
