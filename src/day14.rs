use crate::challenge::{DailyChallenge, Solver};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fmt::{self, Debug, Write};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut, Index, IndexMut, Range};
use std::path::Path;
use std::str::FromStr;
use std::{fs, io};

#[derive(Default)]
pub struct Day14;

#[derive(Debug)]
enum Day14Error {
    InvalidRock,
    IOError(io::Error),
}

impl From<io::Error> for Day14Error {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl Error for Day14Error {}

impl fmt::Display for Day14Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Day14Error as E;
        match self {
            E::InvalidRock => write!(f, "Invalid rock"),
            E::IOError(e) => std::fmt::Display::fmt(&e, f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum Rock {
    Round,
    Cube,
    Empty,
}

impl TryFrom<char> for Rock {
    type Error = Day14Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let rock = match value {
            'O' => Rock::Round,
            '#' => Rock::Cube,
            '.' => Rock::Empty,
            _ => return Err(Day14Error::InvalidRock),
        };

        Ok(rock)
    }
}

impl Into<char> for Rock {
    fn into(self) -> char {
        match self {
            Rock::Round => 'O',
            Rock::Cube => '#',
            Rock::Empty => '.',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn from_gravity(cross_idx: usize, grav_idx: usize, direction: Direction, size: usize) -> Self {
        match direction {
            Direction::North => Coord {
                row: grav_idx,
                col: cross_idx,
            },
            Direction::South => Coord {
                row: size - grav_idx - 1,
                col: cross_idx,
            },
            Direction::East => Coord {
                row: cross_idx,
                col: size - grav_idx - 1,
            },
            Direction::West => Coord {
                row: cross_idx,
                col: grav_idx,
            },
        }
    }
}

#[derive(PartialEq, Clone, Hash)]
struct Platform {
    rocks: Vec<Vec<Rock>>,
    size: usize,
}

impl Platform {
    fn tilt(&mut self, direction: Direction) {
        for cross_idx in 0..self.size {
            let mut round_rocks: usize = 0;
            let mut bottom: usize = 0;

            for grav_idx in 0..self.size {
                let coord = Coord::from_gravity(cross_idx, grav_idx, direction, self.size);
                match self[coord] {
                    Rock::Round => round_rocks += 1,
                    Rock::Cube => {
                        self._apply_partial_gravity(
                            direction,
                            round_rocks,
                            cross_idx,
                            bottom..grav_idx,
                        );
                        bottom = grav_idx + 1;
                        round_rocks = 0;
                    }
                    Rock::Empty => {}
                }
            }
            self._apply_partial_gravity(direction, round_rocks, cross_idx, bottom..self.size)
        }
    }

    fn _apply_partial_gravity(
        &mut self,
        direction: Direction,
        round_rocks: usize,
        cross_idx: usize,
        grav_idxs: Range<usize>,
    ) {
        let mut remaining = round_rocks;
        for grav_idx in grav_idxs {
            let coord = Coord::from_gravity(cross_idx, grav_idx, direction, self.size);
            self[coord] = if remaining > 0 {
                remaining -= 1;
                Rock::Round
            } else {
                Rock::Empty
            };
        }
    }

    fn load(&self) -> u64 {
        let mut load: u64 = 0;
        for (ri, row) in self.rocks.iter().enumerate() {
            for &rock in row {
                if rock == Rock::Round {
                    load += (self.size - ri) as u64
                }
            }
        }

        load
    }

    fn state_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);

        hasher.finish()
    }

    fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }
}

impl Debug for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('\n')?;
        for row in &self.rocks {
            for col in row {
                f.write_char((*col).into())?;
            }
            f.write_char('\n')?;
        }

        fmt::Result::Ok(())
    }
}

impl Deref for Platform {
    type Target = Vec<Vec<Rock>>;
    fn deref(&self) -> &Self::Target {
        &self.rocks
    }
}

impl DerefMut for Platform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rocks
    }
}

impl FromIterator<Vec<Rock>> for Platform {
    fn from_iter<T: IntoIterator<Item = Vec<Rock>>>(iter: T) -> Self {
        let rocks: Vec<Vec<Rock>> = iter.into_iter().collect();
        let size = rocks.len();

        Self { rocks, size }
    }
}

impl FromStr for Platform {
    type Err = Day14Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_whitespace()
            .map(|line| {
                line.chars()
                    .map(|c| Rock::try_from(c))
                    .collect::<Result<Vec<Rock>, Day14Error>>()
            })
            .collect()
    }
}

