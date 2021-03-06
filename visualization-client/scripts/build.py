#!/usr/bin/env python3

import sys
import subprocess
import os
import shutil
import argparse
from typing import List

_WASM_TARGET = 'wasm32-unknown-unknown'
_BINARY_NAME = 'myelin_visualization_client'


def build(with_webpack=False, release=False):
    crate_dir = os.path.join(os.path.dirname(__file__), '..')
    out_dir = os.path.join(crate_dir, 'out')
    build_mode = 'release' if release else 'debug'
    wasm_file = os.path.join(crate_dir, '..', 'target',
                             _WASM_TARGET, build_mode, f'{_BINARY_NAME}.wasm')

    subprocess.check_call(_get_cargo_command(release), cwd=crate_dir)

    shutil.rmtree(out_dir, ignore_errors=True)
    os.makedirs(out_dir)
    subprocess.check_call(['wasm-bindgen', wasm_file, '--out-dir', out_dir])

    if with_webpack:
        subprocess.check_call(['yarn'], cwd=crate_dir)
        subprocess.check_call(['yarn', 'webpack'], cwd=crate_dir)


def _get_cargo_command(release: bool) -> List[str]:
    command = ['cargo', 'build', '--target', _WASM_TARGET]
    if release:
        command.append('--release')
    return command


def _parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument('--webpack', action='store_true',
                        help='Runs webpack after building')
    parser.add_argument('--release', action='store_true',
                        help='Builds in release mode')
    return parser.parse_args()


if __name__ == '__main__':
    args = _parse_arguments()
    try:
        build(release=args.release, with_webpack=args.webpack)
    except KeyboardInterrupt:
        sys.exit(0)
