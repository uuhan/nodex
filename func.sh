#!/bin/bash
cargo build -p func

rm -f func.node
[[ -e target/debug/deps/libfunc.dylib ]] && cp target/debug/deps/libfunc.dylib func.node
[[ -e target/debug/deps/libfunc.so ]] && cp target/debug/deps/libfunc.so func.node

node --napi-modules func.js
