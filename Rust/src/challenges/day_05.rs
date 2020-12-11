use super::challenge::{Challenge, ChallengeErr};

#[derive(PartialEq, Debug)]
struct Seat {
    seat_id: i32,
}

impl Seat {
    fn from_seat_pass(seat_pass: &str) -> Seat {
        let seat_id = seat_pass
            .chars()
            .map(|ch| match ch {
                'F' | 'L' => 0,
                'B' | 'R' => 1,
                ch => panic!("Unexpected seat pass character: {}", ch),
            })
            .fold(0, |acc, el| acc * 2 + el);

        Seat { seat_id }
    }

    fn from_row_and_col(row: i32, col: i32) -> Seat {
        Seat {
            seat_id: row * 8 + col,
        }
    }
}

fn find_max_seat_id(seats: &[Seat]) -> Option<i32> {
    seats.iter().map(|s| s.seat_id).max()
}

fn find_missing_seat_id(seats: &[Seat]) -> Option<i32> {
    let mut seat_ids: Vec<i32> = seats.iter().map(|s| s.seat_id).collect();
    seat_ids.sort_unstable();

    let mut last_value = -1;
    for val in seat_ids.iter() {
        if last_value != -1 && last_value + 2 == *val {
            return Some(last_value + 1);
        }
        last_value = *val;
    }

    None
}

fn parse_seats(input: &str) -> Vec<Seat> {
    input.split_whitespace().map(Seat::from_seat_pass).collect()
}

pub struct Solution {}

impl Challenge for Solution {
    fn first_part(&self, input: &str) -> Result<String, ChallengeErr> {
        let available_seats = parse_seats(input);

        find_max_seat_id(&available_seats)
            .map(|s| Ok(s.to_string()))
            .unwrap_or(Err(ChallengeErr {}))
    }

    fn second_part(&self, input: &str) -> Result<String, ChallengeErr> {
        let available_seats = parse_seats(input);

        find_missing_seat_id(&available_seats)
            .map(|s| Ok(s.to_string()))
            .unwrap_or(Err(ChallengeErr {}))
    }

    fn run_tests(&self) {
        let test_seat_pass = [
            ("FBFBBFFRLR", Seat::from_row_and_col(44, 5), 357),
            ("BFFFBBFRRR", Seat::from_row_and_col(70, 7), 567),
            ("FFFBBBFRRR", Seat::from_row_and_col(14, 7), 119),
            ("BBFFBBFRLL", Seat::from_row_and_col(102, 4), 820),
        ];

        for (seat_pass, expected_seat, seat_id) in &test_seat_pass {
            let found_seat = Seat::from_seat_pass(seat_pass);

            assert_eq!(&found_seat, expected_seat);
            assert_eq!(found_seat.seat_id, *seat_id);
        }
    }
}
