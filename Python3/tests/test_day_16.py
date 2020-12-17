from aoc2020.solutions.day_16 import Notes


def test_sum_of_invalid_values() -> None:
    test_notes_1_txt = """class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"""

    test_notes_1 = Notes.from_str(test_notes_1_txt)
    assert test_notes_1.sum_of_invalid_values_in_nearby_tickets() == 71


def test_field_order():
    test_notes_2_txt = """class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"""

    test_notes_2 = Notes.from_str(test_notes_2_txt)
    assert test_notes_2.find_field_order() == ["row", "class", "seat"]
