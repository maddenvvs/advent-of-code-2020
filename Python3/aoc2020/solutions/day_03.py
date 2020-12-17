from functools import reduce
from operator import mul
from typing import Iterable, List, Sequence, Tuple

from .solution import Solution

Slope = Tuple[int, int]

SLOPES: List[Slope] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]


def product(*args: int) -> int:
    return reduce(mul, args, 1)


def count_trees_on_slope(_map: Sequence[str], slope: Slope) -> int:
    dc, dr = slope
    height, width = len(_map), len(_map[0])
    cr, cc = 0, 0
    trees = 0

    while cr < height:
        if _map[cr][cc] == "#":
            trees += 1

        cr += dr
        cc = (cc + dc) % width

    return trees


def find_product_of_trees_on_slopes(_map: Sequence[str], slopes: Iterable[Slope]) -> int:
    return product(*(count_trees_on_slope(_map, slope) for slope in slopes))


class Day03(Solution):

    def first_task(self, map_text: str) -> str:
        original_map = [line.strip() for line in map_text.splitlines()]

        return str(count_trees_on_slope(original_map, SLOPES[1]))

    def second_task(self, map_text: str) -> str:
        original_map = [line.strip() for line in map_text.splitlines()]

        return str(find_product_of_trees_on_slopes(original_map, SLOPES))
