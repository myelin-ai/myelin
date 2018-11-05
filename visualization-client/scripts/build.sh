#!/usr/bin/env bash
set -e

webpack=1
show_help=0
release=0

for arg in $@
do
  case "$arg" in 
    --no-webpack )
      webpack=0
      ;;
    --help )
      show_help=1
      ;;
    --release )
      release=1
      ;;
    *)
      echo "Argument $arg not supported"
      exit 1
      ;;
  esac
done

if [[ $show_help -eq 1 ]]
then
  echo "Usage: $0 [flags]"
  echo ""
  echo "Supported flags:"
  echo "--no-webpack    Builds without webpack"
  echo "--release       Builds in release mode"
  echo "--help          Prints this help"
  exit
fi

crate_dir=$(cd -- "$(dirname -- "$0")/.." && pwd)

application_name=myelin_visualization_client
target_dir="$crate_dir/out"

release_flag=
if [[ $release -eq 1 ]]
then
  release_flag=--release
fi

(cd -- "$crate_dir" && cargo build --target wasm32-unknown-unknown $release_flag)

rm -rf -- "$target_dir"
mkdir -- "$target_dir"
wasm-bindgen "$crate_dir/../target/wasm32-unknown-unknown/debug/$application_name.wasm" \
             --out-dir "$target_dir"

if [[ $webpack -eq 1 ]]
then
  (cd -- "$crate_dir" && yarn && yarn webpack)
fi
