use super::solution::{Error, Solution};
use std::cmp::{Eq, PartialEq};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

type Border = usize;
type Borders = [Border; 4];
type Grid<T> = Vec<Vec<T>>;
type ArrangementGraph<'a> = HashMap<&'a Tile, HashMap<&'a Tile, Vec<Border>>>;
type Border2Tiles<'a> = HashMap<Border, HashSet<&'a Tile>>;

fn border_value(border: &[char]) -> usize {
    border
        .iter()
        .rev()
        .fold(0, |acc, &el| 2 * acc + if el == '#' { 1 } else { 0 })
}

struct Image {
    image: Grid<char>,
}

fn create_square_grid(size: usize) -> Grid<char> {
    vec![vec!['.'; size]; size]
}

impl Image {
    fn from_str(s: &str) -> Image {
        Image {
            image: s.lines().map(|l| l.chars().collect::<Vec<_>>()).collect(),
        }
    }
    fn square(size: usize) -> Image {
        Image {
            image: create_square_grid(size),
        }
    }

    fn size(&self) -> usize {
        self.image.len()
    }

    fn borders(&self) -> Borders {
        [
            border_value(&self.image[0]),
            border_value(
                &(0..self.size())
                    .map(|r| self.image[r][self.size() - 1])
                    .collect::<Vec<_>>(),
            ),
            border_value(&self.image[self.size() - 1]),
            border_value(
                &(0..self.size())
                    .map(|r| self.image[r][0])
                    .collect::<Vec<_>>(),
            ),
        ]
    }

    fn clone(&self) -> Image {
        let mut image = create_square_grid(self.size());

        for (r, row) in image.iter_mut().enumerate().take(self.size()) {
            for (c, chr) in row.iter_mut().enumerate().take(self.size()) {
                *chr = self.image[r][c];
            }
        }

        Image { image }
    }

    fn rotate_clockwise(&self) -> Image {
        let mut image = create_square_grid(self.size());

        for (r, row) in image.iter_mut().enumerate().take(self.size()) {
            for (c, chr) in row.iter_mut().enumerate().take(self.size()) {
                *chr = self.image[self.size() - c - 1][r];
            }
        }

        Image { image }
    }

    fn flip_horizontally(&self) -> Image {
        let mut image = create_square_grid(self.size());

        for (r, row) in image.iter_mut().enumerate().take(self.size()) {
            for (c, chr) in row.iter_mut().enumerate().take(self.size()) {
                *chr = self.image[self.size() - r - 1][c];
            }
        }

        Image { image }
    }

    fn flip_vertically(&self) -> Image {
        let mut image = create_square_grid(self.size());

        for (r, row) in image.iter_mut().enumerate().take(self.size()) {
            for (c, chr) in row.iter_mut().enumerate().take(self.size()) {
                *chr = self.image[r][self.size() - c - 1];
            }
        }

        Image { image }
    }

    fn possible_images(&self) -> Vec<Image> {
        let mut images = Vec::new();

        for base_image in &[
            self.clone(),
            self.flip_horizontally(),
            self.flip_vertically(),
            self.flip_horizontally().flip_vertically(),
        ] {
            let mut base_image = base_image.clone();
            for _ in 0..4 {
                images.push(base_image.rotate_clockwise());
                base_image = base_image.rotate_clockwise();
            }
        }

        images
    }
}

struct Tile {
    id: usize,
    image: Image,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool {
        self.id == other.id
    }
}

impl Eq for Tile {}

impl Hash for Tile {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state)
    }
}

impl Tile {
    fn size(&self) -> usize {
        self.image.size()
    }

    fn unique_border_sides(&self) -> Vec<usize> {
        self.image
            .possible_images()
            .iter()
            .flat_map(|img| img.borders().iter().copied().collect::<Vec<usize>>())
            .collect::<HashSet<usize>>()
            .iter()
            .copied()
            .collect::<Vec<usize>>()
    }

    fn possible_images(&self) -> Vec<Image> {
        self.image.possible_images()
    }

    fn from_str(tile_text: &str) -> Tile {
        let mut lines = tile_text.lines();
        let id_value = lines
            .next()
            .unwrap()
            .trim_start_matches("Tile ")
            .trim_end_matches(':')
            .parse::<usize>()
            .unwrap();

        let image_text = lines.collect::<Vec<&str>>().join("\n");
        let image = Image::from_str(&image_text);

        Tile {
            id: id_value,
            image,
        }
    }
}

struct Arrangement {
    size: usize,
    grid: Grid<Tile>,
}

fn collect_border2tiles(tiles: &[Tile]) -> Border2Tiles {
    let mut border2tiles: Border2Tiles = HashMap::new();

    for tile in tiles {
        for border in tile.unique_border_sides() {
            let entry = border2tiles.entry(border).or_insert_with(HashSet::new);

            entry.insert(&tile);
        }
    }

    border2tiles
}

