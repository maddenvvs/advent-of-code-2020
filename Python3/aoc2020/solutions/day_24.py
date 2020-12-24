from __future__ import annotations

from collections import defaultdict
from functools import reduce
from typing import Dict, List, Set, Tuple

from .solution import Solution

Point = Tuple[int, int]


def add_point(floor: Set[Point], neighbours: Dict[Point, int], point: Point):
    floor.add(point)

    x, y = point
    for dx, dy in ((-2, 0), (2, 0), (-1, -3), (-1, 3), (1, -3), (1, 3)):
        neighbours[(x + dx, y + dy)] += 1


def result_position(instructions_list: str) -> Point:
    x, y = 0, 0

    idx = 0
    while idx < len(instructions_list):
        if instructions_list[idx] == "e":
            x -= 2
            idx += 1
        elif instructions_list[idx] == "w":
            x += 2
            idx += 1
        elif instructions_list[idx] == "s":
            if instructions_list[idx+1] == "e":
                x -= 1
            elif instructions_list[idx+1] == "w":
                x += 1
            y -= 3
            idx += 2
        elif instructions_list[idx] == "n":
            if instructions_list[idx+1] == "e":
                x -= 1
            elif instructions_list[idx+1] == "w":
                x += 1
            y += 3
            idx += 2

    return x, y


class TileFloor:
    __slots__ = "floor", "buffer", "neighbours", "buffer_neighbours"

    floor: Set[Point]
    buffer: Set[Point]
    neighbours: Dict[Point, int]
    buffer_neighbours: Dict[Point, int]

    def __init__(self, floor: Set[Point]) -> None:
        self.floor = set()
        self.buffer = set()
        self.neighbours = defaultdict(int)
        self.buffer_neighbours = defaultdict(int)

        for point in floor:
            add_point(self.floor, self.neighbours, point)

    @property
    def black_tiles(self) -> int:
        return len(self.floor)

    def simulate_day(self) -> int:
        self.buffer.clear()
        self.buffer_neighbours.clear()
        black_tiles = 0

        for point, active_neighbours in self.neighbours.items():
            is_black = point in self.floor
            if (is_black and 1 <= active_neighbours <= 2) or (not is_black and active_neighbours == 2):
                black_tiles += 1

                add_point(self.buffer, self.buffer_neighbours, point)

        self.floor, self.buffer = self.buffer, self.floor
        self.neighbours, self.buffer_neighbours = self.buffer_neighbours, self.neighbours
        return black_tiles

    @classmethod
    def from_instructions(cls, instructions_text: str) -> TileFloor:
        tiles: Dict[Point, int] = defaultdict(int)

        for instructions in instructions_text.splitlines():
            tiles[result_position(instructions)] ^= 1

        return cls(set(p for p, v in tiles.items() if v == 1))


def black_tiles_after_n_days(tile_floor: TileFloor, days: int) -> int:
    return reduce(lambda acc, el: tile_floor.simulate_day(), range(days), tile_floor.black_tiles)


class Day24(Solution):

    def first_task(self, instructions_text: str) -> str:
        tile_floor = TileFloor.from_instructions(instructions_text)

        return str(tile_floor.black_tiles)

    def second_task(self, instructions_text: str) -> str:
        tile_floor = TileFloor.from_instructions(instructions_text)

        return str(black_tiles_after_n_days(tile_floor, 100))
