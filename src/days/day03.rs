use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

type Symbols = HashMap<(i64, i64), char>;
const GEAR_LEN: usize = 2;

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day3input1.txt");
    let sol2: u64 = solve2("./input/day3input1.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    let (numbers, symbols) = parse_positions(filename);
    numbers
        .iter()
        .filter(|n| n.is_part_number(&symbols))
        .map(|n| n.number)
        .sum()
}

fn solve2(filename: &str) -> u64 {
    let (numbers, symbols) = parse_positions(filename);
    let mut potential_gears: HashMap<(i64, i64), Vec<u64>> = HashMap::new();
    for number in numbers {
        for gear_pos in number.adjacent_gears(&symbols) {
            potential_gears
                .entry(gear_pos)
                .or_default()
                .push(number.number);
        }
    }
    let mut res = 0;
    for (_k, v) in potential_gears.iter() {
        if v.len() == GEAR_LEN {
            res += v[0] * v[1];
        }
    }
    res
}

#[derive(Debug)]
struct Number {
    number: u64,
    line: i64,
    start: i64,
    end: i64,
}

impl Number {
    fn adjacent_coords(&self) -> Vec<(i64, i64)> {
        let mut res = Vec::new();
        for y in self.start - 1..=self.end + 1 {
            res.push((self.line + 1, y));
            res.push((self.line - 1, y));
        }
        res.push((self.line, self.start - 1));
        res.push((self.line, self.end + 1));
        res
    }

    fn adjacent_gears(&self, symbols: &Symbols) -> Vec<(i64, i64)> {
        self.adjacent_coords()
            .iter()
            .filter_map(|c| symbols.get_key_value(c))
            .filter(|(_k, v)| **v == '*')
            .map(|(k, _v)| *k)
            .collect()
    }

    fn is_part_number(&self, symbols: &Symbols) -> bool {
        self.adjacent_coords()
            .iter()
            .any(|c| symbols.contains_key(c))
    }
}

fn parse_positions(filename: &str) -> (Vec<Number>, Symbols) {
    let mut numbers = Vec::new();
    let mut symbols = HashMap::new();
    for (line_number, line) in load_lines(filename).iter().enumerate() {
        let mut num_buf = "".to_owned();
        let mut start = 0;
        for (c_pos, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if num_buf.is_empty() {
                    start = c_pos;
                }
                num_buf.push(c);
            } else {
                if !num_buf.is_empty() {
                    numbers.push(Number {
                        number: num_buf.parse().unwrap(),
                        line: line_number as i64,
                        start: start as i64,
                        end: c_pos as i64 - 1,
                    });
                    num_buf.clear();
                }
                if c != '.' {
                    symbols.insert((line_number as i64, c_pos as i64), c);
                }
            }
        }
        if !num_buf.is_empty() {
            numbers.push(Number {
                number: num_buf.parse().unwrap(),
                line: line_number as i64,
                start: start as i64,
                end: line.len() as i64 - 1,
            });
            num_buf.clear();
        }
    }
    (numbers, symbols)
}

fn load_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
