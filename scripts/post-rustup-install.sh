#!/usr/bin/env bash

set -e

rustup component add rustfmt clippy
rustup target add wasm32-unknown-unknown
