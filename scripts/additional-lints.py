#!/usr/bin/env python3.7

import glob
from typing import Optional, List, Tuple
from dataclasses import dataclass
import sys


@dataclass(frozen=True)
class Error:
    lines: List[Tuple[int, str]]
    message: str


@dataclass(frozen=True)
class CheckedFile:
    filename: str
    errors: List[Error]


def check_files(filenames: List[str]) -> List[CheckedFile]:
    return [_check_file(filename) for filename in filenames if _should_include_file(filename)]


def _check_file(filename: str) -> CheckedFile:
    with open(filename, 'r') as file:
        errors = [*_check_for_box_syntax_new(file)]

    return CheckedFile(filename=filename, errors=errors)


def _check_for_box_syntax_new(file) -> List[Error]:
    ERROR_MESSAGE = 'Use box syntax instead of Box::new'
    errors = []
    for (number, line) in enumerate(file, start=1):
        if _line_contains_box_new(line) and not _is_comment_line(line):
            errors.append(
                Error(lines=[(number, line.rstrip())], message=ERROR_MESSAGE))
    return errors


def _line_contains_box_new(line: str) -> bool:
    return 'Box::new' in line


def _is_comment_line(line: str) -> bool:
    return line.strip().startswith('//')


def _should_include_file(filename: str) -> bool:
    return not filename.startswith('target/')


def _print_checked_file(file: CheckedFile):
    if len(file.errors) > 0:
        for error in file.errors:
            _print_error(file, error)


def _print_error(file: CheckedFile, error: Error):
    print(f'error: {error.message}')
    for line in error.lines:
        print(f'  --> {file.filename}:{line[0]}')
        print(f'   |')
        print(f'   | {line[1]}')
        print(f'   |')


def _get_files_to_check() -> List[str]:
    return glob.iglob('**/*.rs', recursive=True)


if __name__ == '__main__':
    checked_files = check_files(_get_files_to_check())
    files_with_errors = [file for file in checked_files if len(file.errors) > 0]

    for file in checked_files:
        _print_checked_file(file)

    if len(files_with_errors) > 0:
        sys.exit(1)
