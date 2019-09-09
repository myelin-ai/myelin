#!/usr/bin/env bash

set -e

version=0.2.50
archive_name="wasm-bindgen-$version-x86_64-unknown-linux-musl"
archive_file="$archive_name.tar.gz"
download_url="https://github.com/rustwasm/wasm-bindgen/releases/download/$version/$archive_file"
temp_directory=$(mktemp -d)

cd "$temp_directory"

wget "$download_url"
tar -xf "$archive_file"
sudo cp "$archive_name/wasm-bindgen" /usr/local/bin/
