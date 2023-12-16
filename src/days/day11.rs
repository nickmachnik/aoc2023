use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

type Pos = (usize, usize);

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day11input.txt");
    let sol2: u64 = solve2("./input/day11input.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    let uni = Universe::from_file(filename);
    uni.sum_distances(2)
}

fn solve2(filename: &str) -> u64 {
    let uni = Universe::from_file(filename);
    uni.sum_distances(1000000)
}
struct Universe {
    map: Vec<Vec<char>>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Universe {
    fn from_file(filename: &str) -> Self {
        let map = load_lines(filename);
        let ncol = map[0].len();
        let nrow = map.len();
        let empty_rows: Vec<usize> = map
            .iter()
            .enumerate()
            .filter(|t| t.1.iter().all(|e| *e == '.'))
            .map(|t| t.0)
            .collect();
        let empty_cols: Vec<usize> = (0..ncol)
            .filter(|j| (0..nrow).all(|i| map[i][*j] == '.'))
            .collect();
        Self {
            map,
            empty_rows,
            empty_cols,
        }
    }

    fn galaxy_positions(&self) -> Vec<Pos> {
        let mut res = Vec::new();
        for (i, row) in self.map.iter().enumerate() {
            for (j, e) in (row).iter().enumerate() {
                if *e == '#' {
                    res.push((i, j))
                }
            }
        }
        res
    }

    fn expansion_between_rows(&self, a: usize, b: usize) -> usize {
        let mut a = a;
        let mut b = b;
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        self.empty_rows
            .iter()
            .filter(|i| (a < **i) & (**i < b))
            .copied()
            .count()
    }

    fn expansion_between_cols(&self, a: usize, b: usize) -> usize {
        let mut a = a;
        let mut b = b;
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        self.empty_cols
            .iter()
            .filter(|i| (a < **i) & (**i < b))
            .copied()
            .count()
    }

    fn sum_distances(&self, exp_const: u64) -> u64 {
        let pos = self.galaxy_positions();
        let mut sum = 0;
        for (i, a) in pos.iter().enumerate() {
            for b in &pos[i + 1..] {
                let d = ((a.0 as i64 - b.0 as i64).abs()
                    + (self.expansion_between_rows(a.0, b.0) as u64 * (exp_const - 1)) as i64
                    + (b.1 as i64 - a.1 as i64).abs()
                    + (self.expansion_between_cols(a.1, b.1) as u64 * (exp_const - 1)) as i64)
                    as u64;
                sum += d;
            }
        }
        sum
    }
}

fn load_lines(filename: &str) -> Vec<Vec<char>> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}
