from functools import reduce
from operator import mul
from typing import Iterable, Sequence, Tuple


Slope = Tuple[int, int]


def product(*args: int) -> int:
    return reduce(mul, args, 1)


def count_trees_on_slope(_map: Sequence[str], slope: Slope) -> int:
    dc, dr = slope
    height, width = len(_map), len(_map[0])
    cr, cc = 0, 0
    trees = 0

    while cr < height:
        if _map[cr][cc] == "#":
            trees += 1

        cr += dr
        cc = (cc + dc) % width

    return trees


def find_product_of_trees_on_slopes(_map: Sequence[str], slopes: Iterable[Slope]) -> int:
    return product(*(count_trees_on_slope(_map, slope) for slope in slopes))


def tesk_tests(slopes: Sequence[Slope]) -> None:
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

    assert count_trees_on_slope(test_map, slopes[1]) == 7
    assert find_product_of_trees_on_slopes(test_map, slopes) == 336


def run_tasks(slopes: Sequence[Slope]) -> None:
    with open("map.txt") as map_file:
        original_map = [line.strip() for line in map_file]

        print(f"Day 3-1: {count_trees_on_slope(original_map, slopes[1])}")
        print(
            f"Day 3-2: {find_product_of_trees_on_slopes(original_map, slopes)}")


def main() -> None:
    slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]

    tesk_tests(slopes)
    run_tasks(slopes)


if __name__ == "__main__":
    main()
