from aoc2020.solutions.day_14 import parse_program, find_memory_values_sum_v1, find_memory_values_sum_v2


def test_tasks() -> None:
    test_program_1_text = """mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"""

    test_program_1 = parse_program(test_program_1_text)
    assert find_memory_values_sum_v1(test_program_1) == 165


def test_tasks_2() -> None:
    test_program_2_text = """mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"""

    test_program_2 = parse_program(test_program_2_text)
    assert find_memory_values_sum_v2(test_program_2) == 208
