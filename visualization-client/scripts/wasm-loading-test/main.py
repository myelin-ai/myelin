#!/usr/bin/env python3

from selenium import webdriver
import os
import subprocess
import signal
import time
import sys

HTTP_PORT = 8081
SERVER_CRATE = 'myelin-visualization-server'

WEB_DIR = os.path.abspath(os.path.join(
    os.path.dirname(__file__), '..', '..', 'public'))
CARGO_ROOT = os.path.abspath(os.path.join(
    os.path.dirname(__file__), '..', '..', '..'))


def _start_http_server():
    http_server_bin = os.path.abspath(os.path.join(
        os.path.dirname(__file__), 'http_server.py'))

    return subprocess.Popen([http_server_bin, str(HTTP_PORT)], cwd=WEB_DIR)


def _start_websocket_server():
    # Ensures that `cargo run` doesn't take too long, especially on Jenkins when
    # `cargo run` might wait for a lock file
    subprocess.call(['cargo', 'build', '-p', SERVER_CRATE], cwd=CARGO_ROOT)
    return subprocess.Popen(['cargo', 'run', '-p', SERVER_CRATE], cwd=CARGO_ROOT)


def _start_webdriver():
    options = webdriver.chrome.options.Options()
    options.headless = True

    # Sleep for a bit to make sure server is started
    time.sleep(2)

    with webdriver.Chrome(options=options) as driver:
        driver.get(f'http://localhost:{HTTP_PORT}')

        # Sleep for a bit to make sure everything is properly loaded
        time.sleep(2)

        body = driver.find_element_by_css_selector('body')

        assert 'Failed to initialize visualization' not in body.text

        # Sleep for a bit, so panics that might be caused
        # by a websocket message are caught
        time.sleep(5)

        severe_messages = [msg for msg in driver.get_log(
            'browser') if msg['level'] == 'SEVERE']

        if not len(severe_messages) == 0:
            print('Error: fatal messages found in console')
            for message in severe_messages:
                message_source = message['source']
                message_text = message['message']
                print('')
                print(f'source: {message_source}')
                print(f'message: {message_text}')
            sys.exit(1)


httpd = _start_http_server()
websocket = _start_websocket_server()

try:
    _start_webdriver()
finally:
    os.kill(httpd.pid, signal.SIGTERM)
    os.kill(websocket.pid, signal.SIGTERM)
