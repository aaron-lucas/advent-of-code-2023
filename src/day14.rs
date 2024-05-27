use crate::challenge::DailyChallenge;
use std::ops::Deref;
use std::{fs, io};
use std::path::Path;
use std::str::FromStr;
use std::error::Error;
use std::fmt;

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
            E::IOError(e) => e.fmt(f)
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

struct Platform(Vec<Vec<Rock>>);

impl Deref for Platform {
    type Target = Vec<Vec<Rock>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<Vec<Rock>> for Platform {
    fn from_iter<T: IntoIterator<Item = Vec<Rock>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl FromStr for Platform {
    type Err = Day14Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_whitespace().map(|line| {
            line.chars().map(|c| Rock::try_from(c)).collect::<Result<Vec<Rock>, Day14Error>>()
        }).collect()
    }
}

trait Solver {
    type Input;
    type Output;

    fn solve(item: &Self::Input) -> Self::Output;
}

struct Part1;
struct Part2;

impl Solver for Part1 {
    type Input = Platform;
    type Output = u64;

    fn solve(item: &Self::Input) -> Self::Output {
        let height = item.len();
        let width = item.get(0).and_then(|row| Some(row.len())).unwrap_or(0);

        let mut load: u64 = 0;
        for col in 0..width {
            let mut furthest = height;
            for row in 0..height {
                let rock: Rock = item.get(row).unwrap().get(col).copied().unwrap();
                match rock {
                    Rock::Round => {
                        load += furthest as u64;
                        furthest -= 1;
                    },
                    Rock::Cube => furthest = height - row - 1,
                    Rock::Empty => {},
                }
            }
        }

        load
    }
}

impl Solver for Part2 {
    type Input = Platform;
    type Output = u64;

    fn solve(item: &Self::Input) -> Self::Output {
        0
    }
}

impl DailyChallenge for Day14 {
    fn part1(&self, file: &Path) -> u64 {
        let platform: Platform = fs::read_to_string(file).unwrap().parse().unwrap();
        Part1::solve(&platform)
    }

    fn part2(&self, file: &Path) -> u64 {
        todo!()
    }
}

#[test]
fn test_part1() {
    let platform: Platform = fs::read_to_string(Path::new("data/14.sample")).unwrap().parse().unwrap();
    assert_eq!(Part1::solve(&platform), 136)
}

#[test]
fn test_part2() {
    todo!()
}


