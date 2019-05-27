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
from concurrent.futures import wait, FIRST_COMPLETED, ThreadPoolExecutor
from threading import Event
import signal
from typing import List, Tuple

HTTP_PORT = 8081
SERVER_CRATE = 'myelin-visualization-server'
_WEBSOCKET_PORT = 6956
_BEGIN_LOGS_MARKER = '----- BEGIN LOGS -----'

WEB_DIR = os.path.abspath(os.path.join(
    os.path.dirname(__file__), '..', '..', 'public'))
CARGO_ROOT = os.path.abspath(os.path.join(
    os.path.dirname(__file__), '..', '..', '..'))


class _CancellationToken:
    def __init__(self, event: Event):
        self._event = event

    def is_cancelled(self) -> bool:
        return self._event.is_set()


class _CancellationTokenSource:
    def __init__(self, event: Event):
        self._event = event

    def cancel(self):
        return self._event.set()


def _create_cancellation_token() -> Tuple[_CancellationTokenSource, _CancellationToken]:
    event = Event()
    return (_CancellationTokenSource(event), _CancellationToken(event))


def _start_http_server(cancellation_token: _CancellationToken):
    http_server_bin = os.path.abspath(os.path.join(
        os.path.dirname(__file__), 'http_server.py'))
    _start_process([http_server_bin, str(HTTP_PORT)],
                   cancellation_token, cwd=WEB_DIR)


def _start_websocket_server(cancellation_token: _CancellationToken):
    _start_process(['cargo', 'run', '-p', SERVER_CRATE],
                   cancellation_token, cwd=CARGO_ROOT)


def _start_process(command: List[str], cancellation_token: _CancellationToken, *args, **kwargs):
    with subprocess.Popen(command, *args, **kwargs) as process:
        while True:
            if cancellation_token.is_cancelled():
                process.send_signal(signal.SIGTERM)
                process.wait()
                break
            returncode = process.poll()
            if returncode:
                raise subprocess.CalledProcessError(command, returncode)
            time.sleep(0.1)


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

        # For some reason, Firefox is unable to connect to
        # the websocket on the first page load.
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
        except ConnectionRefusedError:
            time.sleep(0.1)
        finally:
            if connection is not None:
                connection.close()


with ThreadPoolExecutor(max_workers=3) as pool:
    cancellation_token_source, cancellation_token = _create_cancellation_token()

    futures = [
        pool.submit(_start_http_server, cancellation_token),
        pool.submit(_start_websocket_server, cancellation_token),
        pool.submit(_start_webdriver),
    ]

    done, _ = wait(futures, return_when=FIRST_COMPLETED)
    cancellation_token_source.cancel()
    [future.result() for future in done]
