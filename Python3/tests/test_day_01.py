from aoc2020.solutions.day_01 import first_task, second_task, parse_entries


def test_first_task():
    test_entries = parse_entries("""1721
979
366
299
675
1456""")

    assert first_task(test_entries) == 514579


def test_second_task():
    test_entries = parse_entries("""1721
979
366
299
675
1456""")

    assert second_task(test_entries) == 241861950
