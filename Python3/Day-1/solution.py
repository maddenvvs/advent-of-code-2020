from functools import reduce
from operator import mul
from typing import Iterable, Sequence, Tuple


def product(*args: int) -> int:
    return reduce(mul, args, 1)


def find_three_entries_with_sum_2020(entries: Sequence[int]) -> Tuple[int, int, int]:
    entries = list(sorted(entries))

    for f in range(len(entries) - 2):
        needed_sum = 2020 - entries[f]
        if needed_sum <= 0:
            continue

        l, r = f + 1, len(entries) - 1
        while l < r:
            temp_sum = entries[l] + entries[r]
            if temp_sum == needed_sum:
                return entries[f], entries[l], entries[r]

            if temp_sum < needed_sum:
                l += 1
            else:
                r -= 1

    return (0, 0, 0)


def find_two_entries_with_sum_2020(entries: Iterable[int]) -> Tuple[int, int]:
    seen = set()
    for entry in entries:
        candidate = 2020 - entry
        if candidate in seen:
            return entry, candidate
        seen.add(entry)

    return (0, 0)


def first_task(entries: Iterable[int]) -> int:
    return product(*find_two_entries_with_sum_2020(entries))


def second_task(entries: Sequence[int]) -> int:
    return product(*find_three_entries_with_sum_2020(entries))


def task_tests() -> None:
    test_entries = [1721, 979, 366, 299, 675, 1456]

    assert first_task(test_entries) == 514579
    assert second_task(test_entries) == 241861950


def run_tasks() -> None:
    with open("report.txt") as report_file:
        report_entries = [int(l) for l in report_file]
        print(
            f"Product of two entries with sum 2020: {first_task(report_entries)}")
        print(
            f"Product of three entries with sum 2020: {second_task(report_entries)}")


def main() -> None:
    task_tests()
    run_tasks()


if __name__ == "__main__":
    main()
