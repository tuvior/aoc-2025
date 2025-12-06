use crate::Day;

pub struct Day06 {
    raw: String,
    nums: Vec<Vec<i64>>,
    operators: Vec<Operator>,
}

enum Operator {
    Plus,
    Multiply,
}

impl Day for Day06 {
    const DAY: u8 = 6;

    fn parse(input: &str) -> Self {
        let rows: Vec<Vec<&str>> = input
            .lines()
            .map(|r| r.split_whitespace().collect())
            .collect();

        let (operators, numbers) = rows.split_last().unwrap();
        let cols = numbers.first().unwrap().len();

        let mut nums = vec![];

        for j in 0..cols {
            let mut current = vec![];

            for row in numbers {
                current.push(row[j].parse().unwrap())
            }

            nums.push(current);
        }

        let operators = operators
            .iter()
            .map(|o| match *o {
                "*" => Operator::Multiply,
                "+" => Operator::Plus,
                _ => panic!("unexpected operator: {o}"),
            })
            .collect();

        Day06 {
            raw: input.to_string(),
            nums,
            operators,
        }
    }

    fn part1(&self) -> String {
        let mut total: i64 = 0;
        for (operator, nums) in self.operators.iter().zip(&self.nums) {
            let result: i64 = match operator {
                Operator::Plus => nums.iter().copied().sum(),
                Operator::Multiply => nums.iter().copied().product(),
            };

            total += result;
        }
        total.to_string()
    }

    fn part2(&self) -> String {
        let mut numbers: Vec<&str> = self.raw.lines().collect();
        numbers.pop();

        let characters: Vec<Vec<char>> = numbers.iter().map(|s| s.chars().collect()).collect();
        let cols = characters.first().unwrap().len();

        let mut vnumbers: Vec<Vec<i64>> = vec![vec![]];
        for j in 0..cols {
            let mut number = String::new();
            for row in &characters {
                let char = row[j];

                if char != ' ' {
                    number.push(char);
                }
            }

            match number.parse::<i64>() {
                Ok(n) => vnumbers.last_mut().unwrap().push(n),
                Err(_) => vnumbers.push(vec![]),
            }
        }

        let mut total: i64 = 0;
        for (operator, nums) in self.operators.iter().zip(&vnumbers) {
            let result: i64 = match operator {
                Operator::Plus => nums.iter().copied().sum(),
                Operator::Multiply => nums.iter().copied().product(),
            };

            total += result;
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    const EXAMPLE_OUTPUT_1: &str = "4277556";
    const EXAMPLE_OUTPUT_2: &str = "3263827";

    #[test]
    fn part1_example() {
        let d = Day06::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part1(), EXAMPLE_OUTPUT_1);
    }

    #[test]
    fn part2_example() {
        let d = Day06::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part2(), EXAMPLE_OUTPUT_2);
    }
}
