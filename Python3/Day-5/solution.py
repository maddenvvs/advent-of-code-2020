from __future__ import annotations

from typing import Iterable


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


def test_tasks() -> None:
    test_seat_pass = [
        ("FBFBBFFRLR", Seat.from_row_and_col(44, 5), 357),
        ("BFFFBBFRRR", Seat.from_row_and_col(70, 7), 567),
        ("FFFBBBFRRR", Seat.from_row_and_col(14, 7), 119),
        ("BBFFBBFRLL", Seat.from_row_and_col(102, 4), 820)
    ]

    for seat_pass, seat, seat_id in test_seat_pass:
        found_seat = Seat.from_seat_pass(seat_pass)

        assert found_seat == seat
        assert found_seat.seat_id == seat_id


def run_tasks() -> None:
    with open("boarding_passes.txt") as passes_file:
        passes = (line.strip() for line in passes_file)
        seats = [Seat.from_seat_pass(seat) for seat in passes]

        print(f"Day 5-1: {find_highest_seat_id(seats)}")
        print(f"Day 5-2: {find_missing_seat_id(seats)}")


def main() -> None:
    test_tasks()
    run_tasks()


if __name__ == "__main__":
    main()
