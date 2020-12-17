from string import ascii_lowercase
from typing import Iterable, List

from .solution import Solution

Group = Iterable[str]


def parse_answers(answers_text: str) -> List[Group]:
    return [group.split() for group in answers_text.split("\n\n")]


def sum_of_unique_answers(group: Group) -> int:
    unique_answers = set()

    for answers in group:
        for answer in answers:
            unique_answers.add(answer)

    return len(unique_answers)


def total_sum_of_unique_answers(groups: Iterable[Group]) -> int:
    return sum(sum_of_unique_answers(group) for group in groups)


def sum_of_common_answers(group: Group) -> int:
    answers = set(ascii_lowercase)

    for answer_line in group:
        answers.intersection_update(set(answer_line))

    return len(answers)


def total_sum_of_common_answers(groups: Iterable[Group]) -> int:
    return sum(sum_of_common_answers(group) for group in groups)


class Day06(Solution):

    def first_task(self, answers_text: str) -> str:
        answers = parse_answers(answers_text)

        return str(total_sum_of_unique_answers(answers))

    def second_task(self, answers_text: str) -> str:
        answers = parse_answers(answers_text)

        return str(total_sum_of_common_answers(answers))
