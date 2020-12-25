use super::solution::{Error, Solution};
use num_complex::Complex;
use std::collections::{HashMap, HashSet};

type Point = Complex<i32>;

static OFFSETS: [Point; 6] = [
    Point::new(-2, 0),
    Point::new(2, 0),
    Point::new(-1, -3),
    Point::new(-1, 3),
    Point::new(1, -3),
    Point::new(1, 3),
];

fn result_position(instructions: &str) -> Point {
    let mut position = Point::new(0, 0);
    let mut chars = instructions.chars();

    loop {
        match chars.next() {
            Some('e') => {
                position += Point::new(-2, 0);
            }
            Some('w') => {
                position += Point::new(2, 0);
            }
            Some('n') => {
                if let Some('e') = chars.next() {
                    position += Point::new(-1, 3);
                } else {
                    position += Point::new(1, 3);
                }
            }
            Some('s') => {
                if let Some('e') = chars.next() {
                    position += Point::new(-1, -3);
                } else {
                    position += Point::new(1, -3);
                }
            }
            Some(n) => panic!("Unsupported instruction: {}", n),

            None => return position,
        };
    }
}

struct TileFloor {
    floor: HashSet<Point>,
    neighbours: HashMap<Point, usize>,
    buffer: HashSet<Point>,
    buffer_neighbours: HashMap<Point, usize>,
}

impl TileFloor {
    fn from_instructions(instructions_text: &str) -> TileFloor {
        let floor = instructions_text
            .lines()
            .map(result_position)
            .fold(&mut HashMap::new(), |acc, el| {
                *acc.entry(el).or_insert(0) += 1;
                acc
            })
            .iter()
            .filter(|(_, &c)| c % 2 == 1)
            .map(|(p, _)| p)
            .copied()
            .collect::<HashSet<_>>();

        let mut neighbours: HashMap<Point, usize> = HashMap::new();
        for &point in floor.iter() {
            for offset in OFFSETS.iter() {
                *neighbours.entry(point + offset).or_insert(0) += 1;
            }
        }

        TileFloor {
            floor,
            neighbours,
            buffer: HashSet::new(),
            buffer_neighbours: HashMap::new(),
        }
    }

    fn black_tiles(&self) -> usize {
        self.floor.len()
    }

    fn simulate_day(&mut self) -> usize {
        let mut black_tiles = 0;
        self.buffer.clear();
        self.buffer_neighbours.clear();

        for (&point, &active_neighbours) in self.neighbours.iter() {
            let is_black = self.floor.contains(&point);
            if (is_black && active_neighbours == 1) || (active_neighbours == 2) {
                black_tiles += 1;

                self.buffer.insert(point);
                for offset in OFFSETS.iter() {
                    *self.buffer_neighbours.entry(point + offset).or_insert(0) += 1;
                }
            }
        }

        std::mem::swap(&mut self.floor, &mut self.buffer);
        std::mem::swap(&mut self.neighbours, &mut self.buffer_neighbours);

        black_tiles
    }
}

fn black_tiles_after_n_days(tile_floor: &mut TileFloor, days: usize) -> usize {
    (0..days).fold(tile_floor.black_tiles(), |_, _| tile_floor.simulate_day())
}

pub struct Day24 {}

impl Solution for Day24 {
    fn first_task(&self, instructions_text: &str) -> Result<String, Error> {
        Ok(TileFloor::from_instructions(instructions_text)
            .black_tiles()
            .to_string())
    }

    fn second_task(&self, instructions_text: &str) -> Result<String, Error> {
        let mut tile_floor = TileFloor::from_instructions(instructions_text);

        Ok(black_tiles_after_n_days(&mut tile_floor, 100).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_position() {
        assert_eq!(result_position("nwwswee"), Point::new(0, 0));
    }

    #[test]
    fn test_first_task() {
        let test_instructions = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

        assert_eq!(
            TileFloor::from_instructions(test_instructions).black_tiles(),
            10
        );
    }

    #[test]
    fn test_simulate_day() {
        let test_instructions = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

        let mut test_floor = TileFloor::from_instructions(test_instructions);

        for &black_tiles in &[15, 12, 25, 14, 23, 28, 41, 37, 49, 37] {
            assert_eq!(test_floor.simulate_day(), black_tiles as usize);
        }
    }

    #[test]
    fn test_second_task() {
        let test_instructions = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

        let mut test_floor = TileFloor::from_instructions(test_instructions);

        assert_eq!(black_tiles_after_n_days(&mut test_floor, 100), 2208);
    }
}
