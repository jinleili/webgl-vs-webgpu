#!/bin/bash

# 编译 webgl 包
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --no-default-features --release --target wasm32-unknown-unknown --features webgl \
--bin webgl-vs-webgpu

for i in target/wasm32-unknown-unknown/release/*.wasm;
do
    wasm-bindgen --no-typescript --out-dir docs/.vitepress/components/wasm/webgl --web "$i";
done

# 编译 webgpu 包
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --no-default-features --release --target wasm32-unknown-unknown \
--bin webgl-vs-webgpu

for i in target/wasm32-unknown-unknown/release/*.wasm;
do
    wasm-bindgen --no-typescript --out-dir docs/.vitepress/components/wasm/webgpu --web "$i";
done