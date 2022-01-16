#!/bin/bash
rm -f demo.node
cargo build -p demo

[[ -e target/debug/deps/libdemo.dylib ]] && cp target/debug/deps/libdemo.dylib demo.node
[[ -e target/debug/deps/libdemo.so ]] && cp target/debug/deps/libdemo.so demo.node

node --napi-modules demo.js
