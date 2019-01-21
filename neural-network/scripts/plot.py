#!/usr/bin/env python3

import sys
import subprocess
import matplotlib.pyplot as plt
from typing import List


def _parse_point(point: str) -> (float, float):
    parts = point.split(',')
    x = float(parts[0])
    y = float(parts[1])
    return (x, y)


def _parse_plot(line: str) -> (str, str):
    items = line.split('\t')
    label = items[0]
    points = [_parse_point(point) for point in items[1:]]
    return (label, points)


def _draw_plot(lines: [str]):
    for line in lines:
        (label, points) = _parse_plot(line)
        x_axis = [x for [x, _] in points]
        y_axis = [y for [_, y] in points]
        plt.plot(x_axis, y_axis, label=label)

    plt.legend()
    plt.plasma()
    plt.ylabel('Membrane Potential (µV)')
    plt.xlabel('Time (ms)')
    plt.show()


if __name__ == "__main__":
    cargo_command = ['cargo', 'run', '--example', 'plot_neuron']
    result = subprocess.run(cargo_command, stdout=subprocess.PIPE, check=True)
    lines = result.stdout.decode('utf-8').splitlines()
    _draw_plot(lines)
