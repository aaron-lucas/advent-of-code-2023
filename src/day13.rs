use crate::challenge::DailyChallenge;
use std::ops::Deref;
use std::path::Path;
use std::str::FromStr;
use std::{fs, io};

#[derive(Default)]
pub struct Day13;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Terrain {
    Ash,
    Rock,
}

type TerrainGrid = Vec<Vec<Terrain>>;

#[derive(Debug)]
enum Error {
    InvalidTerrain,
    IOError(io::Error),
}

impl TryFrom<char> for Terrain {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Terrain::Ash),
            '#' => Ok(Terrain::Rock),
            _ => Err(Error::InvalidTerrain),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Note {
    terrain: TerrainGrid,
}

trait Transpose {
    fn transpose(&self) -> Self;
}

impl<T> Transpose for Vec<Vec<T>>
where
    T: Copy,
{
    fn transpose(&self) -> Self {
        let height = self.len();
        let width = self.first().map(Vec::len).unwrap_or(0);
        let columns: Vec<Vec<T>> = (0..width)
            .map(|col| (0..height).map(|row| self[row][col]).collect::<Vec<T>>())
            .collect();

        columns
    }
}

impl Transpose for Note {
    fn transpose(&self) -> Self {
        Self {
            terrain: self.terrain.transpose(),
        }
    }
}

impl FromStr for Note {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let terrain = s
            .split_whitespace()
            .map(|line| {
                line.chars()
                    .map(|c| Terrain::try_from(c))
                    .collect::<Result<Vec<Terrain>, Error>>()
            })
            .collect::<Result<TerrainGrid, Error>>()?;

        Ok(Self { terrain })
    }
}

#[derive(Debug, PartialEq)]
struct Notes(Vec<Note>);

impl Deref for Notes {
    type Target = Vec<Note>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Notes {
    fn from_file(file: &Path) -> Result<Self, Error> {
        let contents = fs::read_to_string(file).map_err(|e| Error::IOError(e))?;
        contents.parse()
    }
}

impl FromStr for Notes {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let notes = s
            .split("\n\n")
            .map(|s| s.parse::<Note>())
            .collect::<Result<Vec<Note>, Error>>()?;
        Ok(Self(notes))
    }
}

trait Solver {
    fn find_horizontal_reflection(note: &Note) -> Option<usize>;
    fn find_vertical_reflection(note: &Note) -> Option<usize>;

    fn summarize(note: &Note) -> usize {
        if let Some(v) = Self::find_vertical_reflection(note) {
            v
        } else {
            100 * Self::find_horizontal_reflection(note).unwrap_or(0)
        }
    }

    fn summarize_notes(notes: &Notes) -> u64 {
        notes.iter().map(|n| Self::summarize(n) as u64).sum()
    }
}

struct Part1;
struct Part2;

impl Solver for Part1 {
    fn find_horizontal_reflection(note: &Note) -> Option<usize> {
        let height = note.terrain.len();
        for row in 1..height {
            let (above, below) = note.terrain.split_at(row);
            let mut pairs = above.iter().rev().zip(below.iter());

            if pairs.all(|(a, b)| a == b) {
                return Some(row);
            }
        }
        None
    }

    fn find_vertical_reflection(note: &Note) -> Option<usize> {
        let transpose = note.transpose();
        Self::find_horizontal_reflection(&transpose)
    }
}

impl Solver for Part2 {
    fn find_horizontal_reflection(note: &Note) -> Option<usize> {
        let height = note.terrain.len();

        for row in 1..height {
            let (above, below) = note.terrain.split_at(row);
            let row_pairs = above.iter().rev().zip(below.iter());

            let mut differences = 0;
            for (row_a, row_b) in row_pairs {
                let item_pairs = row_a.iter().zip(row_b.iter());
                for (&aa, &bb) in item_pairs {
                    if aa != bb {
                        differences += 1;
                    }
                }
            }

            if differences == 1 {
                println!("Found {row}");
                return Some(row);
            }
        }

        None
    }

    fn find_vertical_reflection(note: &Note) -> Option<usize> {
        let transposed = note.transpose();
        Self::find_horizontal_reflection(&transposed)
    }
}

impl DailyChallenge for Day13 {
    fn part1(&self, file: &Path) -> u64 {
        let notes: Notes = fs::read_to_string(file).unwrap().parse().unwrap();

        Part1::summarize_notes(&notes)
    }

    fn part2(&self, file: &Path) -> u64 {
        let notes: Notes = fs::read_to_string(file).unwrap().parse().unwrap();

        Part2::summarize_notes(&notes)
    }
}

#[test]
fn test_from_string() {
    use Terrain::*;
    let string = "#..\n.#.\n.#.\n\n.##\n.##\n...";

    let note1 = Note {
        terrain: vec![
            vec![Rock, Ash, Ash],
            vec![Ash, Rock, Ash],
            vec![Ash, Rock, Ash],
        ],
    };

    let note2 = Note {
        terrain: vec![
            vec![Ash, Rock, Rock],
            vec![Ash, Rock, Rock],
            vec![Ash, Ash, Ash],
        ],
    };

    assert_eq!(string.parse::<Notes>().unwrap(), Notes(vec![note1, note2]));
}

#[test]
fn test_transpose() {
    use Terrain::*;
    let original = Note {
        terrain: vec![vec![Rock, Rock], vec![Ash, Ash]],
    };

    let transposed = Note {
        terrain: vec![vec![Rock, Ash], vec![Rock, Ash]],
    };

    assert_eq!(original.transpose(), transposed);
}

#[test]
fn test_part1() {
    let notes = Notes::from_file(Path::new("data/13.sample")).unwrap();
    assert_eq!(Part1::summarize_notes(&notes), 405);
}

#[test]
fn test_part2() {
    let notes = Notes::from_file(Path::new("data/13.sample")).unwrap();
    assert_eq!(Part2::summarize_notes(&notes), 400);
}
