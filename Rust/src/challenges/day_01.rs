use super::challenge::{Challenge, ChallengeErr};
use std::collections::HashSet;

const NEW_YEAR: i32 = 2020;

pub struct Solution {}

impl Solution {
    fn parse_input(input: &str) -> Vec<i32> {
        input.lines().map(|s| s.parse().unwrap()).collect()
    }

    fn find_product_of_two_numbers_equal_to_2020(entities: &[i32]) -> Option<i32> {
        let mut seen = HashSet::new();

        for entity in entities {
            let candidate = NEW_YEAR - entity;
            if seen.contains(&candidate) {
                return Some(candidate * entity);
            }

            seen.insert(entity);
        }

        None
    }

    fn find_product_of_three_numbers_equal_to_2020(entities: &[i32]) -> Option<i32> {
        let mut entities_copy = entities.to_vec();
        entities_copy.sort_unstable();

        for f in 0..(entities.len() - 2) {
            let target_sum = NEW_YEAR - entities_copy[f];
            if target_sum <= 0 {
                continue;
            }

            let mut l = f + 1;
            let mut r = entities.len() - 1;

            while l < r {
                let temp_sum = entities_copy[l] + entities_copy[r];

                if temp_sum == target_sum {
                    return Some(entities_copy[f] * entities_copy[l] * entities_copy[r]);
                }

                if temp_sum < target_sum {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
        None
    }
}

impl Challenge for Solution {
    fn first_part(&self, input: &str) -> Result<String, ChallengeErr> {
        let entities = Solution::parse_input(input);

        Solution::find_product_of_two_numbers_equal_to_2020(&entities)
            .map(|v| v.to_string())
            .ok_or(ChallengeErr {})
    }

    fn second_part(&self, input: &str) -> Result<String, ChallengeErr> {
        let entities = Solution::parse_input(input);

        Solution::find_product_of_three_numbers_equal_to_2020(&entities)
            .map(|v| v.to_string())
            .ok_or(ChallengeErr {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_find_product_of_two_numbers() {
        let test_entries = [1721, 979, 366, 299, 675, 1456];

        assert_eq!(
            Solution::find_product_of_two_numbers_equal_to_2020(&test_entries),
            Some(514579)
        );
    }

    #[test]
    fn example_find_product_of_three_numbers() {
        let test_entries = [1721, 979, 366, 299, 675, 1456];

        assert_eq!(
            Solution::find_product_of_three_numbers_equal_to_2020(&test_entries),
            Some(241861950)
        );
    }
}
