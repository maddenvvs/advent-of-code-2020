use super::solution::{Error as ChallengeErr, Solution};
use std::cmp::{Ordering, PartialOrd};
use std::ops::Add;

const NEW_YEAR: i32 = 2020;

fn find_two_indexes_with_given_sum_helper<T>(
    sorted: &[T],
    mut start: usize,
    mut end: usize,
    target: T,
) -> Option<(usize, usize)>
where
    T: Add<Output = T> + PartialOrd + Copy,
{
    use Ordering::*;

    while start < end {
        match (sorted[start] + sorted[end]).partial_cmp(&target) {
            Some(Equal) => return Some((start, end)),
            Some(Less) => start += 1,
            Some(Greater) => end -= 1,
            None => return None,
        };
    }

    None
}

fn find_two_indexes_with_given_sum<T>(sorted: &[T], target: T) -> Option<(usize, usize)>
where
    T: Add<Output = T> + PartialOrd + Copy,
{
    find_two_indexes_with_given_sum_helper(sorted, 0, sorted.len() - 1, target)
}

pub struct Day01 {}

impl Day01 {
    fn parse_input(input: &str) -> Vec<i32> {
        input.lines().map(|s| s.parse().unwrap()).collect()
    }

    fn find_product_of_two_numbers_equal_to_2020(entities: &[i32]) -> Option<i32> {
        let mut entities_copy = entities.to_vec();
        entities_copy.sort_unstable();

        find_two_indexes_with_given_sum(&entities_copy, NEW_YEAR)
            .map(|(f, s)| entities_copy[f] * entities_copy[s])
    }

    fn find_product_of_three_numbers_equal_to_2020(entities: &[i32]) -> Option<i32> {
        let mut entities_copy = entities.to_vec();
        entities_copy.sort_unstable();

        for f in 0..(entities.len() - 2) {
            let target_sum = NEW_YEAR - entities_copy[f];

            if let Some((l, r)) = find_two_indexes_with_given_sum(&entities_copy, target_sum) {
                return Some(entities_copy[f] * entities_copy[l] * entities_copy[r]);
            }
        }

        None
    }
}

impl Solution for Day01 {
    fn first_task(&self, input: &str) -> Result<String, ChallengeErr> {
        let entities = Day01::parse_input(input);

        Day01::find_product_of_two_numbers_equal_to_2020(&entities)
            .map(|v| v.to_string())
            .ok_or(ChallengeErr {})
    }

    fn second_task(&self, input: &str) -> Result<String, ChallengeErr> {
        let entities = Day01::parse_input(input);

        Day01::find_product_of_three_numbers_equal_to_2020(&entities)
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
            Day01::find_product_of_two_numbers_equal_to_2020(&test_entries),
            Some(514579)
        );
    }

    #[test]
    fn example_find_product_of_three_numbers() {
        let test_entries = [1721, 979, 366, 299, 675, 1456];

        assert_eq!(
            Day01::find_product_of_three_numbers_equal_to_2020(&test_entries),
            Some(241861950)
        );
    }
}
