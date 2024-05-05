use clap::{Parser, ValueEnum};
use std::path::Path;

mod challenge;
use challenge::DailyChallenge;

mod day10;
mod day11;
mod day12;
mod day7;
mod day8;
mod day9;

#[derive(ValueEnum, Clone)]
enum Mode {
    Part1,
    Part2,
}

#[derive(Parser)]
struct Args {
    day: u8,
    mode: Mode,
    file: String,
}

fn main() {
    let challenges: Vec<Box<dyn DailyChallenge>> = vec![
        Box::new(day7::Day7::default()),
        Box::new(day8::Day8::default()),
        Box::new(day9::Day9::default()),
        Box::new(day10::Day10::default()),
        Box::new(day11::Day11::default()),
        Box::new(day12::Day12::default()),
    ];

    let args = Args::parse();
    let path = Path::new(&args.file);
    let index = (args.day as usize) - (7_usize);
    let challenge = &challenges[index];
    let result = match args.mode {
        Mode::Part1 => challenge.part1(path),
        Mode::Part2 => challenge.part2(path),
    };
    println!("{result}");
}
