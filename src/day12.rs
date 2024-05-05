use crate::challenge::DailyChallenge;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(PartialEq, Clone, Copy, Debug, Hash, Eq)]
enum Spring {
    Fine,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Spring::Fine),
            '#' => Ok(Spring::Damaged),
            '?' => Ok(Spring::Unknown),
            _ => Err(format!("Invalid spring '{value}'")),
        }
    }
}

#[derive(PartialEq, Debug)]
struct SpringRecord {
    springs: Vec<Spring>,
    damaged_groups: Vec<usize>,
}

impl SpringRecord {
    fn new(springs: Vec<Spring>, damaged_groups: Vec<usize>) -> Self {
        Self {
            springs,
            damaged_groups,
        }
    }

    fn from_string(string: &str) -> Result<Self, String> {
        let mut components = string.split_whitespace();

        let springs = components.next().ok_or("Could not get springs")?;
        let springs = springs
            .chars()
            .map(Spring::try_from)
            .collect::<Result<Vec<Spring>, String>>()?;

        let damaged_groups = components.next().ok_or("Could not get damaged groups")?;
        let damaged_groups = damaged_groups
            .split(",")
            .map(|n| n.parse::<usize>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<usize>, String>>()?;

        Ok(Self {
            springs,
            damaged_groups,
        })
    }

    fn vec_from_file(file: &Path) -> Result<Vec<Self>, String> {
        let contents = fs::read_to_string(file).map_err(|e| e.to_string())?;
        contents.lines().map(Self::from_string).collect()
    }

    fn is_valid(&self, springs: &Vec<Spring>) -> bool {
        let mut group_size = 0;
        let mut expected_iter = self.damaged_groups.iter();
        let mut expected_size = expected_iter.next().copied().unwrap();

        for spring in springs {
            match spring {
                Spring::Fine => {
                    if group_size > 0 {
                        if expected_size != group_size {
                            return false;
                        }
                        group_size = 0;
                        expected_size = expected_iter.next().copied().unwrap_or(0);
                    }
                }
                Spring::Damaged => {
                    group_size += 1;
                    if group_size > expected_size {
                        return false;
                    }
                }
                Spring::Unknown => {
                    return false;
                }
            }
        }

        // Handle groups at the end
        if expected_size != group_size {
            return false;
        } else if expected_iter.next() != None {
            // Finish on a correct group but there's more groups to match
            return false;
        }

        true
    }

    fn unfold(&self, n: usize) -> SpringRecord {
        let mut springs: Vec<Spring> = Vec::with_capacity((self.springs.len() + 1) * n);
        for _ in 0..(n - 1) {
            springs.extend(self.springs.clone());
            springs.push(Spring::Unknown);
        }
        springs.extend(self.springs.clone());

        SpringRecord::new(
            springs,
            self.damaged_groups.repeat(n),
        )
    }
}

struct CachedSolver {
    cache: HashMap<(Vec<Spring>, Vec<usize>, usize), usize>,
}

impl CachedSolver {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn solve_record(&mut self, record: &SpringRecord) -> usize {
        self.solve(&record.springs, &record.damaged_groups, 0)
    }

    fn solve(&mut self, springs: &[Spring], groups: &[usize], current: usize) -> usize {
        let key = (springs.to_vec(), groups.to_vec(), current);
        if let Some(&result) = self.cache.get(&key) {
            return result;
        };

        let result = self.solve_uncached(springs, groups, current);
        self.cache.insert(key, result);

        result
    }

    fn solve_uncached(&mut self, springs: &[Spring], groups: &[usize], current: usize) -> usize {
        // springs: sequence of springs yet to consider
        // groups: damaged spring group sizes yet to find
        // current: number of springs in current damaged group

        let Some(spring) = springs.first() else {
            // No springs left - have we matched all damaged groups?
            let [group_size, other_groups @ ..] = &groups[..] else {
                // no groups left
                // valid if we are not in a damaged group
                return (current == 0) as usize;
            };

            if !other_groups.is_empty() {
                // multiple groups need matching but we've run out of springs
                return 0;
            }

            return (current == *group_size) as usize;
        };

        let Some(&group_size) = groups.first() else {
            // No groups left to find
            if current > 0 {
                return 0;
            } else if springs.contains(&Spring::Damaged) {
                // Matched all groups but there are more damaged springs which is impossible
                return 0;
            } else {
                // Any unknown springs are fine - a single combination
                return 1;
            }
        };

        let rest = &springs[1..];

        match spring {
            Spring::Fine => {
                if current == 0 {
                    return self.solve(rest, groups, 0);
                } else {
                    // Finished a damaged spring group
                    if current == group_size {
                        return self.solve(rest, &groups[1..], 0);
                    } else {
                        // Found a damaged group which is not the right size
                        return 0;
                    }
                }
            },
            Spring::Damaged => {
                if current >= group_size {
                    // This group is bigger than the expected size
                    return 0;
                } else {
                    return self.solve(rest, groups, current + 1)
                }
            },
            Spring::Unknown => {
                if current == 0 {
                    return
                        self.solve(rest, groups, 1)  // if this is damaged
                        + self.solve(rest, groups, 0); // if this is fine
                } else if current == group_size {
                    // Finished the group of damaged springs - move to next one.
                    // This spring is fine.
                    return self.solve(rest, &groups[1..], 0);
                } else {
                    // In the middle of a group - must be damaged
                    return self.solve(rest, groups, current + 1);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct Day12;

impl DailyChallenge for Day12 {
    fn part1(&self, file: &Path) -> u64 {
        let records = SpringRecord::vec_from_file(file).unwrap();
        let mut solver = CachedSolver::new();
        records
            .iter()
            .map(|r| solver.solve_record(r))
            .sum::<usize>() as u64
    }

    fn part2(&self, file: &Path) -> u64 {
        let records = SpringRecord::vec_from_file(file).unwrap();
        let mut solver = CachedSolver::new();
        records
            .iter()
            .map(|r| solver.solve_record(&r.unfold(5)))
            .sum::<usize>() as u64
    }
}

#[test]
fn test_from_string() {
    use Spring::*;
    let rec = SpringRecord::from_string("???.### 1,1,3").unwrap();
    let exp = SpringRecord {
        springs: vec![Unknown, Unknown, Unknown, Fine, Damaged, Damaged, Damaged],
        damaged_groups: vec![1, 1, 3],
    };

    assert_eq!(rec, exp);
}

#[test]
fn test_is_valid() {
    use Spring::*;
    let rec = SpringRecord::from_string("???.### 1,1,3").unwrap();
    let valid = vec![Damaged, Fine, Damaged, Fine, Damaged, Damaged, Damaged];
    let invalid = vec![Fine, Fine, Damaged, Fine, Damaged, Damaged, Damaged];
    assert!(rec.is_valid(&valid));
    assert!(!rec.is_valid(&invalid));
}

#[test]
fn test_unfolded_permuations() {
    let rec = SpringRecord::from_string(".??..??...?##. 1,1,3")
        .unwrap()
        .unfold(5);
    assert_eq!(CachedSolver::new().solve_record(&rec), 16384);
}

#[test]
fn test_part1() {
    assert_eq!(Day12.part1(Path::new("data/12.sample")), 21)
}

#[test]
fn test_part2() {
    assert_eq!(Day12.part2(Path::new("data/12.sample")), 525152)
}
