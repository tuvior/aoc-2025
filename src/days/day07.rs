use std::{collections::HashSet, str::Chars};

use crate::Day;

pub struct Day07 {
    grid: Vec<Vec<char>>,
}

const START: char = 'S';
const EMPTY: char = '.';
const SPLITTER: char = '^';

impl Day for Day07 {
    const DAY: u8 = 7;

    fn parse(input: &str) -> Self {
        let grid = input.lines().map(str::chars).map(Chars::collect).collect();

        Day07 { grid }
    }

    fn part1(&self) -> String {
        let start_position = self.grid[0].iter().position(|p| *p == START).unwrap();

        let mut positions = vec![HashSet::from([start_position])];

        let mut split_count = 0;

        for (i, row) in self.grid.iter().enumerate().skip(1) {
            positions.push(HashSet::new());
            for beam in &positions[i - 1].clone() {
                match row[*beam] {
                    EMPTY => {
                        positions[i].insert(*beam);
                    }
                    SPLITTER => {
                        positions[i].insert(*beam - 1);
                        positions[i].insert(*beam + 1);
                        split_count += 1;
                    }
                    _ => (),
                }
            }
        }

        split_count.to_string()
    }

    fn part2(&self) -> String {
        let start_position = self.grid[0].iter().position(|p| *p == START).unwrap();

        let mut positions = vec![vec![0; self.grid[0].len()]; self.grid.len()];
        positions[0][start_position] = 1;

        for (i, row) in self.grid.iter().enumerate().skip(1) {
            for beam in 0..row.len() {
                match row[beam] {
                    EMPTY => {
                        positions[i][beam] += positions[i - 1][beam];
                    }
                    SPLITTER => {
                        positions[i][beam - 1] += positions[i - 1][beam];
                        positions[i][beam + 1] += positions[i - 1][beam];
                    }
                    _ => (),
                }
            }
        }

        positions.last().unwrap().iter().sum::<i64>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    const EXAMPLE_OUTPUT_1: &str = "21";
    const EXAMPLE_OUTPUT_2: &str = "40";

    #[test]
    fn part1_example() {
        let d = Day07::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part1(), EXAMPLE_OUTPUT_1);
    }

    #[test]
    fn part2_example() {
        let d = Day07::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part2(), EXAMPLE_OUTPUT_2);
    }
}
