#!/bin/bash

set -x
rm -rf ./build

rustup update nightly
rustup override add nightly
rustup component add rust-src

rust_dir=$(rustc --print sysroot)/lib/rustlib/src/rust

git submodule update
mkdir -p ./build
rustc --target x86_64-unknown-none-gnu -Z no-landing-pads --out-dir ./build ${rust_dir}/src/libcore/lib.rs
rustc --target x86_64-unknown-none-gnu -Z no-landing-pads -L ./build --out-dir ./build ${rust_dir}/src/libstd_unicode/lib.rs
rustc --target x86_64-unknown-none-gnu -Z no-landing-pads -L ./build --out-dir ./build ${rust_dir}/src/liballoc/lib.rs
rustc --target x86_64-unknown-none-gnu -Z no-landing-pads -L ./build --out-dir ./build ${rust_dir}/src/libcollections/lib.rs
