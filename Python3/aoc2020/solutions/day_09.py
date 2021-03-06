from collections import defaultdict, deque
from itertools import product
from typing import Deque, Dict, Generator, List, Optional

from .solution import Solution


def parse_cypher(cypher_text: str) -> List[int]:
    return [int(n) for n in cypher_text.splitlines()]


def xmax_cypher(preamble_len: int) -> Generator[bool, int, None]:
    unique_sums: Dict[int, int] = defaultdict(int)
    window: Deque[int] = deque()

    for _ in range(preamble_len):
        new_number = yield True

        for other in window:
            unique_sums[other + new_number] += 1
        window.append(new_number)

        yield True

    while True:
        new_number = yield True

        occurences = unique_sums[new_number]

        if occurences > 0:
            number_to_throw_away = window.popleft()

            for other in window:
                unique_sums[new_number + other] += 1
                unique_sums[number_to_throw_away + other] -= 1
            window.append(new_number)

            yield True
        else:
            yield False


def find_first_incorrect_cypher_number(cypher: List[int], preamble_len: int = 25) -> Optional[int]:
    cypher_gen = xmax_cypher(preamble_len)
    next(cypher_gen)

    for num in cypher:
        val = cypher_gen.send(num)
        if not val:
            return num

        next(cypher_gen)

    return None


def find_encryption_weakness_value(cypher: List[int], target_value: int) -> Optional[int]:
    l, temp_sum = 0, 0
    min_deque: Deque[int] = deque()
    max_deque: Deque[int] = deque()

    for r, next_num in enumerate(cypher):
        temp_sum += next_num

        while temp_sum > target_value:
            num_to_remove = cypher[l]

            if min_deque and min_deque[0] == num_to_remove:
                min_deque.popleft()

            if max_deque and max_deque[0] == num_to_remove:
                max_deque.popleft()

            temp_sum -= num_to_remove
            l += 1

        while min_deque and min_deque[-1] > next_num:
            min_deque.pop()
        min_deque.append(next_num)

        while max_deque and max_deque[-1] < next_num:
            max_deque.pop()
        max_deque.append(next_num)

        if temp_sum == target_value:
            return min_deque[0] + max_deque[0]

    return None


def find_encryption_weakness_of(cypher: List[int], preamble_len: int = 25) -> Optional[int]:
    incorrect_number = find_first_incorrect_cypher_number(cypher, preamble_len)

    if incorrect_number is None:
        return None

    return find_encryption_weakness_value(cypher, incorrect_number)


class Day09(Solution):

    def first_task(self, cypher_text: str) -> str:
        cypher = parse_cypher(cypher_text)

        return str(find_first_incorrect_cypher_number(cypher))

    def second_task(self, cypher_text: str) -> str:
        cypher = parse_cypher(cypher_text)

        return str(find_encryption_weakness_of(cypher))