impl Index<Coord> for Platform {
    type Output = Rock;
    fn index(&self, index: Coord) -> &Self::Output {
        &self.rocks[index.row][index.col]
    }
}

impl IndexMut<Coord> for Platform {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.rocks[index.row][index.col]
    }
}

struct Part1;
struct Part2 {
    iterations: usize,
}

impl Solver for Part1 {
    type Input = Platform;
    type Output = u64;

    fn solve(&self, item: &Self::Input) -> Self::Output {
        let mut platform = Platform::clone(item);
        platform.tilt(Direction::North);
        platform.load()
    }
}

impl Part2 {
    fn new(iterations: usize) -> Self {
        Self { iterations }
    }
}

#[derive(Debug)]
struct Cycle {
    offset: usize,
    length: usize,
}

impl Solver for Part2 {
    type Input = Platform;
    type Output = u64;

    fn solve(&self, item: &Self::Input) -> Self::Output {
        let mut first_observations: Vec<u64> = Vec::new();
        let mut platform = Platform::clone(item);

        let mut iteration = 0;
        let cycle: Option<Cycle> = loop {
            if iteration == self.iterations {
                break None;
            }

            let state_hash = platform.state_hash();

            if let Some(first) = first_observations.iter().position(|&h| h == state_hash) {
                let cycle = Cycle {
                    offset: first,
                    length: iteration - first,
                };
                break Some(cycle);
            };

            first_observations.push(state_hash);

            platform.cycle();
            iteration += 1;
        };

        let final_platform = match cycle {
            Some(Cycle { offset, length }) => {
                let equivalent_iterations = offset + ((self.iterations - offset) % length);
                let mut platform = Platform::clone(item);
                for _ in 0..equivalent_iterations {
                    platform.cycle();
                }
                platform
            }
            None => platform,
        };

        final_platform.load()
    }
}

impl DailyChallenge for Day14 {
    fn part1(&self, file: &Path) -> u64 {
        let platform: Platform = fs::read_to_string(file).unwrap().parse().unwrap();
        Part1.solve(&platform)
    }

    fn part2(&self, file: &Path) -> u64 {
        let platform: Platform = fs::read_to_string(file).unwrap().parse().unwrap();
        let solver = Part2::new(1_000_000_000);
        solver.solve(&platform)
    }
}

#[test]
fn test_part1() {
    let platform: Platform = fs::read_to_string(Path::new("data/14.sample"))
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(Part1.solve(&platform), 136)
}

#[test]
fn test_part2() {
    let platform: Platform = fs::read_to_string(Path::new("data/14.sample"))
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(Part2::new(1_000_000_000).solve(&platform), 64)
}

#[test]
fn test_tilt_north() {
    let mut platform: Platform = fs::read_to_string(Path::new("data/14.sample"))
        .unwrap()
        .parse()
        .unwrap();
    let tilted = "
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
        .parse()
        .unwrap();

    platform.tilt(Direction::North);
    assert_eq!(platform, tilted)
}

#[test]
fn test_tilt_west() {
    let mut platform: Platform = fs::read_to_string(Path::new("data/14.sample"))
        .unwrap()
        .parse()
        .unwrap();
    let tilted = "
O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#...."
        .parse()
        .unwrap();

    platform.tilt(Direction::West);
    assert_eq!(platform, tilted)
}

#[test]
fn test_tilt_south() {
    let mut platform: Platform = fs::read_to_string(Path::new("data/14.sample"))
        .unwrap()
        .parse()
        .unwrap();
    let tilted = "
.....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O"
        .parse()
        .unwrap();

    platform.tilt(Direction::South);
    assert_eq!(platform, tilted)
}

#[test]
fn test_tilt_east() {
    let mut platform: Platform = fs::read_to_string(Path::new("data/14.sample"))
        .unwrap()
        .parse()
        .unwrap();
    let tilted = "
....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#...."
        .parse()
        .unwrap();

    platform.tilt(Direction::East);
    assert_eq!(platform, tilted)
}

#[test]
fn test_cycle() {
    let mut platform: Platform = fs::read_to_string(Path::new("data/14.sample"))
        .unwrap()
        .parse()
        .unwrap();
    let cycled = "
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
        .parse()
        .unwrap();
    platform.cycle();
    assert_eq!(platform, cycled)
}
