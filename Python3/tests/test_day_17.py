from aoc2020.solutions.day_17 import ConwayCube


def test_3D() -> None:
    test_initial_state = """.#.
..#
###"""

    test_cube = ConwayCube.from_str(test_initial_state, 3)
    for active_cubes in [11, 21, 38]:
        assert test_cube.simulate_step() == active_cubes


def test_4D():
    test_initial_state = """.#.
..#
###"""

    test_cube = ConwayCube.from_str(test_initial_state, 4)
    for active_cubes in [29, 60]:
        assert test_cube.simulate_step() == active_cubes
