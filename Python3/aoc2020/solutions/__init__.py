from typing import Optional, List

from .solution import Solution
from .day_01 import Day01
from .day_02 import Day02
from .day_03 import Day03
from .day_04 import Day04
from .day_05 import Day05
from .day_06 import Day06
from .day_07 import Day07
from .day_08 import Day08
from .day_09 import Day09
from .day_10 import Day10
from .day_11 import Day11
from .day_12 import Day12
from .day_13 import Day13
from .day_14 import Day14
from .day_15 import Day15
from .day_16 import Day16
from .day_17 import Day17
from .day_18 import Day18
from .day_19 import Day19
from .day_20 import Day20
from .day_21 import Day21
from .day_22 import Day22
from .day_23 import Day23
from .day_24 import Day24

all_solutions: List = [Day01, Day02, Day03, Day04,
                       Day05, Day06, Day07, Day08,
                       Day09, Day10, Day11, Day12,
                       Day13, Day14, Day15, Day16,
                       Day17, Day18, Day19, Day20,
                       Day21, Day22, Day23, Day24]


def get_solution(day: int) -> Optional[Solution]:
    if day > len(all_solutions):
        return None

    return all_solutions[day-1]()
