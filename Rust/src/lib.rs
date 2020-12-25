#![warn(clippy::all)]

pub mod config;
use config::Opts;

mod solutions;
use solutions::*;

use std::error::Error;
use std::fs;
use std::io::{self, Read};

fn available_solutions() -> Vec<Box<dyn Solution>> {
    vec![
        Box::new(Day01 {}),
        Box::new(Day02 {}),
        Box::new(Day03 {}),
        Box::new(Day04 {}),
        Box::new(Day05 {}),
        Box::new(Day06 {}),
        Box::new(Day07 {}),
        Box::new(Day08 {}),
        Box::new(Day09 {}),
        Box::new(Day10 {}),
        Box::new(Day11 {}),
        Box::new(Day12 {}),
        Box::new(Day13 {}),
        Box::new(Day14 {}),
        Box::new(Day15 {}),
        Box::new(Day16 {}),
        Box::new(Day17 {}),
        Box::new(Day18 {}),
        Box::new(Day19 {}),
        Box::new(Day20 {}),
        Box::new(Day21 {}),
        Box::new(Day22 {}),
        Box::new(Day23 {}),
        Box::new(Day24 {}),
        Box::new(Day25 {}),
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
