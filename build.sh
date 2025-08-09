#!/bin/bash
set -euo pipefail

mkdir -p ./target ./build

rustup component add --toolchain nightly-2025-08-06-x86_64-unknown-linux-gnu miri

# note: workaround miri tmp exe
# could not execute process `/tmp/.tmpXXX/target/custom_sysroot/build/std-XXX/build-script-build`
# sudo mount -o remount exec /tmp

cargo miri setup
cargo miri test

cargo build
cargo build -r

cp ./target/debug/libreproduce_rust_145163.so ./build/debug.node
cp ./target/release/libreproduce_rust_145163.so ./build/release.node
