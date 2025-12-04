use crate::Day;

pub struct Day04 {
    roll_grid: Vec<Vec<char>>,
}

const ROLL: char = '@';
const EMPTY: char = '.';
const MAX_NEIGH: u8 = 4;

fn neighbors_at(grid: &[Vec<char>], x: usize, y: usize) -> u8 {
    let mut n_count = 0;
    let rows = grid.len();
    let cols = grid.first().expect("Empty grid").len();

    // I hate this, probably would be able to make this nicer if I knew rust types
    for ix in x.saturating_sub(1)..=(x + 1).clamp(0, rows - 1) {
        for iy in y.saturating_sub(1)..=(y + 1).clamp(0, cols - 1) {
            if ix == x && iy == y {
                continue;
            }

            let val = grid[ix][iy];

            if val == ROLL {
                n_count += 1;
            }
        }
    }

    n_count
}

impl Day for Day04 {
    const DAY: u8 = 4;

    fn parse(input: &str) -> Self {
        let roll_grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

        Day04 { roll_grid }
    }

    fn part1(&self) -> String {
        let rows = self.roll_grid.len();
        let cols = self.roll_grid.first().expect("Empty grid").len();

        let mut movable = 0;

        for x in 0..rows {
            for y in 0..cols {
                if self.roll_grid[x][y] == EMPTY {
                    continue;
                }

                let n_neigh = neighbors_at(&self.roll_grid, x, y);

                if n_neigh < MAX_NEIGH {
                    movable += 1;
                }
            }
        }

        movable.to_string()
    }

    fn part2(&self) -> String {
        let rows = self.roll_grid.len();
        let cols = self.roll_grid.first().expect("Empty grid").len();

        let mut total_movable = 0;
        let mut grid = self.roll_grid.clone();

        loop {
            let mut movable = 0;
            let mut to_clear = Vec::new();

            for x in 0..rows {
                for y in 0..cols {
                    if grid[x][y] == EMPTY {
                        continue;
                    }

                    let n_neigh = neighbors_at(&grid, x, y);

                    if n_neigh < MAX_NEIGH {
                        movable += 1;
                        to_clear.push((x, y));
                    }
                }
            }

            if movable == 0 {
                break;
            }

            for (x, y) in to_clear {
                grid[x][y] = EMPTY;
            }

            total_movable += movable;
        }

        total_movable.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    const EXAMPLE_OUTPUT_1: &str = "13";
    const EXAMPLE_OUTPUT_2: &str = "43";

    #[test]
    fn part1_example() {
        let d = Day04::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part1(), EXAMPLE_OUTPUT_1);
    }

    #[test]
    fn part2_example() {
        let d = Day04::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part2(), EXAMPLE_OUTPUT_2);
    }
}
