use crate::Day;

pub struct Day02 {
    ranges: Vec<(i64, i64)>,
}

impl Day for Day02 {
    const DAY: u8 = 2;

    fn parse(input: &str) -> Self {
        let ranges = input
            .split(',')
            .map(|range| {
                let (from, to) = range.split_once('-').unwrap();
                (from.parse().unwrap(), to.parse().unwrap())
            })
            .collect();

        Day02 { ranges }
    }

    fn part1(&self) -> String {
        let mut total = 0;

        for &(from, to) in &self.ranges {
            for i in from..=to {
                let stringified = i.to_string();
                let len = stringified.len();

                if len % 2 == 0 {
                    let (first, second) = stringified.split_at(len / 2);

                    if first == second {
                        total += i;
                    }
                }
            }
        }

        total.to_string()
    }

    fn part2(&self) -> String {
        let mut total = 0;

        for &(from, to) in &self.ranges {
            for id in from..=to {
                let stringified = id.to_string();
                let len = stringified.len();

                for size in 2..=len {
                    if len % size != 0 {
                        continue;
                    }

                    let chunk_len = len / size;
                    let key = &stringified[0..chunk_len];

                    if (1..size).all(|i| &stringified[i * chunk_len..(i + 1) * chunk_len] == key) {
                        total += id;
                        break;
                    }
                }
            }
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

    #[test]
    fn part1_example() {
        let d = Day02::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part1(), "1227775554");
    }

    #[test]
    fn part2_example() {
        let d = Day02::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part2(), "4174379265");
    }
}
