#!/usr/bin/env python3

import subprocess
import threading
import os
from concurrent.futures import ThreadPoolExecutor, Future
import argparse
import time

_CARGO_WORKSPACE = os.path.join(os.path.dirname(__file__), '..')


def start(release=False, open=False):
    _build_visualization_client(release)
    _build_visualization_server(release)

    pool = ThreadPoolExecutor(max_workers=2)
    pool.submit(_serve_visualization_client)
    pool.submit(_start_visualization_server, release)

    if open:
        time.sleep(3)
        _open_browser()


def _parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument('--release', '-R', action='store_true',
                        help='Builds in release mode')
    parser.add_argument('--open', action='store_true',
                        help='Opens a browser after starting')
    return parser.parse_args()


def _serve_visualization_client():
    subprocess.check_call([
        os.path.join(_CARGO_WORKSPACE, 'visualization-client',
                     'scripts', 'serve.py')
    ])


def _build_visualization_client(release: bool):
    build_command = [os.path.join(_CARGO_WORKSPACE, 'visualization-client',
                                  'scripts', 'build.py'), '--webpack']
    if release:
        build_command.append('--release')

    subprocess.check_call(build_command)


def _build_visualization_server(release: bool):
    command = ['cargo', 'build', '-p',
               'myelin-visualization-server']
    if release:
        command.append('--release')
    subprocess.check_call(command)


def _start_visualization_server(release: bool):
    command = ['cargo', 'run', '-p',
               'myelin-visualization-server']
    if release:
        command.append('--release')
    subprocess.check_call(command)


def _open_browser():
    subprocess.call(['open', 'http://localhost:8080'])


if __name__ == '__main__':
    args = _parse_arguments()
    start(release=args.release, open=args.open)
