use super::solution::{Error as ChallengeErr, Solution};
use std::str::FromStr;

struct NumbersGame {
    numbers: Vec<usize>,
    seen: Vec<usize>,
    last_move: usize,
    last_number: usize,
}

impl FromStr for NumbersGame {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NumbersGame {
            numbers: s.split(',').map(|el| el.parse().unwrap()).collect(),
            seen: vec![0; 30_000_001],
            last_move: 0,
            last_number: 0,
        })
    }
}

struct GameState {
    number: usize,
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

        if self.last_move < self.numbers.len() {
            self.seen[self.last_number] = self.last_move;
            self.last_number = self.numbers[self.last_move];
        } else {
            let last_seen_at = if self.seen[self.last_number] == 0 {
                0
            } else {
                self.last_move - self.seen[self.last_number]
            };
            self.seen[self.last_number] = self.last_move;
            self.last_number = last_seen_at;
        }

        self.last_move += 1;

        Some(result)
    }
}

impl NumbersGame {
    fn find_number_at_move(&mut self, at_move: usize) -> usize {
        self.nth(at_move - 1).unwrap().number
    }
}

pub struct Day15 {}

impl Solution for Day15 {
    fn first_task(&self, numbers_text: &str) -> Result<String, ChallengeErr> {
        Ok(numbers_text
            .parse::<NumbersGame>()
            .unwrap()
            .find_number_at_move(2020)
            .to_string())
    }

    fn second_task(&self, numbers_text: &str) -> Result<String, ChallengeErr> {
        Ok(numbers_text
            .parse::<NumbersGame>()
            .unwrap()
            .find_number_at_move(30000000)
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_number_at_move_2020_extended() {
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
    }

    #[test]
    fn test_find_number_at_move_30000000() {
        assert_eq!(
            "0,3,6"
                .parse::<NumbersGame>()
                .unwrap()
                .find_number_at_move(30000000),
            175594
        );
    }
}
