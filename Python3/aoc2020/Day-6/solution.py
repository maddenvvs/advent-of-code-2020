from string import ascii_lowercase
from typing import Iterable, List

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


def test_tasks() -> None:
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
    assert total_sum_of_common_answers(test_answers) == 6


def run_tasks() -> None:
    with open("answers.txt") as answers_file:
        answers = parse_answers(answers_file.read())

        print(f"Day 6-1: {total_sum_of_unique_answers(answers)}")
        print(f"Day 6-2: {total_sum_of_common_answers(answers)}")


def main() -> None:
    test_tasks()
    run_tasks()


if __name__ == "__main__":
    main()
