#!/bin/bash
set -e

NAME="$1"
RELEASE="$2"
TARGET="$3:=wasm32-unknown-unknown"

rustup target add $TARGET
pushd "plugins/$NAME"


if [ "$RELEASE" == "release" ]; then
    echo "release build"
    cargo build --release
    cp "target/$TARGET/release/$NAME.wasm" ..
else
    echo debug
    cargo build
    cp "target/$TARGET/debug/$NAME.wasm" ..
fi

popd