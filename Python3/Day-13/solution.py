from __future__ import annotations

from functools import reduce
from itertools import combinations
from math import ceil, gcd
from operator import mul
from typing import Iterable, List, Sequence, Tuple

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


def test_tasks() -> None:
    test_notes_1_text = """939
7,13,x,x,59,x,31,19"""

    test_timestamp, test_buses = parse_notes(test_notes_1_text)

    assert find_earliest_bus_estimation(test_timestamp, test_buses) == 295
    assert find_gold_coin_timestamp(test_buses) == 1068781

    test_gold_timestamps = [("17,x,13,19", 3417), ("67,7,59,61", 754018), (
        "67,x,7,59,61", 779210), ("67,7,x,59,61", 1261476), ("1789,37,47,1889", 1202161486)]

    for buses_text, gold_timestamp in test_gold_timestamps:
        assert find_gold_coin_timestamp(
            parse_buses(buses_text)) == gold_timestamp


def run_tasks() -> None:
    with open("notes.txt") as notes_file:
        timestamp, buses = parse_notes(notes_file.read())

        print(f"Day 13-1: {find_earliest_bus_estimation(timestamp, buses)}")
        print(f"Day 13-2: {find_gold_coin_timestamp(buses)}")


def main() -> None:
    test_tasks()
    run_tasks()


if __name__ == "__main__":
    main()
