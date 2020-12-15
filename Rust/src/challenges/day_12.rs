use super::challenge::{Challenge, ChallengeErr};
use num_complex::Complex;
use std::str::FromStr;

static DIRECTIONS: [Complex<i32>; 4] = [
    Complex::new(1, 0),
    Complex::new(0, 1),
    Complex::new(-1, 0),
    Complex::new(0, -1),
];

enum Direction {
    East,
    North,
    West,
    South,
}

impl Direction {
    fn get_direction_vector(&self) -> Complex<i32> {
        use Direction::*;

        match self {
            East => DIRECTIONS[0],
            North => DIRECTIONS[1],
            West => DIRECTIONS[2],
            South => DIRECTIONS[3],
        }
    }
}

enum Instruction {
    Forward(i32),
    Left(i32),
    Right(i32),
    Move(Direction, i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        use Instruction::*;

        match &val[..1] {
            "F" => Ok(Forward(val[1..].parse().unwrap())),
            "R" => Ok(Right(val[1..].parse().unwrap())),
            "L" => Ok(Left(val[1..].parse().unwrap())),
            "N" => Ok(Move(North, val[1..].parse().unwrap())),
            "E" => Ok(Move(East, val[1..].parse().unwrap())),
            "W" => Ok(Move(West, val[1..].parse().unwrap())),
            "S" => Ok(Move(South, val[1..].parse().unwrap())),
            instr => Err(format!("Unsupported instruction: {}", instr)),
        }
    }
}

fn manhattan_distance(vector: &Complex<i32>) -> i32 {
    vector.re.abs() + vector.im.abs()
}

fn parse_instructions(instructions_text: &str) -> Vec<Instruction> {
    instructions_text
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn simulate_instructions(
    instructions: &[Instruction],
    waypoint: Complex<i32>,
    move_waypoint: bool,
) -> Complex<i32> {
    use Direction::*;
    use Instruction::*;

    let mut position = Complex::new(0, 0);
    let mut waypoint = waypoint;

    for instruction in instructions.iter() {
        match instruction {
            Forward(val) => position += waypoint * val,
            Left(val) => waypoint *= North.get_direction_vector().powi(val / 90),
            Right(val) => waypoint *= South.get_direction_vector().powi(val / 90),
            Move(dir, val) => {
                let direction_vector = dir.get_direction_vector() * val;

                if move_waypoint {
                    waypoint += direction_vector;
                } else {
                    position += direction_vector;
                }
            }
        }
    }

    position
}

fn simulate_instructions_with_rotation(instructions: &[Instruction]) -> i32 {
    manhattan_distance(&simulate_instructions(
        instructions,
        Complex::new(1, 0),
        false,
    ))
}

fn simulate_instructions_with_waypoint(instructions: &[Instruction]) -> i32 {
    manhattan_distance(&simulate_instructions(
        instructions,
        Complex::new(10, 1),
        true,
    ))
}

pub struct Solution {}

impl Challenge for Solution {
    fn first_part(&self, instructions_text: &str) -> Result<String, ChallengeErr> {
        Ok(
            simulate_instructions_with_rotation(&parse_instructions(&instructions_text))
                .to_string(),
        )
    }

    fn second_part(&self, instructions_text: &str) -> Result<String, ChallengeErr> {
        Ok(
            simulate_instructions_with_waypoint(&parse_instructions(&instructions_text))
                .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_instructions_with_rotation() {
        let test_instructions_1_text = "F10
N3
F7
R90
F11";

        let test_instructions_1 = parse_instructions(test_instructions_1_text);
        assert_eq!(
            simulate_instructions_with_rotation(&test_instructions_1),
            25
        );
    }

    #[test]
    fn test_simulate_instructions_with_waypoint() {
        let test_instructions_1_text = "F10
N3
F7
R90
F11";

        let test_instructions_1 = parse_instructions(test_instructions_1_text);
        assert_eq!(
            simulate_instructions_with_waypoint(&test_instructions_1),
            286
        );
    }
}
