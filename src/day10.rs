use crate::challenge::DailyChallenge;
use std::fmt::Debug;
use std::fs;
use std::ops::Neg;
use std::path::Path;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Neg for Direction {
    type Output = Direction;
    fn neg(self) -> Self::Output {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Copy, Clone, Debug)]
enum Tile {
    Ground,
    Start,
    Pipe(Direction, Direction),
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Tile::Pipe(Direction::North, Direction::South)),
            '-' => Ok(Tile::Pipe(Direction::East, Direction::West)),
            'L' => Ok(Tile::Pipe(Direction::North, Direction::East)),
            'J' => Ok(Tile::Pipe(Direction::North, Direction::West)),
            '7' => Ok(Tile::Pipe(Direction::South, Direction::West)),
            'F' => Ok(Tile::Pipe(Direction::South, Direction::East)),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Start),
            _ => Err("Invalid character"),
        }
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        use Direction::*;
        use Tile::*;

        match value {
            Ground => '.',
            Start => 'S',
            Pipe(North, South) | Pipe(South, North) => '|',
            Pipe(North, East) | Pipe(East, North) => 'L',
            Pipe(North, West) | Pipe(West, North) => 'J',
            Pipe(West, South) | Pipe(South, West) => '7',
            Pipe(East, South) | Pipe(South, East) => 'F',
            Pipe(East, West) | Pipe(West, East) => '-',
            _ => 'x',
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        use Tile::*;

        match (self, other) {
            (Pipe(a1, a2), Pipe(b1, b2)) if a1 == b1 => a2 == b2,
            (Pipe(a1, a2), Pipe(b1, b2)) if a1 == b2 => a2 == b1,
            (Ground, Ground) => true,
            (Start, Start) => true,
            _ => false,
        }
    }
}

