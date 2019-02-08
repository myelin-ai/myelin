#!/usr/bin/env python3

from http.server import HTTPServer, SimpleHTTPRequestHandler
import mimetypes
import sys

PORT = int(sys.argv[1])
ADDRESS = ('127.0.0.1', PORT)


class Handler(SimpleHTTPRequestHandler):
    pass


Handler.extensions_map['.wasm'] = 'application/wasm'

server = HTTPServer(ADDRESS, Handler)
print(f'Listening on port {PORT}')
server.serve_forever()
