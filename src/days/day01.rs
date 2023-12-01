use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

const DIGITS: [&str; 20] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
];

const DIGITS_CHAR: [char; 20] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8',
    '9',
];

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day1input1.txt");
    let sol2: u64 = solve2("./input/day1input1.txt");

    (Solution::from(sol1), Solution::from(sol2))
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

fn solve2(filename: &str) -> u64 {
    let mut res = 0;
    for line in load_lines(filename) {
        let mut line_digits = vec![];
        let last_ix = line.len() - 1;
        let mut window_start = 0;
        while window_start <= last_ix {
            let mut found_digit = false;
            for (digit_ix, digit) in DIGITS.iter().enumerate() {
                let window_end = window_start + digit.len();
                if window_end > last_ix + 1 {
                    continue;
                }
                if **digit == line[window_start..window_end] {
                    found_digit = true;
                    line_digits.push(DIGITS_CHAR[digit_ix]);
                    window_start += 1;
                    break;
                }
            }
            if !found_digit {
                window_start += 1;
            }
        }
        println!("{:?}", line_digits);
        let calibration_code: u64 = vec![line_digits.first().unwrap(), line_digits.last().unwrap()]
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
        .map(|l| {
            l.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>()
        })
        .collect()
}

fn load_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
