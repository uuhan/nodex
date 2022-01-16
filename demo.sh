#!/bin/bash
rm -f demo.node

cargo build -p demo && \
cp target/debug/deps/libdemo.dylib demo.node && \
node --napi-modules demo.js
