#!/usr/bin/env python3

import sys
import subprocess
import os
import shutil
import argparse

WASM_TARGET = 'wasm32-unknown-unknown'
BINARY_NAME = 'myelin_visualization_client'


def parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument('--webpack', '-W', action='store_true',
                        help='Runs webpack after building')
    parser.add_argument('--release', '-R', action='store_true',
                        help='Builds in release mode')
    return parser.parse_args()


args = parse_arguments()
release = args.release
webpack = args.webpack

crate_dir = os.path.join(os.path.dirname(__file__), '..')
out_dir = os.path.join(crate_dir, 'out')
build_mode = 'release' if release else 'debug'
wasm_file = os.path.join(crate_dir, '..', 'target',
                         WASM_TARGET, build_mode, '{}.wasm'.format(BINARY_NAME))

cargo_command = ['cargo', 'build', '--target', WASM_TARGET]
if release:
    cargo_command.append('--release')
subprocess.check_call(cargo_command, cwd=crate_dir)

shutil.rmtree(out_dir, ignore_errors=True)
os.makedirs(out_dir)
subprocess.check_call(['wasm-bindgen', wasm_file, '--out-dir', out_dir])

if webpack:
    subprocess.check_call(['yarn'], cwd=crate_dir)
    subprocess.check_call(['yarn', 'webpack'], cwd=crate_dir)
