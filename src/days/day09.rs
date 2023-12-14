use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: i64 = solve1("./input/day9input1.txt");
    let sol2: i64 = solve2("./input/day9input1.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> i64 {
    load_histories(filename)
        .iter()
        .map(|h| h.next_value())
        .sum()
}

fn solve2(filename: &str) -> i64 {
    // for h in load_histories(filename) {
    //     println!("h: {:?}; pv: {:?}", h, h.prev_value());
    // }
    // 0
    load_histories(filename)
        .iter()
        .map(|h| h.prev_value())
        .sum()
}

#[derive(Debug)]
struct History {
    vals: Vec<i64>,
}

impl History {
    fn from_line(line: &str) -> Self {
        Self {
            vals: line
                .split_ascii_whitespace()
                .map(|e| e.parse().unwrap())
                .collect(),
        }
    }

    fn next_value(&self) -> i64 {
        let mut v = self.vals.clone();
        let mut final_vals: Vec<i64> = vec![*v.last().unwrap()];
        while !uniform(&v) {
            v = diff(v);
            final_vals.push(*v.last().unwrap());
        }
        final_vals.iter().sum()
    }

    fn prev_value(&self) -> i64 {
        let mut v = self.vals.clone();
        let mut first_vals: Vec<i64> = vec![*v.first().unwrap()];
        while !uniform(&v) {
            v = diff(v);
            first_vals.push(*v.first().unwrap());
        }
        let mut res = 0;
        first_vals.reverse();
        for fv in first_vals {
            res = fv - res;
        }
        res
    }
}

fn uniform(v: &[i64]) -> bool {
    v.iter().all(|e| *e == v[0])
}

fn diff(v: Vec<i64>) -> Vec<i64> {
    v.windows(2).map(|w| w[1] - w[0]).collect()
}

fn load_histories(filename: &str) -> Vec<History> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(History::from_line)
        .collect()
}
