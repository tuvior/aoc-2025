use std::collections::HashSet;

use crate::Day;

type Box = (i64, i64, i64);

fn sq_dist(v1: Box, v2: Box) -> i64 {
    (v1.0 - v2.0).pow(2) + (v1.1 - v2.1).pow(2) + (v1.2 - v2.2).pow(2)
}

struct BoxPair {
    dist: i64,
    box1: Box,
    box2: Box,
}

pub struct Day08 {
    boxes: Vec<(i64, i64, i64)>,
}

impl Day for Day08 {
    const DAY: u8 = 8;

    fn parse(input: &str) -> Self {
        let boxes = input
            .lines()
            .map(|s| s.split(',').collect())
            .map(|v: Vec<&str>| {
                (
                    v[0].parse().unwrap(),
                    v[1].parse().unwrap(),
                    v[2].parse().unwrap(),
                )
            })
            .collect();

        Day08 { boxes }
    }

    fn part1(&self) -> String {
        let mut pairs = vec![];
        let mut circuits: Vec<HashSet<(i64, i64, i64)>> = vec![];

        for (i, &box1) in self.boxes.iter().enumerate() {
            for &box2 in self.boxes.iter().skip(i + 1) {
                pairs.push(BoxPair {
                    dist: sq_dist(box1, box2),
                    box1,
                    box2,
                });
            }
        }

        pairs.sort_by_key(|p| p.dist);

        for pair in pairs.iter().take(1000) {
            let i1 = circuits.iter().position(|c| c.contains(&pair.box2));
            let i2 = circuits.iter().position(|c| c.contains(&pair.box1));

            match (i1, i2) {
                (Some(c1), Some(c2)) => {
                    if c1 != c2 {
                        let (c1, c2) = if c1 < c2 { (c1, c2) } else { (c2, c1) };

                        let second = circuits.swap_remove(c2);
                        circuits[c1].extend(second);
                    }
                }
                (None, Some(c2)) => {
                    circuits[c2].insert(pair.box2);
                }
                (Some(c1), None) => {
                    circuits[c1].insert(pair.box1);
                }
                (None, None) => {
                    circuits.push(HashSet::from([pair.box1, pair.box2]));
                }
            }
        }

        circuits.sort_by_key(|c| c.len());
        circuits.reverse();

        circuits
            .iter()
            .take(3)
            .map(|c| c.len() as i64)
            .product::<i64>()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut pairs = vec![];
        let mut circuits: Vec<HashSet<(i64, i64, i64)>> = vec![];

        for (i, &box1) in self.boxes.iter().enumerate() {
            for &box2 in self.boxes.iter().skip(i + 1) {
                pairs.push(BoxPair {
                    dist: sq_dist(box1, box2),
                    box1,
                    box2,
                });
            }
        }

        pairs.sort_by_key(|p| p.dist);

        let mut last_pair = None;

        for pair in pairs {
            let i1 = circuits.iter().position(|c| c.contains(&pair.box2));
            let i2 = circuits.iter().position(|c| c.contains(&pair.box1));

            match (i1, i2) {
                (Some(c1), Some(c2)) => {
                    if c1 != c2 {
                        let (c1, c2) = if c1 < c2 { (c1, c2) } else { (c2, c1) };

                        let second = circuits.swap_remove(c2);
                        circuits[c1].extend(second);
                    }
                }
                (None, Some(c2)) => {
                    circuits[c2].insert(pair.box2);
                }
                (Some(c1), None) => {
                    circuits[c1].insert(pair.box1);
                }
                (None, None) => {
                    circuits.push(HashSet::from([pair.box1, pair.box2]));
                }
            }

            // This should mean we have a single cluster
            if circuits.first().unwrap().len() == self.boxes.len() {
                last_pair = Some(pair);
                break;
            }
        }

        let pair = last_pair.unwrap();

        (pair.box1.0 * pair.box2.0).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    const EXAMPLE_OUTPUT_1: &str = "20";
    const EXAMPLE_OUTPUT_2: &str = "25272";

    #[test]
    fn part1_example() {
        let d = Day08::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part1(), EXAMPLE_OUTPUT_1);
    }

    #[test]
    fn part2_example() {
        let d = Day08::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part2(), EXAMPLE_OUTPUT_2);
    }
}
