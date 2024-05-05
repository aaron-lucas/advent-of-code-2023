use std::path::Path;

pub trait DailyChallenge {
    fn part1(&self, file: &Path) -> u64;
    fn part2(&self, file: &Path) -> u64;
}
