#!/bin/sh

VERSION=0.2.0
TARGET_DIR=./release/linux/cal-$VERSION/
BUILD_TARGET=x86_64-unknown-linux-gnu

cargo build --target $BUILD_TARGET --release

mkdir -p $TARGET_DIR && cp ./target/$BUILD_TARGET/release/cal-server $TARGET_DIR