use std::fs;

type Slope = (u8, u8);
type OriginalSlopes = [Slope; 5];

fn count_trees_on_slope(map: &Vec<&str>, slope: Slope) -> u32 {
    let height = map.len();
    let width = map[0].chars().count();
    let (dc, dr) = slope;
    let (dc, dr) = (dc as usize, dr as usize);
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

fn find_product_of_trees_on_slopes(map: &Vec<&str>, slopes: &OriginalSlopes) -> u32 {
    let mut result = 1;

    for slope in slopes {
        result *= count_trees_on_slope(map, *slope);
    }

    result
}

fn task_tests(slopes: &OriginalSlopes) {
    let test_map: Vec<&str> = vec![
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

    assert_eq!(count_trees_on_slope(&test_map, slopes[1]), 7);
    assert_eq!(find_product_of_trees_on_slopes(&test_map, slopes), 336);
}

fn run_tasks(slopes: &OriginalSlopes) {
    let file_content = fs::read_to_string("input/day-3.input").expect("Missing area map file");

    let area_map: Vec<&str> = file_content.lines().collect();

    println!("Day 3-1: {}", count_trees_on_slope(&area_map, slopes[1]));
    println!(
        "Day 3-2: {}",
        find_product_of_trees_on_slopes(&area_map, slopes)
    );
}

fn main() {
    let slopes: OriginalSlopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    task_tests(&slopes);
    run_tasks(&slopes);
}
