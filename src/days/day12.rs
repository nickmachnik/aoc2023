use crate::{Solution, SolutionPair};
use std::{collections::HashMap, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day12input.txt");
    let sol2: u64 = solve2("./input/day12input.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    let mut res = 0;
    for line in load_lines(filename) {
        let spring_row = SpringRow::from_line(&line);
        let loc_count = spring_row.count_configs();
        res += loc_count;
    }
    res
}

fn solve2(filename: &str) -> u64 {
    let mut res = 0;
    for line in load_lines(filename) {
        let spring_row = SpringRow::from_line(&line);
        let loc_count = spring_row.count_configs_unfolded();
        // println!("{}: {}", line, loc_count);
        res += loc_count;
    }
    res
}

struct SpringRow {
    conditions: Vec<char>,
    group_sizes: Vec<usize>,
}

impl SpringRow {
    fn from_line(l: &str) -> Self {
        let conditions = l.split_ascii_whitespace().next().unwrap().chars().collect();
        let group_sizes = l
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|e| e.parse().unwrap())
            .collect();
        Self {
            conditions,
            group_sizes,
        }
    }

    fn unfolded_conditions(&self) -> Vec<char> {
        let mut res = self.conditions.clone();
        for _ in 0..4 {
            res.push('?');
            res.extend(self.conditions.iter());
        }
        res
    }

    fn unfolded_group_sizes(&self) -> Vec<usize> {
        let mut res = self.group_sizes.clone();
        for _ in 0..4 {
            res.extend(self.group_sizes.iter());
        }
        res
    }

    fn count_configs(&self) -> u64 {
        count_configs(
            &self.conditions,
            &self.group_sizes,
            0,
            0,
            0,
            &mut HashMap::new(),
        )
    }

    fn count_configs_unfolded(&self) -> u64 {
        count_configs(
            &self.unfolded_conditions(),
            &self.unfolded_group_sizes(),
            0,
            0,
            0,
            &mut HashMap::new(),
        )
    }
}

fn load_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn count_configs(
    dots: &Vec<char>,
    blocks: &Vec<usize>,
    pos: usize,
    block_ix: usize,
    current_len: usize,
    mem: &mut HashMap<(usize, usize, usize), u64>,
) -> u64 {
    let k = (pos, block_ix, current_len);
    if let Some(v) = mem.get(&k) {
        return *v;
    }
    if pos == dots.len() {
        if (block_ix == blocks.len()) & (current_len == 0) {
            return 1;
        } else if block_ix == blocks.len() - 1 {
            if current_len == blocks[block_ix] {
                return 1;
            } else {
                return 0;
            }
        } else {
            return 0;
        }
    }

    let mut count = 0;
    for c in ['.', '#'] {
        if (dots[pos] == c) | (dots[pos] == '?') {
            if (c == '.') & (current_len == 0) {
                count += count_configs(dots, blocks, pos + 1, block_ix, current_len, mem);
            } else if (c == '.') & (current_len > 0) & (block_ix < blocks.len()) {
                if blocks[block_ix] == current_len {
                    count += count_configs(dots, blocks, pos + 1, block_ix + 1, 0, mem);
                }
            } else if c == '#' {
                count += count_configs(dots, blocks, pos + 1, block_ix, current_len + 1, mem);
            }
        }
    }
    mem.insert(k, count);
    count
}
