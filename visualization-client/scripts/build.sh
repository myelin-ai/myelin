#!/usr/bin/env bash
set -e

crate_dir=$(cd -- "$(dirname -- "$0")/.." && pwd)

application_name=myelin_visualization_client
target_dir="$crate_dir/out"

(cd $crate_dir && cargo build --target wasm32-unknown-unknown)

rm -rf -- "$target_dir"
mkdir -- "$target_dir"
wasm-bindgen "$crate_dir/../target/wasm32-unknown-unknown/debug/$application_name.wasm" \
             --out-dir "$target_dir"

(cd $crate_dir && yarn && yarn webpack)
