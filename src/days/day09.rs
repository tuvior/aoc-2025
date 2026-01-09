use crate::Day;

type Point = (i64, i64);

fn rect_sum(ps: &[Vec<i64>], iy1: usize, ix1: usize, iy2: usize, ix2: usize) -> i64 {
    ps[iy2][ix2] - ps[iy1][ix2] - ps[iy2][ix1] + ps[iy1][ix1]
}

pub struct Day09 {
    points: Vec<Point>,
}

impl Day for Day09 {
    const DAY: u8 = 9;

    fn parse(input: &str) -> Self {
        let points = input
            .lines()
            .map(|s| s.split(',').collect())
            .map(|v: Vec<&str>| (v[0].parse().unwrap(), v[1].parse().unwrap()))
            .collect();

        Day09 { points }
    }

    fn part1(&self) -> String {
        let mut biggest_rect = 0;
        for (i, &point1) in self.points.iter().enumerate() {
            for &point2 in self.points.iter().skip(i + 1) {
                let area = ((point1.0 - point2.0).abs() + 1) * ((point1.1 - point2.1).abs() + 1);

                if area > biggest_rect {
                    biggest_rect = area;
                }
            }
        }

        biggest_rect.to_string()
    }

    fn part2(&self) -> String {
        type Point = (i64, i64);

        let points: &Vec<Point> = &self.points;
        let n = points.len();

        // coordinate compression
        let mut xs: Vec<i64> = Vec::with_capacity(2 * n);
        let mut ys: Vec<i64> = Vec::with_capacity(2 * n);

        for &(x, y) in points {
            xs.push(x);
            xs.push(x + 1);
            ys.push(y);
            ys.push(y + 1);
        }

        xs.sort();
        xs.dedup();
        ys.sort();
        ys.dedup();

        let nx = xs.len() - 1;
        let ny = ys.len() - 1;

        let mut edges: Vec<(Point, Point)> = Vec::with_capacity(n);
        for i in 0..n {
            edges.push((points[i], points[(i + 1) % n]));
        }

        // raster
        let mut inside = vec![vec![false; nx]; ny];

        for j in 0..ny {
            let y0 = ys[j];

            let mut inters: Vec<i64> = Vec::new();

            for &(a, b) in &edges {
                let (x1, y1) = a;
                let (x2, y2) = b;

                if x1 != x2 {
                    continue;
                }

                let ymin = y1.min(y2);
                let ymax = y1.max(y2);

                if ymin <= y0 && y0 < ymax {
                    inters.push(x1);
                }
            }

            inters.sort();

            for pair in inters.chunks_exact(2) {
                let x_start = pair[0];
                let x_end = pair[1];

                let ix_start = xs.binary_search(&x_start).unwrap();
                let ix_end = xs.binary_search(&x_end).unwrap();

                for ix in ix_start..ix_end {
                    inside[j][ix] = true;
                }
            }
        }

        // weighted grid
        let mut w = vec![vec![0_i64; nx]; ny];

        for j in 0..ny {
            let height = ys[j + 1] - ys[j];
            for i in 0..nx {
                if inside[j][i] {
                    let width = xs[i + 1] - xs[i];
                    w[j][i] = width * height;
                }
            }
        }

        // prefix sum
        let mut ps = vec![vec![0_i64; nx + 1]; ny + 1];
        for j in 1..=ny {
            for i in 1..=nx {
                ps[j][i] = w[j - 1][i - 1] + ps[j - 1][i] + ps[j][i - 1] - ps[j - 1][i - 1];
            }
        }

        let reds = &self.points;
        let mut best_rectangle: i64 = 0;

        for a in 0..reds.len() {
            for b in a + 1..reds.len() {
                let (x1, y1) = reds[a];
                let (x2, y2) = reds[b];

                if x1 == x2 || y1 == y2 {
                    continue;
                }

                let lx = x1.min(x2);
                let rx = x1.max(x2);
                let ly = y1.min(y2);
                let ry = y1.max(y2);

                let area = (rx - lx + 1) * (ry - ly + 1);

                let ix1 = xs.binary_search(&lx).unwrap();
                let ix2 = xs.binary_search(&(rx + 1)).unwrap(); // open bound
                let iy1 = ys.binary_search(&ly).unwrap();
                let iy2 = ys.binary_search(&(ry + 1)).unwrap(); // open bound

                let sum_inside = rect_sum(&ps, iy1, ix1, iy2, ix2);

                if sum_inside == area && area > best_rectangle {
                    best_rectangle = area;
                }
            }
        }

        best_rectangle.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    const EXAMPLE_OUTPUT_1: &str = "50";
    const EXAMPLE_OUTPUT_2: &str = "24";

    #[test]
    fn part1_example() {
        let d = Day09::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part1(), EXAMPLE_OUTPUT_1);
    }

    #[test]
    fn part2_example() {
        let d = Day09::parse(EXAMPLE_INPUT.trim());
        assert_eq!(d.part2(), EXAMPLE_OUTPUT_2);
    }
}
