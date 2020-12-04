#![warn(clippy::all)]

use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

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

fn task_tests() {
    let test_passwords = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

    assert_eq!(count_old_valid_passwords(&test_passwords), 2);
    assert_eq!(count_current_valid_passwords(&test_passwords), 1);
}

fn run_tasks() {
    let passwords_file_content =
        fs::read_to_string("input/day-2.input").expect("Missing passwords file");

    let passwords_list: Vec<&str> = passwords_file_content.lines().collect();

    println!("Day 2-1: {}", count_old_valid_passwords(&passwords_list));
    println!(
        "Day 2-2: {}",
        count_current_valid_passwords(&passwords_list)
    );
}

fn main() {
    task_tests();
    run_tasks();
}
