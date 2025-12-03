use crate::Day;

pub struct Day01 {
    moves: Vec<i16>,
}
impl Day01 {
    fn parse_move(token: &str) -> i16 {
        let mut it = token.chars();
        let dir = it.next().expect("empty move token");
        let sign = match dir {
            'L' => -1,
            'R' => 1,
            _ => panic!("unexpected direction: {dir}"),
        };
        let number: i16 = it.as_str().parse().expect("invalid number");
        sign * number
    }
}

impl Day for Day01 {
    const DAY: u8 = 1;

    fn parse(input: &str) -> Self {
        let moves = input.split_whitespace().map(Self::parse_move).collect();

        Day01 { moves }
    }

    fn part1(&self) -> String {
        const START: i16 = 50;
        const MOD: i16 = 100;

        let mut position = START;
        let mut count = 0;

        for delta in &self.moves {
            position = (position + delta).rem_euclid(MOD);

            if position == 0 {
                count += 1;
            }
        }

        count.to_string()
    }

    fn part2(&self) -> String {
        const START: i16 = 50;
        const MOD: i16 = 100;

        let mut position = START;
        let mut count = 0;

        for delta in &self.moves {
            count += (delta / MOD).abs(); // Full rotations
            let start = position;
            let remaining = delta % MOD;

            position += remaining;

            // No extra clicks if we are at 0
            if start != 0 && (position <= 0 || position >= MOD) {
                count += 1;
            }

            position = position.rem_euclid(MOD);
        }

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
        assert_eq!(d.part2(), "6");
    }
}
