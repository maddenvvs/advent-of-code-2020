use super::solution::{Error as ChallengeErr, Solution};
use std::collections::HashSet;

type Group<'a> = Vec<&'a str>;

fn parse_answers(answers_text: &str) -> Vec<Group> {
    answers_text
        .split("\n\n")
        .map(|g| g.split_whitespace().collect())
        .collect()
}

fn sum_of_unique_answers(group: &Group) -> i32 {
    group
        .iter()
        .fold(&mut HashSet::with_capacity(26), |acc, el| {
            acc.extend(el.chars());
            acc
        })
        .len() as i32
}

fn sum_of_common_answers(group: &Group) -> i32 {
    let unique_answers: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    group
        .iter()
        .fold(unique_answers, |acc, el| {
            acc.intersection(&el.chars().collect::<HashSet<char>>())
                .copied()
                .collect()
        })
        .len() as i32
}

fn total_sum_of_unique_answers(groups: &[Group]) -> i32 {
    groups.iter().map(sum_of_unique_answers).sum()
}

fn total_sum_of_common_answers(groups: &[Group]) -> i32 {
    groups.iter().map(sum_of_common_answers).sum()
}

pub struct Day06 {}

impl Solution for Day06 {
    fn first_task(&self, answers_text: &str) -> Result<String, ChallengeErr> {
        let parsed_answers = parse_answers(&answers_text);

        Ok(total_sum_of_unique_answers(&parsed_answers).to_string())
    }

    fn second_task(&self, answers_text: &str) -> Result<String, ChallengeErr> {
        let parsed_answers = parse_answers(&answers_text);

        Ok(total_sum_of_common_answers(&parsed_answers).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_total_sum_of_unique_answers() {
        let test_answers = "abc

a
b
c

ab
ac

a
a
a
a

b";

        let parsed_answers = parse_answers(test_answers);
        assert_eq!(total_sum_of_unique_answers(&parsed_answers), 11);
    }

    #[test]
    fn example_total_sum_of_common_answers() {
        let test_answers = "abc

a
b
c

ab
ac

a
a
a
a

b";

        let parsed_answers = parse_answers(test_answers);
        assert_eq!(total_sum_of_common_answers(&parsed_answers), 6);
    }
}
