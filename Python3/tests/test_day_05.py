from aoc2020.solutions.day_05 import Seat


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
