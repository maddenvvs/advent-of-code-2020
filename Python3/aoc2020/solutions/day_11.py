from __future__ import annotations

from enum import IntEnum
from typing import Callable, List, Tuple

from .solution import Solution

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


class Day11(Solution):

    def first_task(self, seats_text: str) -> str:
        return str(WaitingArea.from_string(seats_text).intolerant_equilibrium())

    def second_task(self, seats_text: str) -> str:
        return str(WaitingArea.from_string(seats_text).tolerant_equilibrium())
