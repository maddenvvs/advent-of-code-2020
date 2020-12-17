from aoc2020.solutions.day_13 import parse_buses, parse_notes, \
    find_earliest_bus_estimation, find_gold_coin_timestamp


def test_tasks() -> None:
    test_notes_1_text = """939
7,13,x,x,59,x,31,19"""

    test_timestamp, test_buses = parse_notes(test_notes_1_text)

    assert find_earliest_bus_estimation(test_timestamp, test_buses) == 295
    assert find_gold_coin_timestamp(test_buses) == 1068781


def test_gold_timestamps():
    test_gold_timestamps = [("17,x,13,19", 3417), ("67,7,59,61", 754018), (
        "67,x,7,59,61", 779210), ("67,7,x,59,61", 1261476), ("1789,37,47,1889", 1202161486)]

    for buses_text, gold_timestamp in test_gold_timestamps:
        assert find_gold_coin_timestamp(
            parse_buses(buses_text)) == gold_timestamp
