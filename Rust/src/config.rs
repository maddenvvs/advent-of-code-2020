use clap::Clap;
use std::error::Error;

#[derive(Clap)]
#[clap()]
pub struct Opts {
    /// Day number (between 1 and 25 inclusive)
    #[clap(validator(is_day_valid))]
    pub day: u8,

    /// Input file containing input data provided by Advent of Code
    pub file: Option<String>,
}

fn is_day_valid(val: &str) -> Result<(), String> {
    match val.parse::<u8>() {
        Ok(val) if val < 26 && val > 0 => Ok(()),
        _ => Err(String::from(
            "Day must be between 1 and 25 (both inclusive)",
        )),
    }
}

impl Opts {
    pub fn new() -> Result<Opts, Box<dyn Error>> {
        Ok(Opts::parse())
    }
}
