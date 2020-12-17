from aoc2020.solutions.day_12 import parse_instructions, \
    simulate_instructions_with_rotation, \
    simulate_instructions_with_waypoint


def test_tasks() -> None:
    test_instructions_1_text = """F10
N3
F7
R90
F11"""

    test_instructions_1 = parse_instructions(test_instructions_1_text)
    assert simulate_instructions_with_rotation(test_instructions_1) == 25
    assert simulate_instructions_with_waypoint(test_instructions_1) == 286
