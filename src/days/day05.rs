use crate::Day;

pub struct Day05 {
    ranges: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}

impl Day for Day05 {
    const DAY: u8 = 5;

    fn parse(input: &str) -> Self {
        let (part1, part2) = input.split_once("\n\n").unwrap();

        let ranges = part1
            .lines()
            .map(|l| {
                let (a, b) = l.split_once("-").unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect();

        let ingredients = part2.lines().map(str::parse).map(Result::unwrap).collect();

        Day05 {
            ranges,
            ingredients,
        }
    }

    fn part1(&self) -> String {
        let mut count = 0;

        for ingredient in &self.ingredients {
            for (min, max) in &self.ranges {
                if ingredient >= min && ingredient <= max {
                    count += 1;
                    break;
                }
            }
        }

        count.to_string()
    }

    fn part2(&self) -> String {
        let mut ranges = self.ranges.clone();
        ranges.sort_by_key(|(a, _)| *a);

        let mut new_ranges = vec![ranges[0]];

        for (min, max) in ranges {
            let to_compare = new_ranges.last_mut().unwrap();

            // widen the range in cases like [1-5] [3-8] => [1-8]
            if min <= to_compare.1 {
                if max > to_compare.1 {
                    to_compare.1 = max;
                }
            } else {
                new_ranges.push((min, max));
            }
        }

        let total_fresh = new_ranges
            .iter()
            .fold(0, |sum, (min, max)| sum + max - min + 1);

        total_fresh.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    const EXAMPLE_OUTPUT_1: &str = "3";
    const EXAMPLE_OUTPUT_2: &str = "14";

    #[test]
    fn part1_example() {
        let d = Day05::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part1(), EXAMPLE_OUTPUT_1);
    }

    #[test]
    fn part2_example() {
        let d = Day05::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part2(), EXAMPLE_OUTPUT_2);
    }
}
