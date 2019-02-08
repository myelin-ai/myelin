#!/usr/bin/env python3

import os
from http.server import HTTPServer, SimpleHTTPRequestHandler
import sys

_DEFAULT_ADDRESS = ('127.0.0.1', 8080)
_PUBLIC_DIRECTORY = os.path.join(
    os.path.dirname(__file__), '..', 'public')


class _Handler(SimpleHTTPRequestHandler):
    def __init__(self, *args, directory=None, **kwargs):
        super().__init__(*args, **kwargs, directory=_PUBLIC_DIRECTORY)

    extensions_map = {
        **SimpleHTTPRequestHandler.extensions_map,
        '.wasm': 'application/wasm',
    }


def serve(address=_DEFAULT_ADDRESS):
    server = HTTPServer(address, _Handler)
    print(f'Serving on {address}')
    server.serve_forever()


if __name__ == '__main__':
    try:
        serve()
    except KeyboardInterrupt:
        sys.exit(0)
