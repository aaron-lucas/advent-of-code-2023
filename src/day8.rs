use crate::challenge::DailyChallenge;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::iter::{Copied, Cycle};
use std::path::Path;
use std::slice::Iter;

#[derive(Default)]
pub struct Day8;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn go<'a, 'b>(&'a self, edges: &'b NodeEdges) -> &'b str {
        match self {
            Self::Left => edges.left.as_str(),
            Self::Right => edges.right.as_str(),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("Unknown direction"),
        }
    }
}

struct NodeEdges {
    left: String,
    right: String,
}

struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<String, NodeEdges>,
}
struct MapIterator<'a> {
    next: &'a str,
    directions: Cycle<Copied<Iter<'a, Direction>>>,
    nodes: &'a HashMap<String, NodeEdges>,
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let to_return = self.next;

        let Some(direction) = self.directions.next() else {
            return None;
        };

        let current_node = self.nodes.get(self.next).expect("Arrived at invalid node");
        let next_node = direction.go(current_node);
        self.next = next_node;

        Some(to_return)
    }
}

const PATTERN: &str = r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)";

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct LocDir<'a>(&'a str, usize);

impl Map {
    fn from_file(file: &Path) -> Result<Self, String> {
        let contents = fs::read_to_string(file).map_err(|e| e.to_string())?;

        let directions = contents
            .lines()
            .next()
            .ok_or("Invalid file format")?
            .chars()
            .map(Direction::try_from)
            .collect::<Result<Vec<Direction>, &str>>()?;

        let mut map = Map {
            directions,
            nodes: HashMap::new(),
        };

        let re = Regex::new(PATTERN).unwrap();
        for (_, [source, left, right]) in re.captures_iter(&contents).map(|c| c.extract()) {
            map.nodes.insert(
                source.to_string(),
                NodeEdges {
                    left: left.to_string(),
                    right: right.to_string(),
                },
            );
        }

        Ok(map)
    }

    fn walk<'a>(&'a self, start: &'a str) -> MapIterator {
        MapIterator {
            next: start,
            directions: self.directions.iter().copied().cycle(),
            nodes: &self.nodes,
        }
    }
}

fn steps_to_z(map: &Map, start: &str) -> u32 {
    let mut taken = 0;
    let mut map_path = map.walk(start);
    while let Some(node) = map_path.next() {
        if node.ends_with("Z") {
            break;
        }
        taken += 1;
    }

    taken
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let old_b = b;
        b = a % b;
        a = old_b;
    }
    a
}

fn lcm2(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn lcm(numbers: &[u64]) -> u64 {
    numbers.iter().copied().fold(1, lcm2)
}

impl DailyChallenge for Day8 {
    fn part1(&self, file: &Path) -> u64 {
        let map = Map::from_file(file).unwrap();
        let mut current = "AAA";

        for (step, dir) in map.directions.iter().cycle().enumerate() {
            if current == "ZZZ" {
                return step as u64;
            }

            let edges = map.nodes.get(current).unwrap();
            current = dir.go(&edges);
        }

        panic!();
    }

    fn part2(&self, file: &Path) -> u64 {
        let map = Map::from_file(file).unwrap();
        let paths: Vec<u64> = map
            .nodes
            .keys()
            .filter(|n| n.ends_with("A"))
            .map(|n| steps_to_z(&map, n) as u64)
            .collect();

        lcm(&paths)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day8.part1(Path::new("data/8.sample")), 2);
    }

    #[test]
    fn test_part1_cycle() {
        assert_eq!(Day8.part1(Path::new("data/8.sample2")), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day8.part2(Path::new("data/8.sample3")), 6);
    }
}
