#!/usr/bin/env bash

set -e

version=0.8.6
archive_name="cargo-tarpaulin-$version-travis"
archive_file="$archive_name.tar.gz"
download_url="https://github.com/xd009642/tarpaulin/releases/download/$version/$archive_file"
temp_directory=$(mktemp -d)

cd "$temp_directory"

wget "$download_url"
tar -xf "$archive_file"
sudo cp "cargo-tarpaulin" /usr/local/bin/
