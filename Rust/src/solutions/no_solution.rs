use super::solution::{Error, Solution};

pub struct NoSolution;

impl Solution for NoSolution {
    fn first_task(&self, _: &str) -> Result<String, Error> {
        Ok(String::from("No solution yet!"))
    }

    fn second_task(&self, _: &str) -> Result<String, Error> {
        Ok(String::from("No solution yet!"))
    }
}
