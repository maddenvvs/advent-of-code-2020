use super::solution::{Error as ChallengeErr, Solution};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PASSWORD_PATTERN: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
}

struct PasswordDefinition<'a> {
    first_number: i32,
    second_number: i32,
    letter: char,
    password: &'a str,
}

impl PasswordDefinition<'_> {
    fn from(password_text: &str) -> PasswordDefinition {
        let captures = PASSWORD_PATTERN.captures(password_text).unwrap();

        let first_number: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let second_number: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let letter: char = captures.get(3).unwrap().as_str().chars().next().unwrap();
        let password: &str = captures.get(4).unwrap().as_str();

        PasswordDefinition {
            first_number,
            second_number,
            letter,
            password,
        }
    }

    fn align_with_old_policy(&self) -> bool {
        let occurences_of_letter = self
            .password
            .chars()
            .filter(|&ch| ch == self.letter)
            .count() as i32;

        self.first_number <= occurences_of_letter && occurences_of_letter <= self.second_number
    }

    fn align_with_current_policy(&self) -> bool {
        (self
            .password
            .chars()
            .nth((self.first_number - 1) as usize)
            .unwrap()
            == self.letter)
            ^ (self
                .password
                .chars()
                .nth((self.second_number - 1) as usize)
                .unwrap()
                == self.letter)
    }
}

pub struct Day02 {}

impl Day02 {
    fn count_old_valid_passwords(passwords: &[&str]) -> i32 {
        return passwords
            .iter()
            .map(|password| PasswordDefinition::from(password))
            .filter(|def| def.align_with_old_policy())
            .count() as i32;
    }

    fn count_current_valid_passwords(passwords: &[&str]) -> i32 {
        return passwords
            .iter()
            .map(|password| PasswordDefinition::from(password))
            .filter(|def| def.align_with_current_policy())
            .count() as i32;
    }
}

impl Solution for Day02 {
    fn first_task(&self, input: &str) -> Result<String, ChallengeErr> {
        let passwords_list: Vec<&str> = input.lines().collect();

        Ok(Day02::count_old_valid_passwords(&passwords_list).to_string())
    }

    fn second_task(&self, input: &str) -> Result<String, ChallengeErr> {
        let passwords_list: Vec<&str> = input.lines().collect();

        Ok(Day02::count_current_valid_passwords(&passwords_list).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_count_old_valid_passwords() {
        let test_passwords = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

        assert_eq!(Day02::count_old_valid_passwords(&test_passwords), 2);
    }

    #[test]
    fn example_count_current_valid_passwords() {
        let test_passwords = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

        assert_eq!(Day02::count_current_valid_passwords(&test_passwords), 1);
    }
}
