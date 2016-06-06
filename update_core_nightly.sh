#!/bin/bash

#rustup update nightly-2016-05-22
#rustup override add nightly-2016-05-22
rustup update nightly
rustup override add nightly
git submodule update
pushd .
cd ../rust
git checkout master
git submodule update
git pull
#git checkout 476fe6eef
popd
mkdir -p ./build
rustc --target x86_64-unknown-none-gnu -Z no-landing-pads --out-dir ./build ../rust/src/libcore/lib.rs
