pub mod days;

use std::fs;

pub trait Day {
    const DAY: u8;

    fn parse(input: &str) -> Self
    where
        Self: Sized;

    fn part1(&self) -> String;

    fn part2(&self) -> String;
}

pub fn run_day<D: Day>(part: Option<u8>) {
    let day = D::DAY;
    let path = format!("inputs/day{day:02}.txt");

    let input = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {path}: {e}"));

    let parsed = D::parse(&input.trim());

    match part {
        Some(1) => {
            let ans1 = parsed.part1();
            println!("Day {day:02} — part 1: {ans1}");
        }
        Some(2) => {
            let ans2 = parsed.part2();
            println!("Day {day:02} — part 2: {ans2}");
        }
        None => {
            let ans1 = parsed.part1();
            let ans2 = parsed.part2();
            println!("Day {day:02} — part 1: {ans1}");
            println!("Day {day:02} — part 2: {ans2}");
        }
        Some(other) => {
            panic!("Unknown part: {other}");
        }
    }
}
