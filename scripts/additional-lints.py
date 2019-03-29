#!/usr/bin/env python3.7

import glob
import re
import sys
from dataclasses import dataclass
from typing import List, Tuple


@dataclass(frozen=True)
class Error:
    lines: List[Tuple[int, str]]
    message: str


@dataclass(frozen=True)
class CheckedFile:
    filename: str
    errors: List[Error]


derive_statement_re = re.compile(r'#\[derive\((.*)\)\]')


def check_files(filenames: List[str]) -> List[CheckedFile]:
    return [_check_file(filename) for filename in filenames if _should_include_file(filename)]


def _check_file(filename: str) -> CheckedFile:
    errors = []

    with open(filename, 'r') as file:
        errors.extend([*_check_for_errors_in_file(file)])

    return CheckedFile(filename=filename, errors=errors)


def _check_for_errors_in_file(file) -> List[Error]:
    errors = []

    for (number, line) in enumerate(file, start=1):
        if _line_contains_box_new(line) and not _is_comment_line(line):
            errors.append(
                Error(lines=[(number, line.rstrip())], message='Use box syntax instead of Box::new'))

        if _line_contains_not_alphabetically_sorted_derive(line):
            errors.append(
                Error(lines=[(number, line.rstrip())], message='Sort derived traits alphabetically'))

    return errors


def _line_contains_box_new(line: str) -> bool:
    return 'Box::new' in line


def _is_comment_line(line: str) -> bool:
    return line.strip().startswith('//')


def _line_contains_not_alphabetically_sorted_derive(line: str) -> bool:
    match = derive_statement_re.match(line)

    if match is None:
        return False

    derives = [x.strip().lower() for x in match.group(1).split(',')]

    return derives is not sorted(derives)


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
