use crate::challenge::{DailyChallenge, Solver};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::str::FromStr;
use std::{fs, io};

#[derive(Default)]
pub struct Day15;

#[derive(Debug)]
enum Day15Error {
    NotASCII,
    InvalidAction,
    IOError(io::Error),
}

impl From<io::Error> for Day15Error {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl Error for Day15Error {}

impl fmt::Display for Day15Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Day15Error as E;
        match self {
            E::NotASCII => write!(f, "Value is not ASCII"),
            E::InvalidAction => write!(f, "Could not parse to action"),
            E::IOError(e) => std::fmt::Display::fmt(&e, f),
        }
    }
}

#[derive(Clone)]
struct Lens(u8);

#[derive(Clone)]
struct LabelledLens {
    lens: Lens,
    label: String,
}

#[derive(Default, Clone)]
struct LensBox(Vec<LabelledLens>);

impl Deref for LensBox {
    type Target = Vec<LabelledLens>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LensBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

enum Action {
    RemoveLens(String),
    InsertLens(LabelledLens),
}

struct Operation {
    box_number: u8,
    action: Action,
}

struct LightFocuser {
    boxes: Vec<LensBox>,
}

impl Default for LightFocuser {
    fn default() -> Self {
        Self {
            boxes: vec![LensBox::default(); 256],
        }
    }
}

impl LightFocuser {
    fn get_operation(&self, instruction: &str) -> Result<Operation, Day15Error> {
        match instruction.split_once(|c| c == '=' || c == '-') {
            Some((lens_label, other)) => {
                let box_number = compute_hash(lens_label.as_bytes()) as u8;
                let lens_label = String::from(lens_label);

                let action = if other.is_empty() {
                    Action::RemoveLens(lens_label)
                } else {
                    let focal_length: u8 =
                        other.parse().expect("Instruction should have focal length");
                    let lens = LabelledLens {
                        lens: Lens(focal_length),
                        label: lens_label,
                    };
                    Action::InsertLens(lens)
                };

                Ok(Operation { box_number, action })
            }
            None => Err(Day15Error::InvalidAction),
        }
    }

    fn apply_operation(&mut self, operation: Operation) {
        let lens_box = self
            .boxes
            .get_mut(operation.box_number as usize)
            .expect("Box numbers should all be valid");

        match operation.action {
            Action::RemoveLens(label) => {
                let existing_lens = lens_box.iter().position(|l| l.label == *label);
                if let Some(position) = existing_lens {
                    lens_box.remove(position);
                }
            }
            Action::InsertLens(lens) => {
                let existing_lens = lens_box.iter().position(|l| l.label == lens.label);
                if let Some(position) = existing_lens {
                    // Replace existing lens
                    lens_box[position] = lens
                } else {
                    // Add new lens
                    lens_box.push(lens)
                }
            }
        }
    }

    fn focusing_power(&self) -> u64 {
        let mut power = 0;
        for (box_number, lens_box) in self.boxes.iter().enumerate() {
            for (slot_number, lens) in lens_box.iter().enumerate() {
                let slot_number = slot_number as u64 + 1;
                let focal_length = lens.lens.0 as u64;
                power += (1 + box_number as u64) * slot_number * focal_length;
            }
        }

        power
    }
}

#[derive(Debug, PartialEq)]
struct InitSequence(Vec<String>);

impl FromStr for InitSequence {
    type Err = Day15Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut strings: Vec<String> = Vec::new();
        for string in s.split(",") {
            if !s.is_ascii() {
                return Err(Day15Error::NotASCII);
            }
            strings.push(String::from(string.trim()));
        }

        Ok(Self(strings))
    }
}

impl Deref for InitSequence {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn compute_hash(ascii: &[u8]) -> u64 {
    let mut value: u64 = 0;
    for &ch in ascii {
        value += ch as u64;
        value *= 17;
        value %= 256;
    }

    value
}

struct Part1;
struct Part2;

impl Solver for Part1 {
    type Input = InitSequence;
    type Output = u64;

    fn solve(&self, item: &Self::Input) -> Self::Output {
        item.iter().map(|x| compute_hash(x.as_bytes())).sum()
    }
}

impl Solver for Part2 {
    type Input = InitSequence;
    type Output = u64;

    fn solve(&self, item: &Self::Input) -> Self::Output {
        let mut focuser = LightFocuser::default();
        for op in item.iter() {
            let operation = focuser
                .get_operation(&op)
                .expect("Operations should be valid");
            focuser.apply_operation(operation);
        }

        focuser.focusing_power()
    }
}

impl DailyChallenge for Day15 {
    fn part1(&self, file: &Path) -> u64 {
        let seq: InitSequence = fs::read_to_string(file).unwrap().parse().unwrap();
        Part1.solve(&seq)
    }

    fn part2(&self, file: &Path) -> u64 {
        let seq: InitSequence = fs::read_to_string(file).unwrap().parse().unwrap();
        Part2.solve(&seq)
    }
}

#[test]
fn test_from_str() {
    let string = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";
    let expected: Vec<String> = [
        "rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7",
    ]
    .iter()
    .map(|&s| String::from(s))
    .collect();
    let expected = InitSequence(expected);

    let parsed: InitSequence = string.parse().unwrap();

    assert_eq!(parsed, expected);
}

#[test]
fn test_part1() {
    let seq: InitSequence = fs::read_to_string(Path::new("data/15.sample"))
        .unwrap()
        .parse()
        .unwrap();
    let states: Vec<u64> = vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231];

    for (string, &expected) in seq.iter().zip(states.iter()) {
        assert_eq!(compute_hash(string.as_bytes()), expected);
    }

    assert_eq!(Part1.solve(&seq), 1320)
}

#[test]
fn test_part2() {
    let seq: InitSequence = fs::read_to_string(Path::new("data/15.sample"))
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(Part2.solve(&seq), 145)
}
