use super::solution::{Error, Solution};

const MODULO: u64 = 20201227;

fn parse_public_keys(keys_text: &str) -> (u64, u64) {
    let mut lines = keys_text.lines();

    (
        lines.next().unwrap().parse().unwrap(),
        lines.next().unwrap().parse().unwrap(),
    )
}

fn find_loop_size(public_key: u64) -> u64 {
    let mut subject_number = 7_u64;
    let mut loop_size = 1_u64;

    while subject_number != public_key {
        subject_number = (subject_number * 7_u64) % MODULO;
        loop_size += 1;
    }

    loop_size
}

fn find_encryption_key(door_public_key: u64, key_public_key: u64) -> u64 {
    let loop_size = find_loop_size(door_public_key);

    (0..loop_size - 1).fold(key_public_key, |acc, _| acc * key_public_key % MODULO)
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
        assert_eq!(find_loop_size(5764801_u64), 8_u64);
        assert_eq!(find_loop_size(17807724_u64), 11_u64);
    }

    #[test]
    fn test_first_task() {
        assert_eq!(find_encryption_key(5764801_u64, 17807724_u64), 14897079_u64);
    }
}
