from aoc2020.solutions.day_10 import parse_adapters, find_jolt_differences, count_number_of_ways_to_connect


def test_tasks_1() -> None:
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


def test_tasks_2() -> None:
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
