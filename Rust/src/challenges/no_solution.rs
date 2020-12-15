use super::challenge::{Challenge, Error};

pub struct NoSolution;

impl Challenge for NoSolution {
    fn first_part(&self, _: &str) -> Result<String, Error> {
        Ok(String::from("No solution yet!"))
    }

    fn second_part(&self, _: &str) -> Result<String, Error> {
        Ok(String::from("No solution yet!"))
    }
}
