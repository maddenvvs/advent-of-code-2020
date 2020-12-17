use super::solution::{Error, Solution};
use itertools::iproduct;
use std::collections::{HashMap, HashSet};
use std::mem;

struct ConwayCube {
    dimensions: usize,
    state: HashSet<Vec<i32>>,
    buffer: HashSet<Vec<i32>>,
    neighbours: HashMap<Vec<i32>, i32>,
    neighbours_buffer: HashMap<Vec<i32>, i32>,
}

impl ConwayCube {
    fn from_str(s: &str, dimensions: usize) -> ConwayCube {
        let mut cube = ConwayCube {
            state: HashSet::new(),
            buffer: HashSet::new(),
            neighbours: HashMap::new(),
            neighbours_buffer: HashMap::new(),
            dimensions,
        };

        for (y, line) in s.lines().enumerate() {
            for (x, v) in line.chars().enumerate() {
                if v == '#' {
                    let mut point = vec![0; dimensions];
                    point[0] = x as i32;
                    point[1] = y as i32;

                    add_point(
                        &mut cube.state,
                        &mut cube.neighbours,
                        cube.dimensions,
                        &point,
                    );
                }
            }
        }

        cube
    }

    fn simulate_step(&mut self) -> usize {
        self.buffer.clear();
        self.neighbours_buffer.clear();
        let mut total = 0;

        for (point, &active_neighbours) in self.neighbours.iter() {
            if (self.state.contains(point) && 2 <= active_neighbours && active_neighbours <= 3)
                || active_neighbours == 3
            {
                total += 1;

                add_point(
                    &mut self.buffer,
                    &mut self.neighbours_buffer,
                    self.dimensions,
                    point,
                );
            }
        }

        mem::swap(&mut self.buffer, &mut self.state);
        mem::swap(&mut self.neighbours, &mut self.neighbours_buffer);

        total
    }

    fn simulate_six_times(&mut self) -> usize {
        (0..6).fold(0, |_, _| self.simulate_step())
    }
}

fn add_point(
    state: &mut HashSet<Vec<i32>>,
    neighbours: &mut HashMap<Vec<i32>, i32>,
    dimensions: usize,
    point: &Vec<i32>,
) {
    state.insert(point.clone());

    for neighbour in neighbour_cubes(&point, dimensions) {
        *neighbours.entry(neighbour).or_insert(0) += 1;
    }
}

fn neighbour_cubes(point: &Vec<i32>, dimensions: usize) -> Vec<Vec<i32>> {
    let mut neighbours = vec![];

    match dimensions {
        3 => {
            for (x, y, z) in iproduct!((-1..=1), (-1..=1), (-1..=1)) {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                neighbours.push(vec![point[0] + x, point[1] + y, point[2] + z]);
            }
        }
        4 => {
            for (x, y, z, w) in iproduct!((-1..=1), (-1..=1), (-1..=1), (-1..=1)) {
                if x == 0 && y == 0 && z == 0 && w == 0 {
                    continue;
                }
                neighbours.push(vec![point[0] + x, point[1] + y, point[2] + z, point[3] + w]);
            }
        }
        _ => panic!("Unsupported dimension: {}", dimensions),
    }

    neighbours
}

pub struct Day17 {}

impl Solution for Day17 {
    fn first_task(&self, cube_text: &str) -> Result<String, Error> {
        let mut cube = ConwayCube::from_str(cube_text, 3);

        Ok(cube.simulate_six_times().to_string())
    }

    fn second_task(&self, cube_text: &str) -> Result<String, Error> {
        let mut cube = ConwayCube::from_str(cube_text, 4);

        Ok(cube.simulate_six_times().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_dimensions() {
        let test_initial_state = ".#.
..#
###";

        let mut test_cube = ConwayCube::from_str(test_initial_state, 3);
        for &active_cubes in &[11, 21, 38] {
            assert_eq!(test_cube.simulate_step(), active_cubes as usize);
        }
    }

    #[test]
    fn test_4_dimensions() {
        let test_initial_state = ".#.
..#
###";

        let mut test_cube = ConwayCube::from_str(test_initial_state, 4);
        for &active_cubes in &[29, 60] {
            assert_eq!(test_cube.simulate_step(), active_cubes as usize);
        }
    }
}
