use super::solution::{Error as ChallengeErr, Solution};

static SLOPES: [Point; 5] = [
    Point { x: 1, y: 1 },
    Point { x: 3, y: 1 },
    Point { x: 5, y: 1 },
    Point { x: 7, y: 1 },
    Point { x: 1, y: 2 },
];

struct Point {
    x: i8,
    y: i8,
}

pub struct Day03 {}

impl Day03 {
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
            .map(|s| Day03::count_trees_on_slope(map, s))
            .product()
    }
}

impl Solution for Day03 {
    fn first_task(&self, input: &str) -> Result<String, ChallengeErr> {
        let area_map: Vec<&str> = input.lines().collect();

        Ok(Day03::count_trees_on_slope(&area_map, &SLOPES[1]).to_string())
    }

    fn second_task(&self, input: &str) -> Result<String, ChallengeErr> {
        let area_map: Vec<&str> = input.lines().collect();

        Ok(Day03::find_product_of_trees_on_slopes(&area_map, &SLOPES).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_count_trees_on_slope() {
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

        assert_eq!(Day03::count_trees_on_slope(&test_map, &SLOPES[1]), 7);
    }

    #[test]
    fn example_find_product_of_trees_on_slopes() {
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

        assert_eq!(
            Day03::find_product_of_trees_on_slopes(&test_map, &SLOPES),
            336
        );
    }
}
