use super::solution::{Error, Solution};
use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};
use std::mem;

struct ConwayCube {
    dimensions: usize,
    state: HashSet<Vec<i8>>,
    buffer: HashSet<Vec<i8>>,
    neighbours: HashMap<Vec<i8>, i8>,
    neighbours_buffer: HashMap<Vec<i8>, i8>,
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
                    point[0] = x as i8;
                    point[1] = y as i8;

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
    state: &mut HashSet<Vec<i8>>,
    neighbours: &mut HashMap<Vec<i8>, i8>,
    dimensions: usize,
    point: &[i8],
) {
    state.insert(point.to_vec());

    for neighbour in neighbour_cubes(&point, dimensions) {
        *neighbours.entry(neighbour).or_insert(0) += 1;
    }
}

fn neighbour_cubes(point: &[i8], dimensions: usize) -> Box<dyn Iterator<Item = Vec<i8>>> {
    match dimensions {
        3 => {
            let (&px, &py, &pz) = point.iter().collect_tuple().unwrap();
            Box::new(
                iproduct!((-1..=1), (-1..=1), (-1..=1))
                    .filter(|&p| p != (0, 0, 0))
                    .map(move |(x, y, z)| vec![px + x, py + y, pz + z]),
            )
        }
        4 => {
            let (&px, &py, &pz, &pw) = point.iter().collect_tuple().unwrap();
            Box::new(
                iproduct!((-1..=1), (-1..=1), (-1..=1), (-1..=1))
                    .filter(|&p| p != (0, 0, 0, 0))
                    .map(move |(x, y, z, w)| vec![px + x, py + y, pz + z, pw + w]),
            )
        }
        _ => panic!("Unsupported dimension: {}", dimensions),
    }
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
