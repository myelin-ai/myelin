#!/usr/bin/env python3

import sys
import getopt
import subprocess
import os
import shutil

USAGE = """Usage: {} [flags]

Supported flags:
--webpack|-w       Runs webpack after building
--release|-r       Builds in release mode
--help|-h          Prints this help"""

WASM_TARGET = 'wasm32-unknown-unknown'
BINARY_NAME = 'myelin_visualization_client'


def print_usage():
    print(USAGE.format(sys.argv[0]))


try:
    opts, args = getopt.getopt(
        sys.argv[1:], "wrh", ["webpack", "release", "help"])
except getopt.GetoptError as err:
    print(err)
    sys.exit(1)

help = False
release = False
webpack = False

for opt, _ in opts:
    if opt in ('--webpack', '-w'):
        webpack = True
    if opt in ('--release', '-r'):
        release = True
    if opt in ('--help', '-h'):
        help = True

if help:
    print_usage()
    sys.exit()

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
