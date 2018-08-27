#!/usr/bin/env bash
set -e

application_name=myelin_visualization
target_dir=visualization/out

(cd visualization && yarn)

cargo build --target wasm32-unknown-unknown
rm -r $target_dir || true
mkdir $target_dir
wasm-bindgen target/wasm32-unknown-unknown/debug/$application_name.wasm --out-dir $target_dir

(cd visualization && yarn webpack)
