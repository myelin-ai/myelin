#!/usr/bin/env python3

import sys
import matplotlib.pyplot as plt
from typing import List


def _parse_point(point: str):
    parts = point.split(',')
    return (float(parts[0]), float(parts[1]))


def _parse_plot(line: str):
    items = line.split('\t')
    label = items[0]
    points = [_parse_point(point) for point in items[1:]]
    return (label, points)


for line in sys.stdin:
    (label, points) = _parse_plot(line)
    x_axis = [x for [x, _] in points]
    y_axis = [y for [_, y] in points]
    plt.plot(x_axis, y_axis, label=label)

plt.legend()
plt.plasma()
plt.ylabel('Membrane Potential (µV)')
plt.xlabel('Time (ms)')
plt.show()
