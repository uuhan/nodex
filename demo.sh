#!/bin/bash
cargo build -p demo && \
cp target/debug/deps/libdemo.dylib demo.node && \
node --napi-modules -e "require(\"./demo.node\")"
