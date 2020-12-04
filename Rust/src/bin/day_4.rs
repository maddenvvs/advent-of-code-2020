#![warn(clippy::all)]

use std::collections::HashMap;
use std::fs;

fn is_number(num: &str, length: usize) -> bool {
    num.len() == length && num.chars().all(|ch| ch.is_ascii_digit())
}

fn is_number_between(num: &str, length: usize, from: i32, to: i32) -> bool {
    is_number(num, length) && {
        let number: i32 = num.parse().unwrap();

        from <= number && number <= to
    }
}

fn is_year_between(num: &str, from: i32, to: i32) -> bool {
    is_number_between(num, 4, from, to)
}

fn is_height_valid(value: &str) -> bool {
    match &value[value.len() - 2..] {
        "cm" => is_number_between(&value[..value.len() - 2], 3, 150, 193),
        "in" => is_number_between(&value[..value.len() - 2], 2, 59, 76),
        _ => false,
    }
}

fn is_hair_color_valid(value: &str) -> bool {
    value.len() == 7 && value.starts_with('#') && value.chars().skip(1).all(|ch| ch.is_digit(16))
}

fn is_eye_color_valid(value: &str) -> bool {
    matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn is_pid_valid(value: &str) -> bool {
    is_number(value, 9)
}

fn is_passport_valid_weak(passport: &HashMap<&str, &str>) -> bool {
    passport.len() == 8 || (passport.len() == 7 && !passport.contains_key("cid"))
}

fn is_field_valid(name: &str, value: &str) -> bool {
    match name {
        "byr" => is_year_between(value, 1920, 2002),
        "iyr" => is_year_between(value, 2010, 2020),
        "eyr" => is_year_between(value, 2020, 2030),
        "hgt" => is_height_valid(value),
        "hcl" => is_hair_color_valid(value),
        "ecl" => is_eye_color_valid(value),
        "pid" => is_pid_valid(value),
        "cid" => true,
        _ => false,
    }
}

fn is_passport_valid_strong(passport: &HashMap<&str, &str>) -> bool {
    is_passport_valid_weak(passport)
        && passport
            .iter()
            .all(|(name, value)| is_field_valid(name, value))
}

fn parse_password(password_entity: &str) -> HashMap<&str, &str> {
    password_entity
        .split_whitespace()
        .map(|e| e.trim().splitn(2, ':').collect::<Vec<&str>>())
        .map(|pair| (pair[0], pair[1]))
        .collect()
}

fn parse_passwords(passwords_text: &str) -> Vec<HashMap<&str, &str>> {
    passwords_text.split("\n\n").map(&parse_password).collect()
}

fn count_valid_passwords(
    passwords_text: &str,
    password_policy: &dyn Fn(&HashMap<&str, &str>) -> bool,
) -> i32 {
    parse_passwords(passwords_text)
        .iter()
        .filter(|p| password_policy(p))
        .count() as i32
}

fn count_passwords_with_weak_validation(passwords_text: &str) -> i32 {
    count_valid_passwords(passwords_text, &is_passport_valid_weak)
}

fn count_passwords_with_strong_validation(passwords_text: &str) -> i32 {
    count_valid_passwords(passwords_text, &is_passport_valid_strong)
}

fn test_tasks() {
    let test_passwords_file = String::from(
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in",
    );

    let test_strong_invalid_passwords = String::from(
        "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007",
    );

    let test_strong_valid_passwords = String::from(
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
    );

    assert_eq!(
        count_passwords_with_weak_validation(&test_passwords_file),
        2
    );
    assert_eq!(
        count_passwords_with_strong_validation(&test_strong_invalid_passwords),
        0
    );
    assert_eq!(
        count_passwords_with_strong_validation(&test_strong_valid_passwords),
        4
    );
}

fn run_tasks() {
    let passwords_file_content = fs::read_to_string("input/day-4.input").unwrap();

    println!(
        "Day 4-1: {}",
        count_passwords_with_weak_validation(&passwords_file_content)
    );
    println!(
        "Day 4-2: {}",
        count_passwords_with_strong_validation(&passwords_file_content)
    );
}

fn main() {
    test_tasks();
    run_tasks();
}
