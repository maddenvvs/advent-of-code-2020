from aoc2020.solutions.day_02 import count_old_valid_passwords, count_current_valid_passwords


def test_first_task() -> None:
    test_passwords = [
        "1-3 a: abcde",
        "1-3 b: cdefg",
        "2-9 c: ccccccccc"
    ]

    assert count_old_valid_passwords(test_passwords) == 2
    assert count_current_valid_passwords(test_passwords) == 1


def test_second_task() -> None:
    test_passwords = [
        "1-3 a: abcde",
        "1-3 b: cdefg",
        "2-9 c: ccccccccc"
    ]

    assert count_current_valid_passwords(test_passwords) == 1
