#!/usr/bin/env python3

from dataclasses import dataclass
import os
from typing import List
from pprint import pprint
from string import Template
from html import escape
import subprocess
import json
import re


@dataclass(frozen=True)
class Package:
    name: str
    manifest_path: str
    version: str


def clean_package_artifacts(packages: List[Package]):
    arguments = _cargo_clean_arguments(packages)
    subprocess.check_call(['cargo', 'clean', *arguments])


def _get_packages() -> List[Package]:
    cargo_metadata_string = subprocess.check_output(
        ['cargo', 'metadata', '--no-deps', '--all-features', '--format-version', '1']).decode('utf-8')
    cargo_metadata = json.loads(cargo_metadata_string)
    packages = cargo_metadata['packages']

    return [_map_package(package) for package in packages]


def _map_package(package: dict) -> Package:
    name = package['name']
    return Package(name=name, manifest_path=package['manifest_path'], version=package['version'])


def _cargo_clean_arguments(packages: List[Package]):
    arguments = []
    for package in packages:
        arguments = [*arguments, '-p', package.name]
    return arguments


def _print_packages(packages: List[Package]):
    cleaning = _green_and_bold('Cleaning')
    for package in packages:
        manifest_dir = os.path.dirname(package.manifest_path)
        print(
            f'    {cleaning} {package.name} v{package.version} ({manifest_dir})')


def _green_and_bold(string: str) -> str:
    _green = _ansii_sequence('92')
    _bold = _ansii_sequence('1')
    _reset = _ansii_sequence('00')
    return f'{_green}{_bold}{string}{_reset}'


def _ansii_sequence(value: str) -> str:
    return f'\033[{value}m'


if __name__ == '__main__':
    packages = _get_packages()
    _print_packages(packages)
    clean_package_artifacts(packages)
