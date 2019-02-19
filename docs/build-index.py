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

_WORKSPACE_ROOT = os.path.join(os.path.dirname(__file__), '..')
_CARGO_MANIFEST_FILE = 'Cargo.toml'
_CRATE_TEMPLATE_FILE = os.path.join(_WORKSPACE_ROOT, 'docs', 'crate.html')
_MAIN_TEMPLATE_FILE = os.path.join(_WORKSPACE_ROOT, 'docs', 'index.html')
_OUTPUT = os.path.join(_WORKSPACE_ROOT, 'target', 'doc', 'index.html')
_PACKAGE_NAME_REGEX = r'^myelin-|mockiato-?$'


@dataclass(frozen=True)
class Crate:
    name: str
    package_name: str
    description: str


def _get_crates() -> List[Crate]:
    cargo_metadata_string = subprocess.check_output(
        ['cargo', 'metadata', '--all-features', '--format-version', '1']).decode('utf-8')
    cargo_metadata = json.loads(cargo_metadata_string)

    crates = []
    for package in cargo_metadata['packages']:
        if _should_include_package(package['name']):
            crates.append(_map_package_to_crate(package))
    return crates


def _map_package_to_crate(package: dict) -> Crate:
    package_name = package['name']
    description = package['description']
    name = _translate_package_name_to_crate_name(package_name)
    return Crate(name=name, package_name=package_name,
                 description=description)


def _should_include_package(package_name: str) -> bool:
    return re.match(_PACKAGE_NAME_REGEX, package_name) is not None


def _translate_package_name_to_crate_name(package_name: str) -> str:
    return package_name.replace('-', '_')


def _get_crate_template() -> Template:
    with open(_CRATE_TEMPLATE_FILE, 'r') as f:
        return Template(f.read())


def _get_main_template() -> Template:
    with open(_MAIN_TEMPLATE_FILE, 'r') as f:
        return Template(f.read())


def _render_crate(template: Template, crate: Crate) -> str:
    return template.safe_substitute(
        name=escape(crate.name),
        package_name=escape(crate.package_name),
        description=escape(crate.description))


def _render_crates(crate_template: Template, crates: List[Crate]) -> str:
    return '\n'.join(_render_crate(crate_template, crate) for crate in crates)


def _render(template: Template, rendered_crates: str) -> str:
    return template.safe_substitute(crates=rendered_crates)


def build_index():
    crates = sorted(_get_crates(), key=lambda crate: crate.name)
    rendered_crates = _render_crates(_get_crate_template(), crates)
    rendered = _render(_get_main_template(), rendered_crates)

    with open(_OUTPUT, 'w+') as f:
        f.write(rendered)


if __name__ == '__main__':
    build_index()
