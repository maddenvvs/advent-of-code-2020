from aoc2020.solutions.day_15 import parse_numbers, find_2020th_number, find_30000000th_number


def test_2020() -> None:
    numbers_game_tests = (("0,3,6", 436), ("1,3,2", 1), ("2,1,3", 10),
                          ("1,2,3", 27), ("2,3,1", 78), ("3,2,1", 438), ("3,1,2", 1836))

    for numbers_text, number_2020 in numbers_game_tests:
        assert find_2020th_number(parse_numbers(numbers_text)) == number_2020


def test_30000000():
    assert find_30000000th_number(parse_numbers("0,3,6")) == 175594
