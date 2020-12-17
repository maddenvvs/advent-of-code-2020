from typing import List, Tuple


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


def test_tasks() -> None:
    test_adapters_1_text = """16
10
15
5
1
11
7
19
6
12
4"""

    test_adapters_1 = parse_adapters(test_adapters_1_text)
    assert find_jolt_differences(test_adapters_1) == (0, 7, 0, 5)
    assert count_number_of_ways_to_connect(test_adapters_1) == 8

    test_adapters_2_text = """28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"""

    test_adapters_2 = parse_adapters(test_adapters_2_text)
    assert find_jolt_differences(test_adapters_2) == (0, 22, 0, 10)
    assert count_number_of_ways_to_connect(test_adapters_2) == 19208


def run_tasks() -> None:
    with open("adapters.txt") as adapters_file:
        adapters = parse_adapters(adapters_file.read())

        print(f"Day 10-1: {find_product_of_jolt_differences(adapters)}")
        print(f"Day 10-2: {count_number_of_ways_to_connect(adapters)}")


def main() -> None:
    test_tasks()
    run_tasks()


if __name__ == "__main__":
    main()
