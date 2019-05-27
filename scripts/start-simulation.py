#!/usr/bin/env python3

import subprocess
import threading
import os
from concurrent.futures import ThreadPoolExecutor, Future
import argparse
import time
from typing import List
import sys
from socket import create_connection

_CARGO_WORKSPACE = os.path.join(os.path.dirname(__file__), '..')
_WEBSOCKET_PORT = 6956


def start(release=False, open=False, build=True):
    if build:
        _build_visualization_client(release)

    with ThreadPoolExecutor(max_workers=2) as executor:
        futures = [
            executor.submit(_serve_visualization_client),
            executor.submit(_start_visualization_server, release)
        ]

        [future.result() for future in as_completed(futures)]

    if open:
        _poll_visualization_server()
        _open_browser()


def _parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument('--release', action='store_true',
                        help='Builds in release mode')
    parser.add_argument('--open', action='store_true',
                        help='Opens a browser after starting')
    parser.add_argument('--no-build', action='store_true',
                        help='Do not build anything, just run')
    return parser.parse_args()


def _serve_visualization_client():
    command = os.path.join(_CARGO_WORKSPACE, 'visualization-client',
                           'scripts', 'serve.py')
    subprocess.check_call(command)


def _build_visualization_client(release: bool):
    subprocess.check_call(_get_visualization_client_build_command(release))


def _get_visualization_client_build_command(release: bool) -> List[str]:
    executable = os.path.join(
        _CARGO_WORKSPACE, 'visualization-client',  'scripts', 'build.py')
    command = [executable, '--webpack']
    return _append_release_flag(command, release)


def _start_visualization_server(release: bool):
    subprocess.check_call(_get_start_visualization_server_command(release))


def _get_start_visualization_server_command(release: bool) -> List[str]:
    return _append_release_flag(['cargo', 'run', '-p',
                                 'myelin-visualization-server'], release)


def _append_release_flag(command: List[str], release: bool) -> List[str]:
    if release:
        return [*command, '--release']
    else:
        return command


def _open_browser():
    subprocess.call(['open', 'http://localhost:8080'])


def _poll_visualization_server():
    while True:
        connection = None
        try:
            connection = create_connection(('localhost', _WEBSOCKET_PORT))
            break
        except ConnectionRefusedError as e:
            time.sleep(0.1)
        finally:
            if connection is not None:
                connection.close()


if __name__ == '__main__':
    args = _parse_arguments()
    try:
        start(release=args.release, open=args.open, build=not args.no_build)
    except KeyboardInterrupt:
        sys.exit(0)
