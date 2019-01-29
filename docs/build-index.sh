#!/usr/bin/env bash

pipenv_dir=$(cd -- "$(dirname -- "$0")" && pwd)

cd -- "$pipenv_dir"

pipenv install
pipenv run ./build-index.py
