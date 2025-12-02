use crate::Day;

pub struct Day01 {
    nums: Vec<i64>,
}

impl Day for Day01 {
    const DAY: u8 = 1;

    fn parse(input: &str) -> Self {
        let nums = input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.trim().parse::<i64>().expect("expected integer in input"))
            .collect();

        Day01 { nums }
    }

    fn part1(&self) -> String {
        let sum: i64 = self.nums.iter().sum();
        sum.to_string()
    }

    fn part2(&self) -> String {
        let count = self.nums.iter().filter(|&&x| x > 0).count();
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn part1_example() {
        let d = Day01::parse(EXAMPLE_INPUT);
        assert_eq!(d.part1(), "3");
    }

    #[test]
    fn part2_example() {
        let d = Day01::parse(EXAMPLE_INPUT);
        assert_eq!(d.part2(), "3");
    }
}
