use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day6input1.txt");
    let sol2: u64 = solve2("./input/day6input1.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    let mut res = 1;
    for race_record in parse_input1(filename) {
        res *= race_record.num_ways_to_win();
    }
    res
}

fn solve2(filename: &str) -> u64 {
    parse_input2(filename).num_ways_to_win()
}

#[derive(Debug)]
struct RaceRecord {
    time: u64,
    distance: u64,
}

impl RaceRecord {
    fn num_ways_to_win(&self) -> u64 {
        // we want to find the zeros of
        // -h^2 + th = d
        let t = self.time as f64;
        let d = self.distance as f64;
        let x1 = ((-t + (t * t - 4.0 * d).sqrt()) / -2.0).floor() as u64;
        let x2 = ((-t - (t * t - 4.0 * d).sqrt()) / -2.0).ceil() as u64;
        x2 - x1 - 1
    }
}

fn parse_input2(filename: &str) -> RaceRecord {
    let file = read_to_string(filename).unwrap();
    let mut lines = file.lines();
    let time: u64 = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_ascii_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: u64 = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_ascii_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();
    RaceRecord { time, distance }
}

fn parse_input1(filename: &str) -> Vec<RaceRecord> {
    let file = read_to_string(filename).unwrap();
    let mut lines = file.lines();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|e| e.parse::<u64>().unwrap())
        .collect();
    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|e| e.parse::<u64>().unwrap())
        .collect();
    std::iter::zip(times, distances)
        .map(|(time, distance)| RaceRecord { time, distance })
        .collect()
}
