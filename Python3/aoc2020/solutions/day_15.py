from itertools import dropwhile
from typing import Generator, List, Sequence, Tuple

from .solution import Solution


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


class Day15(Solution):

    def first_task(self, numbers_text: str) -> str:
        numbers = parse_numbers(numbers_text)

        return str(find_2020th_number(numbers))

    def second_task(self, numbers_text: str) -> str:
        numbers = parse_numbers(numbers_text)

        return str(find_30000000th_number(numbers))
