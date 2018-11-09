#!/usr/bin/env python3

from selenium import webdriver
import os
import subprocess
import signal

HTTP_PORT = 8081

WEB_DIR = os.path.abspath(os.path.join(
    os.path.dirname(__file__), '..', '..', 'public'))
CARGO_ROOT = os.path.abspath(os.path.join(
    os.path.dirname(__file__), '..', '..', '..'))


def _start_http_server():
    http_server_bin = os.path.abspath(os.path.join(
        os.path.dirname(__file__), 'http_server.py'))

    return subprocess.Popen([http_server_bin, str(HTTP_PORT)], cwd=WEB_DIR)


def _start_websocket_server():
    return subprocess.Popen(['cargo', 'run'], cwd=CARGO_ROOT)


def _start_webdriver():
    options = webdriver.chrome.options.Options()
    options.headless = True

    with webdriver.Chrome(options=options) as driver:
        driver.get('http://localhost:{}'.format(HTTP_PORT))

        body = driver.find_element_by_css_selector('body')

        assert 'Failed to initialize visualization' not in body.text


httpd = _start_http_server()
websocket = _start_websocket_server()

try:
    _start_webdriver()
finally:
    os.kill(httpd.pid, signal.SIGTERM)
    os.kill(websocket.pid, signal.SIGTERM)
