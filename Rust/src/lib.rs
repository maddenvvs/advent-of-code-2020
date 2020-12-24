#![warn(clippy::all)]

pub mod config;
use config::Opts;

mod solutions;
use solutions::no_solution::NoSolution;
use solutions::*;

use std::error::Error;
use std::fs;
use std::io::{self, Read};

fn available_solutions() -> Vec<Box<dyn solution::Solution>> {
    vec![
        Box::new(day_01::Day01 {}),
        Box::new(day_02::Day02 {}),
        Box::new(day_03::Day03 {}),
        Box::new(day_04::Day04 {}),
        Box::new(day_05::Day05 {}),
        Box::new(day_06::Day06 {}),
        Box::new(day_07::Day07 {}),
        Box::new(day_08::Day08 {}),
        Box::new(day_09::Day09 {}),
        Box::new(day_10::Day10 {}),
        Box::new(day_11::Day11 {}),
        Box::new(day_12::Day12 {}),
        Box::new(day_13::Day13 {}),
        Box::new(day_14::Day14 {}),
        Box::new(day_15::Day15 {}),
        Box::new(day_16::Day16 {}),
        Box::new(day_17::Day17 {}),
        Box::new(day_18::Day18 {}),
        Box::new(day_19::Day19 {}),
        Box::new(day_20::Day20 {}),
        Box::new(day_21::Day21 {}),
        Box::new(day_22::Day22 {}),
        Box::new(day_23::Day23 {}),
        Box::new(day_24::Day24 {}),
        Box::new(NoSolution),
    ]
}

fn read_problem_input(filename: &str) -> io::Result<String> {
    match filename {
        "-" => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
        filename => fs::read_to_string(filename),
    }
}

pub fn run(opts: Opts) -> Result<(), Box<dyn Error>> {
    let problem_input = read_problem_input(&opts.file)?;

    let solutions = available_solutions();
    let solution = &solutions[(opts.day - 1) as usize];

    println!(
        "Day {}-1: {}",
        opts.day,
        solution.first_task(&problem_input)?
    );
    println!(
        "Day {}-2: {}",
        opts.day,
        solution.second_task(&problem_input)?
    );

    Ok(())
}
