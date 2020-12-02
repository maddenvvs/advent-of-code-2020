use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PASSWORD_PATTERN: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
}

fn current_policy(first_number: i32, second_number: i32, letter: char, password: &str) -> bool {
    return (password.chars().nth((first_number - 1) as usize).unwrap() == letter)
        ^ (password.chars().nth((second_number - 1) as usize).unwrap() == letter);
}

fn old_policy(first_number: i32, second_number: i32, letter: char, password: &str) -> bool {
    let occurences_of_letter = password.chars().filter(|&ch| ch == letter).count() as i32;

    return first_number <= occurences_of_letter && occurences_of_letter <= second_number;
}

fn is_password_valid(
    password: &str,
    password_policy: &dyn Fn(i32, i32, char, &str) -> bool,
) -> bool {
    let captures = PASSWORD_PATTERN.captures(password).unwrap();

    let first_number: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
    let second_number: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
    let letter: char = captures.get(3).unwrap().as_str().chars().nth(0).unwrap();
    let password_text: &str = captures.get(4).unwrap().as_str();

    return password_policy(first_number, second_number, letter, password_text);
}

fn count_old_valid_passwords(passwords: &Vec<&str>) -> i32 {
    return passwords
        .iter()
        .filter(|password| is_password_valid(password, &old_policy))
        .count() as i32;
}

fn count_current_valid_passwords(passwords: &Vec<&str>) -> i32 {
    return passwords
        .iter()
        .filter(|password| is_password_valid(password, &current_policy))
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

    let passwords_list: Vec<&str> = passwords_file_content.split('\n').collect();

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
