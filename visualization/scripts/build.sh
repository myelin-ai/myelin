#!/usr/bin/env bash
set -e

application_name=myelin_visualization
target_dir=visualization/out

cargo build --target wasm32-unknown-unknown --release

rm -rf $target_dir
mkdir $target_dir
wasm-bindgen target/wasm32-unknown-unknown/debug/$application_name.wasm --out-dir $target_dir

(cd visualization && yarn && yarn webpack)
