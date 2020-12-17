from __future__ import annotations

from functools import reduce
from itertools import combinations
from math import ceil, gcd
from operator import mul
from typing import Iterable, List, Sequence, Tuple

from .solution import Solution

BusInfo = Tuple[int, int]


def coprime(a, b):
    return gcd(a, b) == 1


def pairwise_coprime(numbers: Iterable[int]) -> bool:
    return all(coprime(a, b) for a, b in combinations(numbers, r=2))


def mod_inverse(n, mod):
    return pow(n, mod-2, mod)


def chinese_reminder_theorem_solver(r_i: Sequence[int], a_i: Sequence[int]) -> int:
    assert len(r_i) == len(a_i)
    assert pairwise_coprime(a_i)

    M = reduce(mul, a_i)
    M_i = [M // b for b in a_i]
    M_i_inv = [mod_inverse(m, a) for m, a in zip(M_i, a_i)]

    return sum(r*m*m_i for r, m, m_i in zip(r_i, M_i, M_i_inv)) % M


def parse_buses(buses_text: str) -> List[BusInfo]:
    return [(i, int(b, base=10)) for i, b in enumerate(buses_text.split(",")) if b != "x"]


def parse_notes(notes_text: str) -> Tuple[int, List[BusInfo]]:
    timestamp, buses = notes_text.splitlines()

    return int(timestamp, base=10), parse_buses(buses)


def find_earliest_bus_estimation(timestamp: int, buses: Iterable[BusInfo]) -> int:
    minutes_to_wait, earliest_bus = 10**9, 0

    for _, bus_id in buses:
        time_to_wait = ceil(timestamp / bus_id) * bus_id - timestamp

        if time_to_wait < minutes_to_wait:
            minutes_to_wait, earliest_bus = time_to_wait, bus_id

    return earliest_bus * minutes_to_wait


def find_gold_coin_timestamp(buses: Iterable[BusInfo]) -> int:
    a_i = [b for _, b in buses]
    r_i = [-i for i, _ in buses]

    return chinese_reminder_theorem_solver(r_i, a_i)


class Day13(Solution):

    def first_task(self, notes_text: str) -> str:
        timestamp, buses = parse_notes(notes_text)

        return str(find_earliest_bus_estimation(timestamp, buses))

    def second_task(self, notes_text: str) -> str:
        _, buses = parse_notes(notes_text)

        return str(find_gold_coin_timestamp(buses))
