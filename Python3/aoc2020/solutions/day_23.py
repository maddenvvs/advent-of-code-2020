from __future__ import annotations

from typing import List, Tuple, Optional, cast

from .solution import Solution


def parse_cups(cups_text: str) -> List[int]:
    return [int(c, base=10) for c in cups_text]


def build_cups_buffer(cups: List[int], size: int) -> List[int]:
    buffer = [0] * (size + 1)

    last_num = cups[0]
    for i in range(1, size):
        buffer[last_num] = cups[i] if i < len(cups) else (i+1)
        last_num = buffer[last_num]

    buffer[last_num] = cups[0]

    return buffer


def simulate_game_classy_way(first_cup: int, buffer: List[int], moves: int) -> List[int]:
    curr_node = first_cup
    for _ in range(moves):
        a, b, c = buffer[curr_node], buffer[buffer[curr_node]
                                            ], buffer[buffer[buffer[curr_node]]]

        value_to_append_after = curr_node - 1
        if value_to_append_after == 0:
            value_to_append_after = len(buffer) - 1

        while value_to_append_after == a or value_to_append_after == b or value_to_append_after == c:
            value_to_append_after -= 1
            if value_to_append_after == 0:
                value_to_append_after = len(buffer) - 1

        buffer[curr_node] = buffer[c]
        buffer[c] = buffer[value_to_append_after]
        buffer[value_to_append_after] = a

        curr_node = buffer[curr_node]

    return buffer


def find_1_based_label(buffer: List[int]) -> str:
    res = []
    curr_val = 1
    for _ in range(len(buffer) - 2):
        res.append(buffer[curr_val])
        curr_val = buffer[curr_val]

    return "".join(map(str, res))


def count_1_based_label_after(cups: List[int], moves: int) -> str:
    buffer = build_cups_buffer(cups, len(cups))
    new_cups = simulate_game_classy_way(cups[0], buffer, moves)

    return find_1_based_label(new_cups)


def count_product_of_two_labels_after_1(cups: List[int]) -> int:
    buffer = build_cups_buffer(cups, 1_000_000)
    new_cups = simulate_game_classy_way(cups[0], buffer, 10_000_000)

    return new_cups[1] * new_cups[new_cups[1]]


class Day23(Solution):

    def first_task(self, cups_text: str) -> str:
        cups = parse_cups(cups_text)

        return str(count_1_based_label_after(cups, 100))

    def second_task(self, cups_text: str) -> str:
        cups = parse_cups(cups_text)

        return str(count_product_of_two_labels_after_1(cups))
