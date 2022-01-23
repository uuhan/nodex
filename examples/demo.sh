#!/bin/bash
NAME=$(basename $0)
NAME=${NAME/.sh/}
pushd $(dirname $0) &>/dev/null

rm -f ${NAME}.node
cargo build -p ${NAME}

[[ -e ../target/debug/deps/lib${NAME}.dylib ]] && cp ../target/debug/deps/lib${NAME}.dylib ${NAME}.node
[[ -e ../target/debug/deps/lib${NAME}.so ]] && cp ../target/debug/deps/lib${NAME}.so ${NAME}.node

node --napi-modules ${NAME}.js

popd &>/dev/null
