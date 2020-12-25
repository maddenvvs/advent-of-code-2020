from aoc2020.solutions.day_25 import find_loop_size, find_encryption_key


def test_find_loop_size():
    assert find_loop_size(5764801) == 8
    assert find_loop_size(17807724) == 11


def test_first_task():
    assert find_encryption_key(5764801, 17807724) == 14897079
