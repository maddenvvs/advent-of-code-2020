from typing import List, Tuple

from .solution import Solution


def parse_adapters(adapters_text: str) -> List[int]:
    return [int(a) for a in adapters_text.splitlines()]


def prepare_devices(adapters: List[int]) -> List[int]:
    return [0] + sorted(adapters) + [max(adapters) + 3]


def find_jolt_differences(adapters: List[int]) -> Tuple[int, int, int, int]:
    devices = prepare_devices(adapters)
    diffs = [0, 0, 0, 0]

    for i in range(1, len(devices)):
        diffs[devices[i] - devices[i-1]] += 1

    return (diffs[0], diffs[1], diffs[2], diffs[3])


def find_product_of_jolt_differences(adapters: List[int]) -> int:
    _, ones, _, threes = find_jolt_differences(adapters)

    return ones * threes


def count_number_of_ways_to_connect(adapters: List[int]) -> int:
    devices = prepare_devices(adapters)
    ways = [0] * len(devices)
    ways[0] = 1

    for i in range(1, len(devices)):
        for j in range(i-1, max(-1, i-4), -1):
            if devices[i] - devices[j] < 4:
                ways[i] += ways[j]

    return ways[-1]


class Day10(Solution):

    def first_task(self, adapters_text: str) -> str:
        adapters = parse_adapters(adapters_text)

        return str(find_product_of_jolt_differences(adapters))

    def second_task(self, adapters_text: str) -> str:
        adapters = parse_adapters(adapters_text)

        return str(count_number_of_ways_to_connect(adapters))
