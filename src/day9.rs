use crate::challenge::DailyChallenge;
use std::fs;
use std::path::Path;

#[derive(Default)]
pub struct Day9;

#[derive(Eq, PartialEq, Debug)]
struct History(Vec<i32>);

impl History {
    fn diff(&self) -> Self {
        Self(self.0.windows(2).map(|pair| pair[1] - pair[0]).collect())
    }

    fn all_zero(&self) -> bool {
        return self.0.iter().all(|&h| h == 0);
    }

    fn most_recent(&self) -> Option<i32> {
        self.0.last().copied()
    }

    fn predict(&self) -> i32 {
        if self.all_zero() {
            0
        } else {
            self.most_recent().unwrap_or(0) + self.diff().predict()
        }
    }

    fn extrapolate(&self) -> i32 {
        if self.all_zero() {
            0
        } else {
            self.0.first().copied().unwrap_or(0) - self.diff().extrapolate()
        }
    }
}

impl FromIterator<i32> for History {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        Self(iter.into_iter().collect::<Vec<i32>>())
    }
}

#[derive(Eq, PartialEq, Debug)]
struct OASISReport {
    histories: Vec<History>,
}

impl OASISReport {
    fn from_file(file: &Path) -> Result<Self, String> {
        let contents = fs::read_to_string(file).map_err(|e| e.to_string())?;

        let histories = contents
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse::<i32>().map_err(|e| e.to_string()))
                    .collect::<Result<History, String>>()
            })
            .collect::<Result<Vec<History>, String>>()?;

        Ok(Self { histories })
    }

    fn predict_all(&self) -> Vec<i32> {
        self.histories.iter().map(History::predict).collect()
    }

    fn extrapolate_all(&self) -> Vec<i32> {
        self.histories.iter().map(History::extrapolate).collect()
    }
}

impl DailyChallenge for Day9 {
    fn part1(&self, file: &Path) -> u64 {
        let report = OASISReport::from_file(file).unwrap();
        report.predict_all().iter().map(|&h| h as i64).sum::<i64>() as u64
    }

    fn part2(&self, file: &Path) -> u64 {
        let report = OASISReport::from_file(file).unwrap();
        report
            .extrapolate_all()
            .iter()
            .map(|&h| h as i64)
            .sum::<i64>() as u64
    }
}

#[test]
fn test_from_file() {
    let report = OASISReport::from_file(Path::new("data/9.sample")).expect("Test file missing");
    let expected = OASISReport {
        histories: vec![
            History(vec![0, 3, 6, 9, 12, 15]),
            History(vec![1, 3, 6, 10, 15, 21]),
            History(vec![10, 13, 16, 21, 30, 45]),
        ],
    };
    assert_eq!(report, expected);
}

#[test]
fn test_part1() {
    assert_eq!(Day9.part1(Path::new("data/9.sample")), 114)
}

#[test]
fn test_part2() {
    assert_eq!(Day9.part2(Path::new("data/9.sample")), 2)
}
