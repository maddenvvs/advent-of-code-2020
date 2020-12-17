from functools import reduce
from operator import mul
from typing import Iterable, Sequence, Tuple, List

from .solution import Solution

NEW_YEAR = 2020


def parse_entries(entries_text: str) -> List[int]:
    return [int(l) for l in entries_text.splitlines()]


def product(*args: int) -> int:
    return reduce(mul, args, 1)


def find_three_entries_with_sum_2020(entries: Sequence[int]) -> Tuple[int, int, int]:
    entries = list(sorted(entries))

    for f in range(len(entries) - 2):
        needed_sum = NEW_YEAR - entries[f]
        if needed_sum <= 0:
            continue

        l, r = f + 1, len(entries) - 1
        while l < r:
            temp_sum = entries[l] + entries[r]
            if temp_sum == needed_sum:
                return entries[f], entries[l], entries[r]

            if temp_sum < needed_sum:
                l += 1
            else:
                r -= 1

    return (0, 0, 0)


def find_two_entries_with_sum_2020(entries: Iterable[int]) -> Tuple[int, int]:
    seen = set()
    for entry in entries:
        candidate = NEW_YEAR - entry
        if candidate in seen:
            return entry, candidate
        seen.add(entry)

    return (0, 0)


def first_task(entries: Iterable[int]) -> int:
    return product(*find_two_entries_with_sum_2020(entries))


def second_task(entries: Sequence[int]) -> int:
    return product(*find_three_entries_with_sum_2020(entries))


class Day01(Solution):

    def first_task(self, entries_text: str) -> str:
        entries = parse_entries(entries_text)

        return str(first_task(entries))

    def second_task(self, entries_text: str) -> str:
        entries = parse_entries(entries_text)

        return str(second_task(entries))
