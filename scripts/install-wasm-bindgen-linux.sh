#!/usr/bin/env bash

version=0.2.43
download_url="https://github.com/rustwasm/wasm-bindgen/releases/download/$version/wasm-bindgen-$version-x86_64-unknown-linux-musl.tar.gz"
archive_file="wasm-bindgen.tar.gz"
temp_directory=$(mktemp -d)

cd "$temp_directory"

wget -O "$archive_file" "$download_url"
tar -xf "$archive_file"
cp wasm-bindgen /usr/local/bin/
