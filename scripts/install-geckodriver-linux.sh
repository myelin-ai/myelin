#!/usr/bin/env bash

version="v0.24.0"
archive_file="geckodriver-$version-linux64.tar.gz"
download_url="https://github.com/mozilla/geckodriver/releases/download/$version/$archive_file"
temp_directory=$(mktemp -d)

cd "$temp_directory"

wget $download_url
tar -xf $archive_file
sudo cp geckodriver /usr/local/bin/
