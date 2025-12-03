use crate::Day;

pub struct Day03 {
    batteries: Vec<Vec<u64>>,
}

impl Day for Day03 {
    const DAY: u8 = 3;

    fn parse(input: &str) -> Self {
        let batteries = input
            .split_whitespace()
            .map(|bank| {
                bank.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect()
            })
            .collect();

        Day03 { batteries }
    }

    fn part1(&self) -> String {
        let mut total = 0;

        for bank in &self.batteries {
            let bank_size = bank.len();

            let mut biggest = 0;
            let mut index = 0;

            for i in 0..bank_size - 1 {
                let bat = bank[i];

                if bat > biggest {
                    index = i;
                    biggest = bat;
                }
            }

            let mut second_biggest = 0;

            for i in index + 1..bank_size {
                let bat = bank[i];

                if bat > second_biggest {
                    second_biggest = bat;
                }
            }

            total += biggest * 10 + second_biggest;
        }

        total.to_string()
    }

    fn part2(&self) -> String {
        const SIZE: usize = 12;
        let mut total = 0;

        for bank in &self.batteries {
            let bank_size = bank.len();
            let mut cut = 0;

            let mut bat_power = 0;

            for pow in (1..=SIZE).rev() {
                let mut biggest = 0;
                let mut index = cut;

                for i in cut..=bank_size - pow {
                    let bat = bank[i];

                    if bat > biggest {
                        index = i;
                        biggest = bat;
                    }
                }

                cut = index + 1;
                bat_power += biggest * 10u64.pow((pow - 1) as u32);
            }

            total += bat_power;
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

    const EXAMPLE_OUTPUT_1: &str = "357";
    const EXAMPLE_OUTPUT_2: &str = "3121910778619";

    #[test]
    fn part1_example() {
        let d = Day03::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part1(), EXAMPLE_OUTPUT_1);
    }

    #[test]
    fn part2_example() {
        let d = Day03::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part2(), EXAMPLE_OUTPUT_2);
    }
}