impl Tile {
    fn enter_from(&self, direction: Direction) -> Option<Direction> {
        match self {
            Tile::Pipe(enter, exit) | Tile::Pipe(exit, enter) if *enter == direction => Some(*exit),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Coord {
    row: i32,
    col: i32,
}

impl Coord {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    fn go(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self {
                row: self.row - 1,
                col: self.col,
            },
            Direction::South => Self {
                row: self.row + 1,
                col: self.col,
            },
            Direction::East => Self {
                row: self.row,
                col: self.col + 1,
            },
            Direction::West => Self {
                row: self.row,
                col: self.col - 1,
            },
        }
    }
}

#[derive(PartialEq)]
struct Map {
    start: Coord,
    tiles: Vec<Vec<Tile>>,
}

struct LoopPath(Vec<Coord>);

impl LoopPath {
    fn contains(&self, coord: Coord) -> bool {
        self.0.contains(&coord)
    }
}

impl Map {
    fn from_file(file: &Path) -> Result<Self, String> {
        let contents = fs::read_to_string(file).map_err(|e| e.to_string())?;

        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        let mut start: Option<Coord> = None;

        for (rn, line) in contents.lines().enumerate() {
            let mut row: Vec<Tile> = Vec::new();

            for (cn, c) in line.chars().enumerate() {
                let tile = Tile::try_from(c)?;
                if tile == Tile::Start {
                    if start.is_some() {
                        return Err("Multiple start tiles".to_string());
                    }
                    start = Some(Coord::new(rn as i32, cn as i32));
                }
                row.push(tile);
            }
            tiles.push(row);
        }

        let start = start.ok_or("Missing start tile".to_string())?;
        Ok(Map { start, tiles })
    }

    fn at(&self, coord: Coord) -> Option<Tile> {
        let n_rows = self.tiles.len() as i32;
        let n_cols = self.tiles.first().unwrap().len() as i32;
        if coord.row < 0 || coord.col < 0 || coord.row >= n_rows || coord.col >= n_cols {
            return None;
        }

        self.tiles
            .get(coord.row as usize)?
            .get(coord.col as usize)
            .copied()
    }

    fn find_loop(&self) -> Option<LoopPath> {
        for start_direction in ALL_DIRECTIONS {
            let mut loop_tiles = vec![self.start];
            let mut move_direction = start_direction;

            loop {
                let prev_coord = *(loop_tiles.last().unwrap());
                let current_coord = prev_coord.go(move_direction);
                let Some(current_tile) = self.at(current_coord) else {
                    break;
                };

                if current_tile == Tile::Start {
                    return Some(LoopPath(loop_tiles));
                }

                if let Some(exit_dir) = current_tile.enter_from(-move_direction) {
                    move_direction = exit_dir;
                    loop_tiles.push(current_coord);
                } else {
                    break;
                }
            }
        }

        None
    }

    fn infer_start_tile(&self) -> Option<Tile> {
        let mut inferred_directions: Vec<Direction> = Vec::new();
        for direction in ALL_DIRECTIONS {
            let Some(tile) = self.at(self.start.go(direction)) else {
                continue;
            };

            if let Tile::Pipe(x, y) = tile {
                if (x == -direction) || (y == -direction) {
                    inferred_directions.push(direction);
                }
            }
        }

        if let [x, y] = inferred_directions[..] {
            Some(Tile::Pipe(x, y))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn print_masked(&self, mask: impl Fn(Coord) -> bool) {
        for (row, row_tiles) in self.tiles.iter().enumerate() {
            for (col, tile) in row_tiles.iter().enumerate() {
                let coord = Coord::new(row as i32, col as i32);
                let c = if mask(coord) {
                    (*tile).try_into().unwrap()
                } else {
                    '_'
                };
                print!("{c}");
            }
            print!("\n");
        }
        print!("\n");
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = &self.start;
        writeln!(f, "Map {{ start: {start:?}")?;
        for row in &self.tiles {
            writeln!(f, "  {row:?}")?;
        }
        writeln!(f, "}}")
    }
}

fn find_enlosed_tiles(map: &Map, loop_path: &LoopPath) -> Vec<Coord> {
    let mut enclosed: Vec<Coord> = Vec::new();

    for (row, row_tiles) in map.tiles.iter().enumerate() {
        let mut boundaries_crossed = 0;
        let mut on_edge: Option<Direction> = None;
        for (col, tile) in row_tiles.iter().enumerate() {
            let tile = match tile {
                Tile::Start => map.infer_start_tile().expect("Invalid start tile"),
                _ => *tile,
            };
            let coord = Coord::new(row as i32, col as i32);

            if loop_path.contains(coord) {
                // Tile must be a pipe

                if let Some(edge_start) = on_edge {
                    if tile.enter_from(-edge_start).is_some() {
                        // Edge counts as a boundary
                        // E.g. F---J
                        boundaries_crossed += 1;
                        on_edge = None;
                    } else if tile.enter_from(edge_start).is_some() {
                        // Edge does not count as a boundary
                        // E.g. F---7
                        on_edge = None;
                    }
                } else if tile == Tile::Pipe(Direction::North, Direction::South) {
                    boundaries_crossed += 1;
                } else if tile.enter_from(Direction::North).is_some() {
                    on_edge = Some(Direction::North);
                } else if tile.enter_from(Direction::South).is_some() {
                    on_edge = Some(Direction::South);
                }
            } else {
                if boundaries_crossed % 2 == 1 {
                    enclosed.push(coord);
                }
            }
        }
    }

    enclosed
}

#[derive(Default)]
pub struct Day10;

impl DailyChallenge for Day10 {
    fn part1(&self, file: &Path) -> u64 {
        let map = Map::from_file(file).unwrap();
        if let Some(LoopPath(map_loop)) = map.find_loop() {
            return map_loop.len() as u64 / 2;
        }

        panic!("No loop found");
    }

    fn part2(&self, file: &Path) -> u64 {
        let map = Map::from_file(file).unwrap();
        if let Some(map_loop) = map.find_loop() {
            let enclosed = find_enlosed_tiles(&map, &map_loop);
            return enclosed.len() as u64;
        }
        panic!("No loop found");
    }
}

#[test]
fn test_tile_eq() {
    use Direction::*;
    use Tile::*;

    assert_eq!(Start, Start);
    assert_eq!(Ground, Ground);
    assert_eq!(Pipe(North, South), Pipe(North, South));
    assert_eq!(Pipe(North, South), Pipe(South, North));

    assert_ne!(Pipe(North, South), Pipe(West, North));
    assert_ne!(Pipe(North, South), Ground);
}

#[test]
fn test_from_file() {
    use Direction::*;
    use Tile::*;

    let map = Map::from_file(Path::new("data/10.sample")).expect("Test file missing");
    let expected = Map {
        start: Coord { row: 2, col: 0 },
        tiles: vec![
            vec![Ground, Ground, Pipe(South, East), Pipe(South, West), Ground],
            vec![
                Ground,
                Pipe(South, East),
                Pipe(North, West),
                Pipe(North, South),
                Ground,
            ],
            vec![
                Start,
                Pipe(West, North),
                Ground,
                Pipe(East, North),
                Pipe(West, South),
            ],
            vec![
                Pipe(South, North),
                Pipe(East, South),
                Pipe(West, East),
                Pipe(East, West),
                Pipe(North, West),
            ],
            vec![Pipe(East, North), Pipe(West, North), Ground, Ground, Ground],
        ],
    };
    assert_eq!(map, expected);
}

#[test]
fn test_part1() {
    assert_eq!(Day10.part1(Path::new("data/10.sample")), 8)
}

#[test]
fn test_part2() {
    assert_eq!(Day10.part2(Path::new("data/10.sample2")), 4)
}

#[test]
fn test_part2_larger() {
    assert_eq!(Day10.part2(Path::new("data/10.sample3")), 10)
}
