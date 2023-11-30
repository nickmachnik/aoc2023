use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day2input1.txt");
    let sol2: u64 = solve2("./input/day2input1.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve2(filename: &str) -> u64 {
    let lines = read_lines(filename);
    let mut res = 0;
    for mut line in lines {
        line.sort();
        let mut line_done = false;
        for (v1_pos, v1) in line.iter().enumerate() {
            if line_done {
                break;
            }
            for v2 in &line[v1_pos + 1..] {
                let q = *v2 as f64 / *v1 as f64;
                if q.fract() == 0.0 {
                    res += q as u64;
                    line_done = true;
                    break;
                }
            }
        }
    }
    res
}

fn solve1(filename: &str) -> u64 {
    let lines = read_lines(filename);
    let mut res = 0;
    for line in lines {
        let minmax = find_min_max(&line);
        res += minmax.max - minmax.min;
    }
    res
}

struct MinMax {
    min: u64,
    max: u64,
}

fn find_min_max(v: &Vec<u64>) -> MinMax {
    let mut min = v[0];
    let mut max = v[0];
    for &e in v {
        if e > max {
            max = e;
        } else if e < min {
            min = e;
        }
    }
    MinMax { min, max }
}

fn read_lines(filename: &str) -> Vec<Vec<u64>> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .map(|l| l.split_whitespace().map(|e| e.parse().unwrap()).collect())
        .collect()
}
