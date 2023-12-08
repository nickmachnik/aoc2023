use crate::{Solution, SolutionPair};
use std::{collections::HashMap, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day8input1.txt");
    let sol2: u64 = solve2("./input/day8input1.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn solve2(filename: &str) -> u64 {
    let map = parse_input(filename);
    let mut res = 1;
    for starting_node in map.starting_nodes() {
        let mut steps: u64 = 0;
        let mut curr_node = starting_node;
        for dir in map.instr.iter().cycle() {
            steps += 1;
            let pair = map.net.get(curr_node).unwrap();
            curr_node = match dir {
                'L' => &pair.0,
                'R' => &pair.1,
                _ => panic!("unexpected dir symbol!"),
            };
            if curr_node.ends_with('Z') {
                break;
            }
        }
        res = lcm(res, steps);
    }
    res
}

fn solve1(filename: &str) -> u64 {
    let map = parse_input(filename);
    let mut steps: u64 = 0;
    let mut curr_node = "AAA";
    for dir in map.instr.iter().cycle() {
        steps += 1;
        let pair = map.net.get(curr_node).unwrap();
        curr_node = match dir {
            'L' => &pair.0,
            'R' => &pair.1,
            _ => panic!("unexpected dir symbol!"),
        };
        if curr_node == "ZZZ" {
            break;
        }
    }
    steps
}

#[derive(Default)]
struct Map {
    instr: Vec<char>,
    net: HashMap<String, (String, String)>,
}

impl Map {
    fn starting_nodes(&self) -> Vec<&String> {
        self.net.keys().filter(|k| k.ends_with('A')).collect()
    }
}

fn parse_input(filename: &str) -> Map {
    let mut map = Map::default();
    let file = read_to_string(filename).unwrap();
    let mut lines = file.lines();
    map.instr = lines.next().unwrap().chars().collect();
    lines.next(); // skip empty line
    for l in lines {
        let chars: Vec<char> = l.chars().collect();
        map.net.insert(
            chars[0..=2].iter().collect(),
            (
                chars[7..=9].iter().collect(),
                chars[12..=14].iter().collect(),
            ),
        );
    }
    map
}