fn recover_tile_graph<'a>(border2tiles: &'a Border2Tiles) -> ArrangementGraph<'a> {
    let mut graph: ArrangementGraph = HashMap::new();

    for (side, tiles) in border2tiles {
        if tiles.len() == 1 {
            continue;
        }

        let tiles_list = tiles.iter().collect::<Vec<_>>();
        let first = tiles_list[0];
        let second = tiles_list[1];

        let first_entry = graph
            .entry(*first)
            .or_insert_with(HashMap::new)
            .entry(*second)
            .or_insert(vec![]);
        first_entry.push(*side);

        let second_entry = graph
            .entry(*second)
            .or_insert_with(HashMap::new)
            .entry(*first)
            .or_insert(vec![]);
        second_entry.push(*side);
    }

    graph
}

fn find_corner_tiles<'a>(border2tiles: &'a Border2Tiles) -> Vec<&'a Tile> {
    let mut counter: HashMap<&Tile, usize> = HashMap::new();

    for tiles in border2tiles.values() {
        if tiles.len() == 1 {
            continue;
        }

        for tile in tiles {
            *counter.entry(tile).or_insert(0) += 1;
        }
    }

    counter
        .iter()
        .filter(|(_, &size)| size == 4)
        .map(|(&tile, _)| tile)
        .collect::<Vec<_>>()
}

fn recover_arrangement_grid<'a>(
    size: usize,
    starting_tile: &'a Tile,
    graph: &'a ArrangementGraph,
) -> Grid<Tile> {
    let mut visited: HashSet<&Tile> = HashSet::new();
    visited.insert(starting_tile);

    let mut arrangement: Grid<&Tile> = vec![];
    let mut current_row: Vec<&Tile> = vec![starting_tile];

    for _ in 0..size - 1 {
        for neighbour in graph[current_row[current_row.len() - 1]].keys() {
            if !visited.contains(neighbour) && graph[neighbour].len() < 4 {
                visited.insert(neighbour);
                current_row.push(neighbour);
                break;
            }
        }
    }

    arrangement.push(current_row);

    for _ in 0..size - 1 {
        let mut current_row = vec![];

        for &last_elem in arrangement[arrangement.len() - 1].iter() {
            for &n in graph[last_elem].keys() {
                if !visited.contains(n) {
                    visited.insert(n);
                    current_row.push(n);
                    break;
                }
            }
        }

        arrangement.push(current_row);
    }

    let mut fixed_arrangement: Grid<Tile> = vec![];

    for r in 0..size {
        let mut current_row = vec![];

        for c in 0..size {
            let tile = arrangement[r][c];

            for possible_image in tile.possible_images() {
                let mut orientation_found = true;

                for (border, (dr, dc)) in [(-1_i32, 0), (0, 1), (1, 0), (0, -1_i32)]
                    .iter()
                    .enumerate()
                {
                    let nr: i32 = r as i32 + dr;
                    let nc: i32 = c as i32 + dc;

                    if nr < 0 || nr as usize >= size || nc < 0 || nc as usize >= size {
                        continue;
                    }

                    let possible_border = possible_image.borders()[border];
                    let allowed_borders = graph
                        .get(tile)
                        .unwrap()
                        .get(arrangement[nr as usize][nc as usize])
                        .unwrap();

                    if !allowed_borders.contains(&possible_border) {
                        orientation_found = false;
                        break;
                    }
                }

                if orientation_found {
                    current_row.push(Tile {
                        id: tile.id,
                        image: possible_image,
                    });

                    break;
                }
            }
        }

        fixed_arrangement.push(current_row);
    }

    validate_arrangement_grid(size, &fixed_arrangement);

    fixed_arrangement
}

fn validate_arrangement_grid(size: usize, arrangement: &Grid<Tile>) {
    for r in 0..size {
        for c in 0..size {
            let tile = &arrangement[r][c];
            let tile_border = tile.image.borders();

            for (border, (dr, dc)) in [(-1_i32, 0), (0, 1), (1, 0), (0, -1_i32)]
                .iter()
                .enumerate()
            {
                let nr: i32 = r as i32 + dr;
                let nc: i32 = c as i32 + dc;

                if nr < 0 || nr as usize >= size || nc < 0 || nc as usize >= size {
                    continue;
                }

                let n_border = &arrangement[nr as usize][nc as usize].image.borders();

                assert!(tile_border[border] == n_border[(border + 2) % 4]);
            }
        }
    }
}

fn recover_from_tiles(size: usize, tiles: Vec<Tile>) -> Grid<Tile> {
    let border2tiles = collect_border2tiles(&tiles);
    let graph = recover_tile_graph(&border2tiles);
    let border_tiles = find_corner_tiles(&border2tiles);

    recover_arrangement_grid(size, border_tiles[0], &graph)
}

impl Arrangement {
    fn find_corner_tiles(&self) -> [usize; 4] {
        [
            self.grid[0][0].id,
            self.grid[0][self.grid.len() - 1].id,
            self.grid[self.grid.len() - 1][self.grid.len() - 1].id,
            self.grid[self.grid.len() - 1][0].id,
        ]
    }

