from aoc2020.solutions.day_06 import parse_answers, total_sum_of_unique_answers, total_sum_of_common_answers


def test_first_task() -> None:
    test_answers_str = """abc

a
b
c

ab
ac

a
a
a
a

b
"""
    test_answers = parse_answers(test_answers_str)

    assert total_sum_of_unique_answers(test_answers) == 11


def test_second_task() -> None:
    test_answers_str = """abc

a
b
c

ab
ac

a
a
a
a

b
"""
    test_answers = parse_answers(test_answers_str)

    assert total_sum_of_common_answers(test_answers) == 6
