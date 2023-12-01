use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

const DIGITS: [&str; 20] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
];

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day1input1.txt");
    let sol2: u64 = solve2("./input/day1example1.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve2(filename: &str) -> u64 {
    // let lines = read_lines(filename);
    let mut res = 0;
    res
}

fn solve1(filename: &str) -> u64 {
    let lines = filter_lines(filename);
    let mut res = 0;
    for line in lines {
        let calibration_code: u64 = vec![line.first().unwrap(), line.last().unwrap()]
            .into_iter()
            .collect::<String>()
            .parse()
            .unwrap();
        res += calibration_code;
    }
    res
}

fn filter_lines(filename: &str) -> Vec<Vec<char>> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| l.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>())
        .collect()
}
