from __future__ import annotations

from collections import defaultdict
from functools import reduce
from itertools import product
from typing import Dict, Iterator, List, Set, Tuple

from .solution import Solution

Point = Tuple


class ConwayCube:
    __slots__ = "_state", "_buffer", "_neighbours", "_buffer_neighbours", "_dimensions"

    _state: Set[Point]
    _buffer: Set[Point]
    _neighbours: Dict[Point, int]
    _buffer_neighbours: Dict[Point, int]
    _dimensions: int

    def __init__(self, initial_slice: List[str], dimensions: int):
        self._state = set()
        self._neighbours = defaultdict(int)
        self._buffer = set()
        self._buffer_neighbours = defaultdict(int)
        self._dimensions = dimensions

        for y, row in enumerate(initial_slice):
            for x, v in enumerate(row):
                if v == "#":
                    point = [x, y] + [0] * (dimensions - 2)
                    self.add_point(tuple(point))

    def neighbour_cubes(self, point: Point) -> Iterator[Point]:
        for diff in product([-1, 0, 1], repeat=self._dimensions):
            if not any(diff):
                continue

            yield tuple(c + dc for c, dc in zip(point, diff))

    def add_point(self, point: Point, buffer=False):
        if not buffer:
            buffer, neighbours = self._state, self._neighbours
        else:
            buffer, neighbours = self._buffer, self._buffer_neighbours

        buffer.add(point)

        for neighbour in self.neighbour_cubes(point):
            neighbours[neighbour] += 1

    def simulate_step(self) -> int:
        self._buffer.clear()
        self._buffer_neighbours.clear()

        total = 0

        for point, active_neighbours in self._neighbours.items():
            if (point in self._state and 2 <= active_neighbours <= 3) or active_neighbours == 3:
                total += 1
                self.add_point(point, buffer=True)

        self._state, self._buffer = self._buffer, self._state
        self._neighbours, self._buffer_neighbours = self._buffer_neighbours, self._neighbours

        return total

    def active_cubes_after_six_steps(self):
        return reduce(lambda l, r: self.simulate_step(), range(6), 0)

    @ classmethod
    def from_str(cls, initial_text: str, dimensions: int) -> ConwayCube:
        return ConwayCube(initial_text.splitlines(), dimensions)


class Day17(Solution):

    def first_task(self, cube_text: str) -> str:
        cube = ConwayCube.from_str(cube_text, 3)
        return str(cube.active_cubes_after_six_steps())

    def second_task(self, cube_text: str) -> str:
        cube = ConwayCube.from_str(cube_text, 4)
        return str(cube.active_cubes_after_six_steps())
