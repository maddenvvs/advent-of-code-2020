#![warn(clippy::all)]

use std::fs;

struct Slope {
    dx: i8,
    dy: i8,
}

fn count_trees_on_slope(map: &[&str], slope: &Slope) -> u32 {
    let height = map.len();
    let width = map[0].chars().count();
    let (dc, dr) = (slope.dx as usize, slope.dy as usize);
    let mut cr = 0;
    let mut cc = 0;
    let mut trees = 0;

    while cr < height {
        if map[cr].chars().nth(cc).unwrap() == '#' {
            trees += 1;
        }

        cr += dr;
        cc = (cc + dc) % width;
    }

    trees
}

fn find_product_of_trees_on_slopes(map: &[&str], slopes: &[Slope]) -> u32 {
    let mut result = 1;

    for slope in slopes {
        result *= count_trees_on_slope(map, slope);
    }

    result
}

fn task_tests(slopes: &[Slope]) {
    let test_map = [
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];

    assert_eq!(count_trees_on_slope(&test_map, &slopes[1]), 7);
    assert_eq!(find_product_of_trees_on_slopes(&test_map, slopes), 336);
}

fn run_tasks(slopes: &[Slope]) {
    let file_content = fs::read_to_string("input/day-3.input").expect("Missing area map file");

    let area_map: Vec<&str> = file_content.lines().collect();

    println!("Day 3-1: {}", count_trees_on_slope(&area_map, &slopes[1]));
    println!(
        "Day 3-2: {}",
        find_product_of_trees_on_slopes(&area_map, slopes)
    );
}

fn main() {
    let slopes = [
        Slope { dx: 1, dy: 1 },
        Slope { dx: 3, dy: 1 },
        Slope { dx: 5, dy: 1 },
        Slope { dx: 7, dy: 1 },
        Slope { dx: 1, dy: 2 },
    ];

    task_tests(&slopes);
    run_tasks(&slopes);
}
