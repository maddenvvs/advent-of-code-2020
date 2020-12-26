use super::solution::{Error as ChallengeErr, Solution};
use core::fmt;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Free,
    Occupied,
}

impl Cell {
    fn from_char(ch: char) -> Cell {
        match ch {
            '#' => Cell::Occupied,
            'L' => Cell::Free,
            '.' => Cell::Empty,
            val => panic!(format!("Unrecognized cell value: {}", val)),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Cell::Occupied => write!(f, "#"),
            Cell::Free => write!(f, "L"),
            Cell::Empty => write!(f, "."),
        }
    }
}

struct WaitingArea {
    width: usize,
    height: usize,
    area: Vec<Vec<Cell>>,
    buffer: Vec<Vec<Cell>>,
}

impl fmt::Display for WaitingArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for (r, row) in self.area.iter().enumerate() {
            for cell in row {
                if let err @ Err(_) = write!(f, "{}", cell) {
                    return err;
                }
            }
            if r + 1 < self.height {
                if let err @ Err(_) = write!(f, "\n") {
                    return err;
                }
            }
        }
        Ok(())
    }
}

static ADJACENT_DIRECTIONS: [(i8, i8); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

impl WaitingArea {
    fn from_str(s: &str) -> WaitingArea {
        let lines: Vec<&str> = s.lines().collect();
        let height = lines.len();
        let width = lines[0].len();

        WaitingArea {
            width,
            height,
            area: lines
                .iter()
                .map(|line| line.chars().map(&Cell::from_char).collect())
                .collect(),
            buffer: vec![vec![Cell::Empty; width]; height],
        }
    }

    fn is_in_area(&self, r: i8, c: i8) -> bool {
        0 <= r && r < self.height as i8 && 0 <= c && c < self.width as i8
    }

    fn adjacent_neighbors(&self, r: usize, c: usize) -> impl Iterator<Item = Cell> + '_ {
        ADJACENT_DIRECTIONS
            .iter()
            .map(move |(dr, dc)| (r as i8 + *dr, c as i8 + *dc))
            .filter(move |(nr, nc)| self.is_in_area(*nr, *nc))
            .map(move |(nr, nc)| self.area[nr as usize][nc as usize])
            .filter(|&cell| !matches!(cell, Cell::Empty))
    }

    fn visible_seats_from(&self, r: usize, c: usize) -> impl Iterator<Item = Cell> + '_ {
        ADJACENT_DIRECTIONS
            .iter()
            .map(move |(dr, dc)| {
                let (mut nr, mut nc) = (r as i8 + dr, c as i8 + dc);
                while 0 <= nr && nr < self.height as i8 && 0 <= nc && nc < self.width as i8 {
                    if self.area[nr as usize][nc as usize] != Cell::Empty {
                        return self.area[nr as usize][nc as usize];
                    }

                    nr += dr;
                    nc += dc;
                }

                Cell::Empty
            })
            .filter(|&cell| !matches!(cell, Cell::Empty))
    }

    fn simulate_intolerant_step(&mut self) -> (bool, i32) {
        let (mut has_changes, mut occupied_seats) = (false, 0);

        for (r, row) in self.area.iter().enumerate() {
            for (c, seat) in row.iter().enumerate() {
                let new_seat = match self.area[r as usize][c as usize] {
                    Cell::Empty => Cell::Empty,
                    Cell::Free => {
                        if self
                            .adjacent_neighbors(r, c)
                            .filter(|c| matches!(c, Cell::Occupied))
                            .count()
                            == 0
                        {
                            Cell::Occupied
                        } else {
                            Cell::Free
                        }
                    }
                    Cell::Occupied => {
                        if self
                            .adjacent_neighbors(r, c)
                            .filter(|c| matches!(c, Cell::Occupied))
                            .count()
                            >= 4
                        {
                            Cell::Free
                        } else {
                            Cell::Occupied
                        }
                    }
                };
                if new_seat != *seat {
                    has_changes = true
                }
                if new_seat == Cell::Occupied {
                    occupied_seats += 1;
                }
                self.buffer[r][c] = new_seat;
            }
        }

        std::mem::swap(&mut self.area, &mut self.buffer);

        (has_changes, occupied_seats)
    }

    fn simulate_tolerant_step(&mut self) -> (bool, i32) {
        let (mut has_changes, mut occupied_seats) = (false, 0);

        for (r, row) in self.area.iter().enumerate() {
            for (c, seat) in row.iter().enumerate() {
                let new_seat = match self.area[r as usize][c as usize] {
                    Cell::Empty => Cell::Empty,
                    Cell::Free => {
                        if self
                            .visible_seats_from(r, c)
                            .filter(|c| matches!(c, Cell::Occupied))
                            .count()
                            == 0
                        {
                            Cell::Occupied
                        } else {
                            Cell::Free
                        }
                    }
                    Cell::Occupied => {
                        if self
                            .visible_seats_from(r, c)
                            .filter(|c| matches!(c, Cell::Occupied))
                            .count()
                            >= 5
                        {
                            Cell::Free
                        } else {
                            Cell::Occupied
                        }
                    }
                };
                if new_seat != *seat {
                    has_changes = true
                }
                if new_seat == Cell::Occupied {
                    occupied_seats += 1;
                }
                self.buffer[r][c] = new_seat;
            }
        }

        std::mem::swap(&mut self.area, &mut self.buffer);

        (has_changes, occupied_seats)
    }

    fn intolerant_equilibrium(&mut self) -> i32 {
        let (mut has_changes, mut occupied_seats) = self.simulate_intolerant_step();
        while has_changes {
            let (h, s) = self.simulate_intolerant_step();
            has_changes = h;
            occupied_seats = s;
        }
        occupied_seats
    }

    fn tolerant_equilibrium(&mut self) -> i32 {
        let (mut has_changes, mut occupied_seats) = self.simulate_tolerant_step();
        while has_changes {
            let (h, s) = self.simulate_tolerant_step();
            has_changes = h;
            occupied_seats = s;
        }
        occupied_seats
    }
}

pub struct Day11 {}

impl Solution for Day11 {
    fn first_task(&self, seats_str: &str) -> Result<String, ChallengeErr> {
        let mut waiting_area = WaitingArea::from_str(&seats_str);

        Ok(waiting_area.intolerant_equilibrium().to_string())
    }

    fn second_task(&self, seats_str: &str) -> Result<String, ChallengeErr> {
        let mut waiting_area = WaitingArea::from_str(&seats_str);

        Ok(waiting_area.tolerant_equilibrium().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intolerant_step_simulation() {
        let test_area_str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let mut test_area = WaitingArea::from_str(&test_area_str);
        let intolerant_simulation_steps = [
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
            "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
            "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##",
            "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##",
            "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##",
        ];

        for simulation_map in intolerant_simulation_steps.iter() {
            test_area.simulate_intolerant_step();
            assert_eq!(format!("{}", test_area), String::from(*simulation_map));
        }
    }

    #[test]
    fn test_intolerant_equilibrium() {
        let test_area_str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let mut test_area = WaitingArea::from_str(&test_area_str);
        assert_eq!(test_area.intolerant_equilibrium(), 37);
    }

    #[test]
    fn test_tolerant_equilibrium() {
        let test_area_str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let mut test_area = WaitingArea::from_str(&test_area_str);
        assert_eq!(test_area.tolerant_equilibrium(), 26);
    }
}
