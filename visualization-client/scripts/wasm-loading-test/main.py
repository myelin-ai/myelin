#!/usr/bin/env python3

from selenium import webdriver
from socket import create_connection
import os
import subprocess
import signal
import time
import sys
from tempfile import mkstemp
import json

HTTP_PORT = 8081
SERVER_CRATE = 'myelin-visualization-server'
_WEBSOCKET_PORT = 6956
_BEGIN_LOGS_MARKER = '----- BEGIN LOGS -----'

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
    options = webdriver.firefox.options.Options()
    options.headless = True

    profile = webdriver.FirefoxProfile()
    profile.set_preference('devtools.console.stdout.content', True)
    profile.update_preferences()

    # Sleep for a bit to make sure server is started
    _poll_visualization_server()

    _, log_file = mkstemp(prefix='geckodriver', suffix='.log')

    with webdriver.Firefox(options=options, firefox_profile=profile, service_log_path=log_file) as driver:
        time.sleep(2)

        driver.get(f'http://localhost:{HTTP_PORT}')
        driver.execute_script(f'console.log(\'{_BEGIN_LOGS_MARKER}\')')
        driver.get(f'http://localhost:{HTTP_PORT}')

        # Sleep for a bit to make sure everything is properly loaded
        time.sleep(2)

        body = driver.find_element_by_css_selector('body')

        assert 'Failed to initialize visualization' not in body.text

        # Sleep for a bit, so panics that might be caused
        # by a websocket message are caught
        time.sleep(5)

        with open(log_file, 'r') as f:
            log_lines = f.read().splitlines()

        index_of_start_marker = log_lines.index(
            f'console.log: "{_BEGIN_LOGS_MARKER}"')
        log_lines = log_lines[index_of_start_marker:]

        severe_messages = [
            msg for msg in log_lines if msg.startswith('console.error:')]

        if not len(severe_messages) == 0:
            print('Error: fatal messages found in console')
            for message in severe_messages:
                print('')
                print(message)
            sys.exit(1)

    if os.path.exists(log_file):
        os.remove(log_file)


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


httpd = _start_http_server()
websocket = _start_websocket_server()

try:
    _start_webdriver()
finally:
    os.kill(httpd.pid, signal.SIGTERM)
    os.kill(websocket.pid, signal.SIGTERM)