    fn recover_original_image(&self) -> Image {
        let tile_size = self.grid[0][0].size();
        let big_tile_size = self.size * (tile_size - 2);
        let mut big_tile_image = Image::square(big_tile_size);

        for (row, grid_row) in self.grid.iter().enumerate() {
            for (col, tile) in grid_row.iter().enumerate() {
                for tr in 1..(tile_size - 1) {
                    for tc in 1..(tile_size - 1) {
                        let nr = row * (tile_size - 2) + tr - 1;
                        let nc = col * (tile_size - 2) + tc - 1;
                        big_tile_image.image[nr][nc] = tile.image.image[tr][tc];
                    }
                }
            }
        }

        big_tile_image
    }

    fn from_str(tiles_text: &str) -> Arrangement {
        let tiles = tiles_text
            .split("\n\n")
            .map(|t| Tile::from_str(t))
            .collect::<Vec<_>>();

        let size = (tiles.len() as f64).sqrt() as usize;
        let grid = recover_from_tiles(size, tiles);

        Arrangement { size, grid }
    }
}

fn count_product_of_corners_in_arrangement(arr: &Arrangement) -> usize {
    let border_tiles = arr.find_corner_tiles();

    border_tiles.iter().product()
}

fn dragon_points() -> Vec<(usize, usize)> {
    [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ]
    .iter()
    .enumerate()
    .flat_map(|(r, row)| row.chars().enumerate().map(move |(c, ch)| (r, c, ch)))
    .filter(|(_, _, ch)| *ch == '#')
    .map(|(r, c, _)| (r, c))
    .collect()
}

fn has_dragon_at(image: &Image, r: usize, c: usize) -> bool {
    for (dr, dc) in dragon_points() {
        let (nr, nc) = (r + dr, c + dc);
        if nr >= image.size() || nc >= image.size() {
            return false;
        }

        if image.image[nr][nc] != '#' {
            return false;
        }
    }

    true
}

fn mark_dragon_at(image: &mut Image, r: usize, c: usize) {
    for (dr, dc) in dragon_points() {
        let (nr, nc) = (r + dr, c + dc);
        if nr >= image.size() || nc >= image.size() {
            continue;
        }

        image.image[nr][nc] = 'O';
    }
}

fn try_mark_sea_monsters(image: &mut Image) -> usize {
    let size = image.size();
    let mut count = 0;

    for r in 0..size {
        for c in 0..size {
            if has_dragon_at(image, r, c) {
                count += 1;
                mark_dragon_at(image, r, c);
            }
        }
    }

    count
}

fn mark_image_with_sea_monsters(image: &Image) -> Image {
    for possible_image in image.possible_images() {
        let mut possible_image = possible_image;
        if try_mark_sea_monsters(&mut possible_image) > 0 {
            return possible_image;
        }
    }

    image.clone()
}

fn count_hashes_in(image: &Image) -> usize {
    image
        .image
        .iter()
        .flatten()
        .filter(|&&el| el == '#')
        .count()
}

fn count_water_roughness(arr: &Arrangement) -> usize {
    let recovered_image = arr.recover_original_image();
    let marked_image = mark_image_with_sea_monsters(&recovered_image);

    count_hashes_in(&marked_image)
}

pub struct Day20 {}

impl Solution for Day20 {
    fn first_task(&self, tiles_text: &str) -> Result<String, Error> {
        let arrangement = Arrangement::from_str(tiles_text);

        Ok(count_product_of_corners_in_arrangement(&arrangement).to_string())
    }

    fn second_task(&self, tiles_text: &str) -> Result<String, Error> {
        let arrangement = Arrangement::from_str(tiles_text);

        Ok(count_water_roughness(&arrangement).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_function() {
        let test_borders = [
            ("#", 1),
            ("#.", 1),
            (".#", 2),
            ("#.#", 5),
            ("..##.#...", 44),
        ];

        for (border, number) in test_borders.iter() {
            assert_eq!(
                border_value(&border.chars().collect::<Vec<_>>()),
                *number as usize
            );
        }
    }

    #[test]
    fn test_borders_initial_image() {
        let test_image = "..##.
##..#
#...#
####.
##.##";

        assert_eq!(Image::from_str(test_image).borders(), [12, 22, 27, 30]);
    }

    #[test]
    fn test_borders_flip_horizontally() {
        let test_image = "..##.
##..#
#...#
####.
##.##";

        assert_eq!(
            Image::from_str(test_image).flip_horizontally().borders(),
            [27, 13, 12, 15]
        );
    }

    #[test]
    fn test_borders_flip_vertically() {
        let test_image = "..##.
##..#
#...#
####.
##.##";

        assert_eq!(
            Image::from_str(test_image).flip_vertically().borders(),
            [6, 30, 27, 22]
        );
    }

    #[test]
    fn test_first_task() {
        let test_tiles_text = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

        let test_arrangement = Arrangement::from_str(test_tiles_text);
        assert_eq!(
            count_product_of_corners_in_arrangement(&test_arrangement),
            20899048083289
        );
    }

    #[test]
    fn test_second_task() {
        let test_tiles_text = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

        let test_arrangement = Arrangement::from_str(test_tiles_text);
        assert_eq!(count_water_roughness(&test_arrangement), 273);
    }
}
