#![warn(clippy::all)]
use std::process;

use aoc2020::config::Opts;

fn main() {
    let opts: Opts = Opts::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    if let Err(err) = aoc2020::run(opts) {
        eprintln!("Application error: {}", err);

        process::exit(1);
    }
}
