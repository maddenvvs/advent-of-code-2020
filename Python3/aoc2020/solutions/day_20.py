from __future__ import annotations

from collections import defaultdict
from functools import reduce
from operator import mul
from typing import Dict, Iterator, List, Sequence, Set, Tuple, TypeVar

from .solution import Solution

TGridItem = TypeVar("TGridItem")
Grid = List[List[TGridItem]]

Borders = Tuple[int, int, int, int]


def border_value(border: Sequence[str]) -> int:
    return reduce(lambda acc, v: 2 * acc + (v == "#"), reversed(border), 0)


class Image:
    __slots__ = "image"

    image: Grid[str]

    def __init__(self, image: Grid[str]):
        assert len(image) == len(image[0])

        self.image = image

    def __getitem__(self, key):
        return self.image[key]

    def __str__(self) -> str:
        return "\n".join("".join(row) for row in self.image)

    @property
    def size(self) -> int:
        return len(self.image)

    def rotate_clockwise(self) -> Image:
        return Image([list(row) for row in zip(*self.image)])

    def flip_vertically(self) -> Image:
        return Image([row[::-1] for row in self.image])

    def flip_horizontally(self) -> Image:
        return Image([row[:] for row in reversed(self.image)])

    def clone(self) -> Image:
        return Image([row[:] for row in self.image])

    def borders(self) -> Borders:
        return (
            border_value(self.image[0]),
            border_value(list(zip(*self.image))[-1]),
            border_value(self.image[-1]),
            border_value(next(zip(*self.image))),
        )

    def possible_images(self) -> Iterator[Image]:
        for base_configuration in [
            self.clone(),
            self.flip_vertically(),
            self.flip_horizontally(),
            self.flip_horizontally().flip_vertically()
        ]:
            yield base_configuration

            for _ in range(3):
                base_configuration = base_configuration.rotate_clockwise()
                yield base_configuration

    @classmethod
    def square(cls, size: int) -> Image:
        return cls([["."] * size for _ in range(size)])


class Tile:
    __slots__ = "id", "image"

    def __init__(self, _id: int, image: Image):
        self.id = _id
        self.image = image

    @property
    def size(self):
        return self.image.size

    def unique_border_sides(self):
        return set(b for image in self.possible_images() for b in image.borders())

    def possible_images(self) -> Iterator[Image]:
        return self.image.possible_images()

    def __repr__(self):
        return f"Tile {self.id}"

    def __str__(self):
        return f"Tile {self.id}"

    def __hash__(self):
        return hash(self.id)

    def __eq__(self, other):
        return isinstance(other, Tile) and self.id == other.id

    @ classmethod
    def from_str(cls, tile_text: str) -> Tile:
        lines = tile_text.splitlines()
        id_value = int(lines[0][-5: -1], base=10)
        image = Image([list(lines[idx]) for idx in range(1, len(lines))])

        return cls(id_value, image)


ArrangementGraph = Dict[Tile, Dict[Tile, List[int]]]
Border2Tiles = Dict[int, Set[Tile]]


