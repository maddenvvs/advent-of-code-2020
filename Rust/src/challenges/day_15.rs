use super::challenge::{Challenge, ChallengeErr};
use std::collections::HashMap;
use std::str::FromStr;

struct NumbersGame {
    numbers: Vec<i32>,
    seen: HashMap<i32, i32>,
    last_move: i32,
    last_number: i32,
}

impl FromStr for NumbersGame {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NumbersGame {
            numbers: s.split(',').map(|el| el.parse().unwrap()).collect(),
            seen: HashMap::new(),
            last_move: 0,
            last_number: 0,
        })
    }
}

struct GameState {
    number: i32,
}

impl Iterator for NumbersGame {
    type Item = GameState;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last_move == 0 {
            self.last_move = 1;
            self.last_number = self.numbers[0];
        }

        let result = GameState {
            number: self.last_number,
        };

        if (self.last_move as usize) < self.numbers.len() {
            self.seen.insert(self.last_number, self.last_move);
            self.last_number = self.numbers[self.last_move as usize];
        } else {
            let last_seen_at = match self.seen.get(&self.last_number) {
                Some(last_seen_at) => self.last_move - last_seen_at,
                None => 0,
            };
            self.seen.insert(self.last_number, self.last_move);
            self.last_number = last_seen_at;
        }

        self.last_move += 1;

        Some(result)
    }
}

impl NumbersGame {
    fn find_number_at_move(&mut self, at_move: i32) -> i32 {
        self.nth((at_move - 1) as usize).unwrap().number
    }
}

fn test_tasks() {
    let numbers_game_tests = [
        ("0,3,6", 436),
        ("1,3,2", 1),
        ("2,1,3", 10),
        ("1,2,3", 27),
        ("2,3,1", 78),
        ("3,2,1", 438),
        ("3,1,2", 1836),
    ];

    for (numbers_text, number_2020) in &numbers_game_tests {
        let mut numbers_game: NumbersGame = numbers_text.parse().unwrap();
        assert_eq!(numbers_game.find_number_at_move(2020), *number_2020);
    }

    assert_eq!(
        "0,3,6"
            .parse::<NumbersGame>()
            .unwrap()
            .find_number_at_move(30000000),
        175594
    );
}

pub struct Solution {}

impl Challenge for Solution {
    fn first_part(&self, numbers_text: &str) -> Result<String, ChallengeErr> {
        Ok(numbers_text
            .parse::<NumbersGame>()
            .unwrap()
            .find_number_at_move(2020)
            .to_string())
    }

    fn second_part(&self, numbers_text: &str) -> Result<String, ChallengeErr> {
        Ok(numbers_text
            .parse::<NumbersGame>()
            .unwrap()
            .find_number_at_move(30000000)
            .to_string())
    }

    fn run_tests(&self) {
        test_tasks();
    }
}
