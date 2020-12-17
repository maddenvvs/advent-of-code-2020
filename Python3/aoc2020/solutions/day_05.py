from __future__ import annotations

from typing import Iterable

from .solution import Solution


class Seat:
    __slots__ = "seat_id"

    def __init__(self, seat_id: int):
        self.seat_id = seat_id

    def __eq__(self, other) -> bool:
        return isinstance(other, Seat) and self.seat_id == other.seat_id

    @classmethod
    def from_seat_pass(cls, seat_pass: str) -> Seat:
        seat_id_str = seat_pass.translate({
            ord("F"): "0",
            ord("L"): "0",
            ord("R"): "1",
            ord("B"): "1"
        })

        return cls(seat_id=int(seat_id_str, base=2))

    @classmethod
    def from_row_and_col(cls, row: int, col: int) -> Seat:
        return cls(row * 8 + col)


def find_missing_seat_id(seats: Iterable[Seat]) -> int:
    last_seat_id = None

    for seat_id in sorted(seat.seat_id for seat in seats):

        if last_seat_id and last_seat_id + 2 == seat_id:
            return last_seat_id + 1

        last_seat_id = seat_id

    return -1


def find_highest_seat_id(seats: Iterable[Seat]) -> int:
    return max(seats, key=lambda x: x.seat_id).seat_id


class Day05(Solution):

    def first_task(self, passes_text: str) -> str:
        passes = (line.strip() for line in passes_text.splitlines())
        seats = [Seat.from_seat_pass(seat) for seat in passes]

        return str(find_highest_seat_id(seats))

    def second_task(self, passes_text: str) -> str:
        passes = (line.strip() for line in passes_text.splitlines())
        seats = [Seat.from_seat_pass(seat) for seat in passes]

        return str(find_missing_seat_id(seats))
