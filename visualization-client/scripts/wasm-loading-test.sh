#!/usr/bin/env bash

pipenv_dir=$(cd -- "$(dirname -- "$0")/wasm-loading-test" && pwd)

cd -- "$pipenv_dir"

pipenv install
pipenv run ./main.py
