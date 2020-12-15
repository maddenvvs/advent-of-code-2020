pub struct ChallengeErr {}

pub trait Challenge {
    fn first_part(&self, input: &str) -> Result<String, ChallengeErr>;

    fn second_part(&self, input: &str) -> Result<String, ChallengeErr>;
}

pub struct NoSolution;

impl Challenge for NoSolution {
    fn first_part(&self, _: &str) -> Result<String, ChallengeErr> {
        Ok(String::from("No solution yet!"))
    }

    fn second_part(&self, _: &str) -> Result<String, ChallengeErr> {
        Ok(String::from("No solution yet!"))
    }
}
