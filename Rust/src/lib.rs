pub mod config;
use config::Opts;

mod challenges;
use challenges::no_solution::NoSolution;
use challenges::*;

use std::error::Error;
use std::fs;
use std::io::{self, Read};

fn available_solutions() -> Vec<Box<dyn challenge::Challenge>> {
    vec![
        Box::new(day_01::Solution {}),
        Box::new(day_02::Solution {}),
        Box::new(day_03::Solution {}),
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
        Box::new(day_15::Solution {}),
        Box::new(NoSolution),
        Box::new(NoSolution),
        Box::new(NoSolution),
        Box::new(NoSolution),
        Box::new(NoSolution),
        Box::new(NoSolution),
        Box::new(NoSolution),
        Box::new(NoSolution),
        Box::new(NoSolution),
        Box::new(NoSolution),
    ]
}

fn read_problem_input(filename: &Option<String>) -> io::Result<String> {
    match filename {
        Some(filename) => fs::read_to_string(filename),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}

pub fn run(opts: Opts) -> Result<(), Box<dyn Error>> {
    let solutions = available_solutions();

    let problem_input = read_problem_input(&opts.file)?;
    let solution = &solutions[(opts.day - 1) as usize];

    println!(
        "Day {}-1: {}",
        opts.day,
        solution.first_part(&problem_input)?
    );
    println!(
        "Day {}-2: {}",
        opts.day,
        solution.second_part(&problem_input)?
    );

    Ok(())
}
