#!/bin/bash
set -euo pipefail

mkdir -p ./target ./build

cargo build
cargo build -r

cp ./target/debug/libreproduce_rust_145163.so ./build/debug.node
cp ./target/release/libreproduce_rust_145163.so ./build/release.node
