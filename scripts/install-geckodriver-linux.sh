#!/usr/bin/env bash

version="v0.24.0"
archive_file="geckodriver-$version-linux64.tar.gz"
download_url="https://github.com/mozilla/geckodriver/releases/download/$version/$archive_file"
temp_directory=$(mktemp -d)

wget -O "$temp_directory/$archive_file" "$download_url"
tar -xf "$temp_directory/$archive_file"
sudo cp "$temp_directory/geckodriver" /usr/local/bin/
