#!/bin/bash
set -e

NAME="$1"
RELEASE="$2"

rustup target add wasm32-unknown-unknown
pushd "plugins/$NAME"


if [ "$RELEASE" == "release" ]; then
    echo "release build"
    cargo build --release
    cp "target/wasm32-unknown-unknown/release/$NAME.wasm" ..
else
    echo debug
    cargo build
    cp "target/wasm32-unknown-unknown/debug/$NAME.wasm" ..
fi

popd