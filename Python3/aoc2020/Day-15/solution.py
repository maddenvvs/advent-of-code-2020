from itertools import dropwhile
from typing import Generator, List, Sequence, Tuple


def parse_numbers(numbers_text: str) -> List[int]:
    return [int(i, base=10) for i in numbers_text.split(",")]


def numbers_game(numbers: Sequence[int]) -> Generator[Tuple[int, int], None, None]:
    seen = {}
    last_move, last_number = 1, numbers[0]

    for i in range(1, len(numbers)):
        yield last_move, last_number
        seen[last_number] = last_move
        last_number, last_move = numbers[i], i + 1

    while True:
        yield last_move, last_number

        last_seen_at = seen.get(last_number, 0)
        seen[last_number] = last_move
        last_number = (last_move - last_seen_at) if last_seen_at else 0

        last_move += 1


def find_number_at_move(numbers: Sequence[int], move: int) -> int:
    return next(dropwhile(lambda t: t[0] < move, numbers_game(numbers)))[1]


def find_2020th_number(numbers: Sequence[int]) -> int:
    return find_number_at_move(numbers, 2020)


def find_30000000th_number(numbers: Sequence[int]) -> int:
    return find_number_at_move(numbers, 30000000)


def test_tasks() -> None:
    numbers_game_tests = (("0,3,6", 436), ("1,3,2", 1), ("2,1,3", 10),
                          ("1,2,3", 27), ("2,3,1", 78), ("3,2,1", 438), ("3,1,2", 1836))

    for numbers_text, number_2020 in numbers_game_tests:
        assert find_2020th_number(parse_numbers(numbers_text)) == number_2020

    assert find_30000000th_number(parse_numbers("0,3,6")) == 175594


def run_tasks() -> None:
    with open("numbers.txt") as numbers_file:
        numbers = parse_numbers(numbers_file.read())

        print(f"Day 15-1: {find_2020th_number(numbers)}")
        print(f"Day 15-2: {find_30000000th_number(numbers)}")


def main() -> None:
    test_tasks()
    run_tasks()


if __name__ == "__main__":
    main()