class Arrangement:
    __slots__ = "size", "grid"

    size: int
    grid: Grid[Tile]

    def __init__(self, size: int):
        self.size = size
        self.grid = []

    def collect_border2tiles(self, tiles: List[Tile]) -> Border2Tiles:
        border2tiles = defaultdict(set)

        for tile in tiles:
            for side in tile.unique_border_sides():
                border2tiles[side].add(tile)

        return border2tiles

    def recover_tile_graph(self, border2tiles: Border2Tiles) -> ArrangementGraph:
        graph: ArrangementGraph = defaultdict(lambda: defaultdict(list))
        for side, tiles in border2tiles.items():
            if len(tiles) == 1:
                continue

            assert len(tiles) == 2

            first, second = list(tiles)
            graph[first][second].append(side)
            graph[second][first].append(side)

        for edges in graph.values():
            assert 2 <= len(edges) <= 4

        return graph

    def _find_corner_tiles(self, border2tiles: Border2Tiles) -> List[Tile]:
        counter: Dict[Tile, int] = defaultdict(int)

        for side, tiles in border2tiles.items():
            if len(tiles) == 1:
                continue

            for tile in tiles:
                counter[tile] += 1

        for tile in counter:
            counter[tile] //= 2

        border_tiles = [tile for tile,
                        neighbours in counter.items() if neighbours == 2]

        assert len(border_tiles) == 4

        return border_tiles

    def recover_arrangement_grid(self, starting_tile: Tile, graph: ArrangementGraph) -> Grid[Tile]:
        visited = set([starting_tile])
        arrangement = []
        current_row = [starting_tile]
        for _ in range(self.size - 1):
            for neighbour in graph[current_row[-1]]:
                if neighbour not in visited and len(graph[neighbour]) < 4:
                    visited.add(neighbour)
                    current_row.append(neighbour)
                    break

        arrangement.append(current_row[:])

        for _ in range(self.size - 1):
            current_row.clear()

            for last_elem in arrangement[-1]:
                for n in graph[last_elem]:
                    if n not in visited:
                        visited.add(n)
                        current_row.append(n)
                        break

            arrangement.append(current_row[:])

        for r in range(self.size):
            for c in range(self.size):
                tile = arrangement[r][c]

                tile_orientation_found = False
                for possible_image in tile.possible_images():
                    for border, (dr, dc) in enumerate(((-1, 0), (0, 1), (1, 0), (0, -1))):
                        nr, nc = r + dr, c + dc
                        if nr < 0 or nr >= self.size or nc < 0 or nc >= self.size:
                            continue

                        possible_border = possible_image.borders()[border]
                        allowed_borders = graph[tile][arrangement[nr][nc]]

                        if possible_border not in allowed_borders:
                            break
                    else:
                        arrangement[r][c].image = possible_image
                        tile_orientation_found = True
                        break

                assert tile_orientation_found

        self.validate_arrangement_grid(arrangement)

        return arrangement

    def validate_arrangement_grid(self, arrangement: Grid[Tile]):
        for r in range(self.size):
            for c in range(self.size):
                tile = arrangement[r][c]
                tile_border = tile.image.borders()
                for border, (dr, dc) in enumerate(((-1, 0), (0, 1), (1, 0), (0, -1))):
                    nr, nc = r + dr, c + dc
                    if nr < 0 or nr >= self.size or nc < 0 or nc >= self.size:
                        continue
                    n_border = arrangement[nr][nc].image.borders()

                    assert tile_border[border] == n_border[(border + 2) % 4]

    def recover_from_tiles(self, tiles: List[Tile]) -> None:
        border2tiles = self.collect_border2tiles(tiles)
        graph = self.recover_tile_graph(border2tiles)

        assert len(graph) == len(tiles)

        border_tiles = self._find_corner_tiles(border2tiles)
        grid = self.recover_arrangement_grid(border_tiles[0], graph)

        self.grid = grid

    def find_corner_tiles(self) -> List[Tile]:
        return [
            self.grid[0][0],
            self.grid[0][-1],
            self.grid[-1][-1],
            self.grid[-1][0],
        ]

    def recover_original_image(self) -> Image:
        tile_size = self.grid[0][0].size
        big_tile_size = self.size * (tile_size - 2)
        big_tile_image = Image.square(big_tile_size)

        for row, grid_row in enumerate(self.grid):
            for col, tile in enumerate(grid_row):
                for tr, tile_row in enumerate(tile.image[1:-1]):
                    for tc, tile_col in enumerate(tile_row[1:-1]):
                        nr = row * (tile_size - 2) + tr
                        nc = col * (tile_size - 2) + tc
                        big_tile_image[nr][nc] = tile_col

        return big_tile_image

    def __str__(self):
        tile_size = self.grid[0][0].size
        screen = []

        for row, grid_row in enumerate(self.grid):
            line = [[] for _ in range(tile_size)]

            for col, tile in enumerate(grid_row):

                for i, r in enumerate(tile.image):
                    line[i].append("".join(r))

            for l in line:
                screen.append(" ".join(l))

            screen.append("")

        return "\n".join(screen)

    @ classmethod
    def recover_from_str(cls, tiles_text: str) -> Arrangement:
        tiles = [Tile.from_str(tile_text)
                 for tile_text in tiles_text.split("\n\n")]

        arrangement = cls(int(pow(len(tiles), 0.5)))
        arrangement.recover_from_tiles(tiles)

        return arrangement


def has_dragon_at(image: Image, r: int, c: int) -> bool:
    for dr, dc in dragon_points():
        nr, nc = r + dr, c + dc
        if nr >= image.size or nc >= image.size:
            return False

        if image.image[nr][nc] != "#":
            return False
    return True


def mark_dragon_at(image: Image, r: int, c: int) -> None:
    for dr, dc in dragon_points():
        nr, nc = r + dr, c + dc

        if nr >= image.size or nc >= image.size:
            continue

        image.image[nr][nc] = "O"


def dragon_points():
    dragon = ["                  # ",
              "#    ##    ##    ###",
              " #  #  #  #  #  #   "]

    for r, row in enumerate(dragon):
        for c, v in enumerate(row):
            if v == "#":
                yield r, c


def try_mark_sea_monsters(image: Image) -> int:
    size = image.size
    count = 0

    for r in range(size):
        for c in range(size):
            if has_dragon_at(image, r, c):
                count += 1
                mark_dragon_at(image, r, c)

    return count


def mark_image_with_sea_monsters(image: Image) -> Image:
    for possible_image in image.possible_images():
        if try_mark_sea_monsters(possible_image) > 0:
            return possible_image

    return image


def count_hashes_in(image: Image) -> int:
    hashes = 0
    for r, row in enumerate(image.image):
        for c, v in enumerate(row):
            if v == "#":
                hashes += 1
    return hashes


def count_product_of_corners_in_arrangement(arrangement: Arrangement):
    border_tiles = arrangement.find_corner_tiles()

    return reduce(mul, (tile.id for tile in border_tiles), 1)


def count_water_roughness(arrangement: Arrangement):
    recovered_image = arrangement.recover_original_image()
    marked_image = mark_image_with_sea_monsters(recovered_image)

    return count_hashes_in(marked_image)


class Day20(Solution):

    def first_task(self, tiles_text: str) -> str:
        arrangement = Arrangement.recover_from_str(tiles_text)

        return str(count_product_of_corners_in_arrangement(arrangement))

    def second_task(self, tiles_text: str) -> str:
        arrangement = Arrangement.recover_from_str(tiles_text)

        return str(count_water_roughness(arrangement))
