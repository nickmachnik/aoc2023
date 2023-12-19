use crate::{Solution, SolutionPair};
use core::panic;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day18input.txt");
    let sol2: u64 = solve2("./input/day18input.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    Polygon::from_lines_p1(load_lines(filename)).area()
}

fn solve2(filename: &str) -> u64 {
    Polygon::from_lines_p2(load_lines(filename)).area()
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}
struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn from_lines_p1(lines: Vec<String>) -> Self {
        let mut points = vec![Point { x: 0, y: 0 }];
        for l in lines {
            let fields: Vec<&str> = l.split_ascii_whitespace().collect();
            let dir = fields[0].chars().next().unwrap();
            let len: i64 = fields[1].parse().unwrap();
            let last_point = points.last().unwrap();
            points.push(match dir {
                'U' => Point {
                    x: last_point.x,
                    y: last_point.y - len,
                },
                'D' => Point {
                    x: last_point.x,
                    y: last_point.y + len,
                },
                'L' => Point {
                    x: last_point.x - len,
                    y: last_point.y,
                },
                'R' => Point {
                    x: last_point.x + len,
                    y: last_point.y,
                },
                _ => panic!("Unexpected direction"),
            });
        }
        Self { points }
    }

    fn from_lines_p2(lines: Vec<String>) -> Self {
        let mut points = vec![Point { x: 0, y: 0 }];
        for l in lines {
            let hex: &str = l.split_ascii_whitespace().nth(2).unwrap();
            let len = i64::from_str_radix(&hex[2..=6], 16).unwrap();
            let dir = match hex.chars().nth(7).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!("Unexpected dir char!"),
            };
            let last_point = points.last().unwrap();
            points.push(match dir {
                'U' => Point {
                    x: last_point.x,
                    y: last_point.y - len,
                },
                'D' => Point {
                    x: last_point.x,
                    y: last_point.y + len,
                },
                'L' => Point {
                    x: last_point.x - len,
                    y: last_point.y,
                },
                'R' => Point {
                    x: last_point.x + len,
                    y: last_point.y,
                },
                _ => panic!("Unexpected direction"),
            });
        }
        Self { points }
    }

    fn area(&self) -> u64 {
        let mut filled = 0;
        let mut boundary = 0;
        for w in self.points.windows(2) {
            boundary += (w[0].x - w[1].x).abs() + (w[0].y - w[1].y).abs();
            filled += w[0].x * w[1].y - w[0].y * w[1].x;
        }
        filled /= 2;
        filled.unsigned_abs() + boundary.unsigned_abs() / 2 + 1
    }
}

fn load_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
