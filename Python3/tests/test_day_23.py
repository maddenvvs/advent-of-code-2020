from aoc2020.solutions.day_23 import parse_cups, \
    build_cups_buffer, \
    count_1_based_label_after, \
    find_1_based_label, \
    count_product_of_two_labels_after_1


def test_1_based_label():
    test_cups_text = "389125467"

    test_cups = parse_cups(test_cups_text)
    test_buffer = build_cups_buffer(test_cups, len(test_cups))

    assert find_1_based_label(test_buffer) == "25467389"


def test_some_next_moves():
    test_cups_text = "389125467"

    test_cups = parse_cups(test_cups_text)

    for moves, label in [(1, "54673289"), (2, "32546789"), (3, "34672589")]:
        assert count_1_based_label_after(test_cups, moves) == label


def test_first_task():
    test_cups_text = "389125467"

    test_cups = parse_cups(test_cups_text)

    assert count_1_based_label_after(test_cups, 10) == "92658374"
    assert count_1_based_label_after(test_cups, 100) == "67384529"


def test_second_task():
    test_cups_text = "389125467"

    test_cups = parse_cups(test_cups_text)

    assert count_product_of_two_labels_after_1(test_cups) == 149245887792
