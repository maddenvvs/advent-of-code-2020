#![warn(clippy::all)]

use std::fs;

struct Point {
    x: i8,
    y: i8,
}

fn count_trees_on_slope(map: &[&str], slope: &Point) -> u32 {
    let height = map.len();
    let width = map[0].chars().count();
    let (dc, dr) = (slope.x as usize, slope.y as usize);
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

fn find_product_of_trees_on_slopes(map: &[&str], slopes: &[Point]) -> u32 {
    slopes
        .iter()
        .map(|s| count_trees_on_slope(map, s))
        .product()
}

fn task_tests(slopes: &[Point]) {
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

fn run_tasks(slopes: &[Point]) {
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
        Point { x: 1, y: 1 },
        Point { x: 3, y: 1 },
        Point { x: 5, y: 1 },
        Point { x: 7, y: 1 },
        Point { x: 1, y: 2 },
    ];

    task_tests(&slopes);
    run_tasks(&slopes);
}
