use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day15input.txt");
    let sol2: u64 = solve2("./input/day15input.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    load_codes(filename).iter().map(|s| hash(s)).sum::<u32>() as u64
}

fn solve2(filename: &str) -> u64 {
    let mut boxes = Boxes::new();
    for step in load_steps(filename) {
        boxes.do_step(step);
    }
    boxes.total_focussing_power()
}

fn hash(s: &str) -> u32 {
    let mut res = 0;
    for c in s.chars() {
        res += c as u32;
        res *= 17;
        res %= 256;
    }
    res
}

struct Boxes {
    boxes: Vec<Vec<Step>>,
}

impl Boxes {
    fn new() -> Self {
        Self {
            boxes: vec![vec![]; 256],
        }
    }

    fn do_step(&mut self, step: Step) {
        match step.op {
            '=' => {
                let box_ix = step.hash();
                let mut in_box = false;
                for s in &mut self.boxes[box_ix as usize] {
                    if s.label == step.label {
                        *s = step.clone();
                        in_box = true;
                        break;
                    }
                }
                if !in_box {
                    self.boxes[box_ix as usize].push(step);
                }
            }
            '-' => {
                let box_ix = step.hash();
                let mut entry_ix = None;
                for (ix, s) in self.boxes[box_ix as usize].iter().enumerate() {
                    if s.label == step.label {
                        entry_ix = Some(ix);
                        break;
                    }
                }
                if let Some(ix) = entry_ix {
                    self.boxes[box_ix as usize].remove(ix);
                }
            }
            _ => panic!("Invalid op!"),
        }
    }

    fn total_focussing_power(&self) -> u64 {
        let mut res = 0;
        for (bix, b) in self.boxes.iter().enumerate() {
            for (six, s) in b.iter().enumerate() {
                res += (bix as u64 + 1) * (six as u64 + 1) * s.len as u64;
            }
        }
        res
    }
}

#[derive(Debug, Clone)]
struct Step {
    label: String,
    op: char,
    len: u32,
}

impl Step {
    fn from_str(s: &str) -> Self {
        if s.ends_with('-') {
            Self {
                label: s.strip_suffix('-').unwrap().to_owned(),
                op: '-',
                len: 0,
            }
        } else {
            let len = s.chars().last().unwrap().to_digit(10).unwrap();
            Self {
                label: s[..s.len() - 2].to_owned(),
                op: '=',
                len,
            }
        }
    }

    fn hash(&self) -> u32 {
        hash(&self.label)
    }
}

fn load_steps(filename: &str) -> Vec<Step> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(Step::from_str)
        .collect()
}

fn load_codes(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(String::from)
        .collect()
}
