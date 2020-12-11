from __future__ import annotations

from enum import IntEnum
from typing import Callable, List, Tuple

CELL_VARIATIONS = [".", "L", "#"]


class Cell(IntEnum):
    EMPTY = 0
    FREE = 1
    OCCUPIED = 2

    def __str__(self):
        return CELL_VARIATIONS[self]


class WaitingArea:
    __slots__ = ("_seats", "_m", "_n", "_buffer")

    ADJACENT_DIRECTIONS = ((0, 1), (1, 0), (-1, 0), (0, -1),
                           (1, 1), (-1, -1), (1, -1), (-1, 1))

    def __init__(self, seats: List[List[Cell]]):
        self._m = len(seats)
        self._n = len(seats[0])
        self._seats = seats
        self._buffer = [[Cell.FREE] * self._n for _ in range(self._m)]

    def adjacent_neighbours(self, r: int, c: int) -> List[Cell]:
        adjacent_seats = []

        for dr, dc in self.ADJACENT_DIRECTIONS:
            nr, nc = r + dr, c + dc
            if 0 <= nr < self._m and 0 <= nc < self._n and self._seats[nr][nc] != Cell.EMPTY:
                adjacent_seats.append(self._seats[nr][nc])

        return adjacent_seats

    def visible_seats_from(self, r: int, c: int) -> List[Cell]:
        seats = []

        for dr, dc in self.ADJACENT_DIRECTIONS:
            nr, nc = r + dr, c + dc
            while 0 <= nr < self._m and 0 <= nc < self._n:
                if self._seats[nr][nc] != Cell.EMPTY:
                    seats.append(self._seats[nr][nc])
                    break
                nr, nc = nr + dr, nc + dc

        return seats

    def intolerant_way(self, r: int, c: int) -> Cell:
        seat = self._seats[r][c]

        if seat == Cell.FREE and \
                sum(seat == Cell.OCCUPIED for seat in self.adjacent_neighbours(r, c)) == 0:
            return Cell.OCCUPIED

        elif seat == Cell.OCCUPIED and \
                sum(seat == Cell.OCCUPIED for seat in self.adjacent_neighbours(r, c)) >= 4:
            return Cell.FREE

        return seat

    def tolerant_way(self, r: int, c: int) -> Cell:
        seat = self._seats[r][c]

        if seat == Cell.FREE and \
                sum(seat == Cell.OCCUPIED for seat in self.visible_seats_from(r, c)) == 0:
            return Cell.OCCUPIED

        elif seat == Cell.OCCUPIED and \
                sum(seat == Cell.OCCUPIED for seat in self.visible_seats_from(r, c)) >= 5:
            return Cell.FREE

        return seat

    def simulate_step(self, seat_decider: Callable[[int, int], Cell]) -> Tuple[bool, int]:
        has_changes, occupied_seats = False, 0
        for i, row in enumerate(self._seats):
            for j, seat in enumerate(row):

                new_seat = seat_decider(i, j)
                if seat != new_seat:
                    has_changes = True
                if new_seat == Cell.OCCUPIED:
                    occupied_seats += 1
                self._buffer[i][j] = new_seat

        self._seats, self._buffer = self._buffer, self._seats

        return has_changes, occupied_seats

    def simulate_intolerant_step(self) -> Tuple[bool, int]:
        return self.simulate_step(self.intolerant_way)

    def simulate_tolerant_step(self) -> Tuple[bool, int]:
        return self.simulate_step(self.tolerant_way)

    def find_occupied_seats_in_equilibrium(self, seat_decider: Callable[[int, int], Cell]) -> int:
        has_changes, occupied_seats = self.simulate_step(seat_decider)
        while has_changes:
            has_changes, occupied_seats = self.simulate_step(seat_decider)
        return occupied_seats

    def intolerant_equilibrium(self) -> int:
        return self.find_occupied_seats_in_equilibrium(self.intolerant_way)

    def tolerant_equilibrium(self) -> int:
        return self.find_occupied_seats_in_equilibrium(self.tolerant_way)

    @classmethod
    def from_string(cls, seats_text: str) -> WaitingArea:
        return cls([[cls.parse_cell(c) for c in line] for line in seats_text.splitlines()])

    @staticmethod
    def parse_cell(cell_text: str) -> Cell:
        if cell_text == "#":
            return Cell.OCCUPIED
        if cell_text == "L":
            return Cell.FREE

        return Cell.EMPTY

    def __str__(self) -> str:
        return "\n".join("".join(map(str, line)) for line in self._seats)


def test_tasks():
    test_area_str = """L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"""

    test_area = WaitingArea.from_string(test_area_str)
    intolerant_simulation_steps = ["""#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##""", """#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##""", """#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##""", """#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##""", """#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##"""]

    for simulation_map in intolerant_simulation_steps:
        test_area.simulate_intolerant_step()
        assert str(test_area) == simulation_map

    assert test_area.intolerant_equilibrium() == 37

    test_visibility_1 = WaitingArea.from_string(""".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....""")
    assert test_visibility_1.visible_seats_from(4, 3) == [Cell.OCCUPIED] * 8

    test_visibility_2 = WaitingArea.from_string(""".............
.L.L.#.#.#.#.
.............""")
    assert test_visibility_2.visible_seats_from(1, 1) == [Cell.FREE]

    test_visibility_3 = WaitingArea.from_string(""".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.""")
    assert test_visibility_3.visible_seats_from(3, 3) == []

    tolerant_area = WaitingArea.from_string(test_area_str)
    tolerant_simulation_steps = ["""#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##""", """#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#""", """#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#""", """#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#""", """#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#""", """#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#"""]

    for simulation_map in tolerant_simulation_steps:
        tolerant_area.simulate_tolerant_step()
        assert str(tolerant_area) == simulation_map

    assert tolerant_area.tolerant_equilibrium() == 26


def run_tasks():
    with open("seats.txt") as seats_file:
        waiting_area_text = seats_file.read()

        print(
            f"Day 11-1: {WaitingArea.from_string(waiting_area_text).intolerant_equilibrium()}")

        print(
            f"Day 11-2: {WaitingArea.from_string(waiting_area_text).tolerant_equilibrium()}")


def main():
    test_tasks()
    run_tasks()


if __name__ == "__main__":
    main()
