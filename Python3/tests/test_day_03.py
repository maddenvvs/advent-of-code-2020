from aoc2020.solutions.day_03 import SLOPES, count_trees_on_slope, find_product_of_trees_on_slopes


def test_first_task():
    test_map = ["..##.......",
                "#...#...#..",
                ".#....#..#.",
                "..#.#...#.#",
                ".#...##..#.",
                "..#.##.....",
                ".#.#.#....#",
                ".#........#",
                "#.##...#...",
                "#...##....#",
                ".#..#...#.#"]

    assert count_trees_on_slope(test_map, SLOPES[1]) == 7


def test_second_task():
    test_map = ["..##.......",
                "#...#...#..",
                ".#....#..#.",
                "..#.#...#.#",
                ".#...##..#.",
                "..#.##.....",
                ".#.#.#....#",
                ".#........#",
                "#.##...#...",
                "#...##....#",
                ".#..#...#.#"]

    assert find_product_of_trees_on_slopes(test_map, SLOPES) == 336
