use std::error::Error as ErrorTrait;
use std::fmt::{Display, Error as fmtError, Formatter};

pub trait Solution {
    fn first_task(&self, input: &str) -> Result<String, Error>;

    fn second_task(&self, input: &str) -> Result<String, Error>;
}

#[derive(Debug)]
pub struct Error {}

impl Display for Error {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), fmtError> {
        Ok(())
    }
}

impl ErrorTrait for Error {}
