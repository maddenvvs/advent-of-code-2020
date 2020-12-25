use super::solution::{Error, Solution};
use itertools::Itertools;

const MODULO: u64 = 20201227;

fn parse_public_keys(keys_text: &str) -> (u64, u64) {
    keys_text
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

fn transforms(subject: u64) -> impl Iterator<Item = u64> {
    itertools::iterate(subject, move |current| current * subject % MODULO)
}

fn find_loop_size(public_key: u64) -> usize {
    transforms(7).position(|v| v == public_key).unwrap()
}

fn find_encryption_key(door_public_key: u64, card_public_key: u64) -> u64 {
    let loop_size = find_loop_size(door_public_key);

    transforms(card_public_key).nth(loop_size).unwrap()
}

pub struct Day25 {}

impl Solution for Day25 {
    fn first_task(&self, keys_text: &str) -> Result<String, Error> {
        let (first_key, second_key) = parse_public_keys(keys_text);

        Ok(find_encryption_key(first_key, second_key).to_string())
    }

    fn second_task(&self, _: &str) -> Result<String, Error> {
        Ok(String::from("I did it!!!"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_loop_size() {
        assert_eq!(find_loop_size(5764801), 7);
        assert_eq!(find_loop_size(17807724), 10);
    }

    #[test]
    fn test_first_task() {
        assert_eq!(find_encryption_key(5764801, 17807724), 14897079);
    }
}
