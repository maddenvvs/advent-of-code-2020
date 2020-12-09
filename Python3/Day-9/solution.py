from collections import defaultdict, deque
from itertools import product
from typing import Deque, Dict, Generator, List, Optional


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
    min_queue: Deque[int] = deque()
    max_queue: Deque[int] = deque()

    for r, next_num in enumerate(cypher):
        temp_sum += next_num

        while temp_sum > target_value:
            num_to_remove = cypher[l]

            if min_queue and min_queue[0] == num_to_remove:
                min_queue.popleft()

            if max_queue and max_queue[0] == num_to_remove:
                max_queue.popleft()

            temp_sum -= num_to_remove
            l += 1

        while min_queue and min_queue[-1] > next_num:
            min_queue.pop()
        min_queue.append(next_num)

        while max_queue and max_queue[-1] < next_num:
            max_queue.pop()
        max_queue.append(next_num)

        if temp_sum == target_value:
            return min_queue[0] + max_queue[0]

    return None


def find_encryption_weakness_of(cypher: List[int], preamble_len: int = 25) -> Optional[int]:
    incorrect_number = find_first_incorrect_cypher_number(cypher, preamble_len)

    if incorrect_number is None:
        return None

    return find_encryption_weakness_value(cypher, incorrect_number)


def test_tasks() -> None:
    test_cypher_text = """35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"""

    test_cypher = parse_cypher(test_cypher_text)

    assert find_first_incorrect_cypher_number(
        test_cypher, preamble_len=5) == 127

    assert find_encryption_weakness_of(test_cypher, preamble_len=5) == 62


def run_tasks() -> None:
    with open("cypher.txt") as cypher_file:
        cypher = parse_cypher(cypher_file.read())

        print(f"Day 9-1: {find_first_incorrect_cypher_number(cypher)}")
        print(f"Day 9-2: {find_encryption_weakness_of(cypher)}")


def main() -> None:
    test_tasks()
    run_tasks()


if __name__ == "__main__":
    main()
