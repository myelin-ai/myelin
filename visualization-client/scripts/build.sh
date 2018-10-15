#!/usr/bin/env bash
set -e

application_name=myelin_visualization_client
target_dir=visualization-client/out

cargo build -p myelin-visualization-client --target wasm32-unknown-unknown

rm -rf $target_dir
mkdir $target_dir
wasm-bindgen target/wasm32-unknown-unknown/debug/$application_name.wasm --out-dir $target_dir

(cd visualization-client && yarn && yarn webpack)
