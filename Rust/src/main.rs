#![warn(clippy::all)]

mod challenges;

use challenges::*;
use clap::Clap;
use std::error::Error;
use std::fs;

#[derive(Clap)]
#[clap(version = "0.1.0")]
struct Opts {
    /// Day number (between 1 and 25 inclusive)
    #[clap(validator(is_day_valid))]
    day: u8,

    /// Input file containing input data provided by Advent of Code
    #[clap(short('f'), long("file"), value_name("file"))]
    input_file: Option<String>,
}

fn is_day_valid(val: &str) -> Result<(), String> {
    match val.parse::<u8>() {
        Ok(val) if val < 26 && val > 0 => Ok(()),
        _ => Err(String::from(
            "Day must be between 1 and 25 (both inclusive)",
        )),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let solutions: Vec<Box<dyn challenge::Challenge>> = vec![
        Box::new(day_01::Solution {}),
        Box::new(day_02::Solution {}),
        Box::new(day_03::Solution::new()),
        Box::new(day_04::Solution {}),
        Box::new(day_05::Solution {}),
        Box::new(day_06::Solution {}),
        Box::new(day_07::Solution {}),
        Box::new(day_08::Solution {}),
        Box::new(day_09::Solution {}),
        Box::new(day_10::Solution {}),
        Box::new(day_11::Solution {}),
        Box::new(day_12::Solution {}),
        Box::new(day_13::Solution {}),
        Box::new(day_14::Solution {}),
        Box::new(challenge::NoSolution),
        Box::new(challenge::NoSolution),
        Box::new(challenge::NoSolution),
        Box::new(challenge::NoSolution),
        Box::new(challenge::NoSolution),
        Box::new(challenge::NoSolution),
        Box::new(challenge::NoSolution),
        Box::new(challenge::NoSolution),
        Box::new(challenge::NoSolution),
        Box::new(challenge::NoSolution),
        Box::new(challenge::NoSolution),
    ];

    let opts: Opts = Opts::parse();

    let filename = match opts.input_file {
        Some(val) => val,
        None => format!("input/day-{}.input", opts.day),
    };

    let problem_input = match fs::read_to_string(&filename) {
        Err(val) => panic!("Error reading file '{}': {}", &filename, val),
        Ok(input) => input,
    };
    let solution = &solutions[(opts.day - 1) as usize];

    solution.run_tests();
    println!(
        "Day {}-1: {}",
        opts.day,
        solution
            .first_part(&problem_input)
            .unwrap_or_else(|_| "Error!!!".to_string())
    );
    println!(
        "Day {}-2: {}",
        opts.day,
        solution
            .second_part(&problem_input)
            .unwrap_or_else(|_| "Error!!!".to_string())
    );

    Ok(())
}
