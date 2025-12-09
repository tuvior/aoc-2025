use std::env;

use aoc_2025::days;
use aoc_2025::run_day;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: aoc-2025 <day> [part]");
        eprintln!("Example: aoc-2025 1      # runs both parts");
        eprintln!("         aoc-2025 1 2    # runs part 2 only");
        std::process::exit(1);
    }

    let day: u8 = args[1].parse().expect("day must be a number 1-25");
    let part: Option<u8> = args.get(2).and_then(|s| s.parse().ok());

    match day {
        1 => run_day::<days::day01::Day01>(part),
        2 => run_day::<days::day02::Day02>(part),
        3 => run_day::<days::day03::Day03>(part),
        4 => run_day::<days::day04::Day04>(part),
        5 => run_day::<days::day05::Day05>(part),
        6 => run_day::<days::day06::Day06>(part),
        7 => run_day::<days::day07::Day07>(part),
        _ => {
            eprintln!("Day {day} not implemented yet");
            std::process::exit(1);
        }
    }
}
