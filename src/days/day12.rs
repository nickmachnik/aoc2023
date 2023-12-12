use crate::{Solution, SolutionPair};
use core::panic;
use std::{collections::HashSet, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    // let sol1: u64 = solve1("./input/day12example1.txt");
    let sol1: u64 = solve1("./input/day12input.txt");
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    let mut res = 0;
    for line in load_lines(filename) {
        let loc_count = SpringRow::from_line(&line).count_configs();
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
            .skip(1)
            .next()
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
        for _ in 0..5 {
            res.push('?');
            res.extend(self.conditions.iter());
        }
        res
    }

    fn unfolded_group_sizes(&self) -> Vec<usize> {
        let mut res = self.group_sizes.clone();
        for _ in 0..5 {
            res.extend(self.group_sizes.iter());
        }
        res
    }

    fn count_configs(&self) -> u64 {
        self._count_configs(
            self.conditions.clone(),
            self.group_sizes.clone(),
            Vec::new(),
            0,
            self.conditions.clone(),
        )
    }

    fn valid_alloc(&self, alloc: Vec<(usize, usize)>, conditions: Vec<char>) -> bool {
        let mut alloc_pos = HashSet::new();
        for t in alloc {
            for p in t.0..t.0 + t.1 {
                alloc_pos.insert(p);
            }
        }
        for (pos, c) in conditions.iter().enumerate() {
            match c {
                '#' => {
                    if !alloc_pos.contains(&pos) {
                        return false;
                    }
                }
                '?' | '.' => continue,
                _ => panic!("unexpected condition char!"),
            }
        }
        true
    }

    fn _count_configs(
        &self,
        conditions: Vec<char>,
        groups: Vec<usize>,
        alloc: Vec<(usize, usize)>,
        position: usize,
        full_conditions: Vec<char>,
    ) -> u64 {
        if groups.len() == 0 {
            if self.valid_alloc(alloc, full_conditions) {
                return 1;
            } else {
                return 0;
            }
        } else if conditions.len() == 0 {
            return 0;
        } else if conditions.len() < (groups.iter().sum::<usize>() + groups.iter().count() - 1) {
            return 0;
        }
        let mut count = 0;
        match conditions[0] {
            '.' => {
                count += self._count_configs(
                    conditions[1..].to_vec(),
                    groups,
                    alloc,
                    position + 1,
                    full_conditions,
                )
            }
            '?' => {
                // dot
                count += self._count_configs(
                    conditions[1..].to_vec(),
                    groups.clone(),
                    alloc.clone(),
                    position + 1,
                    full_conditions.clone(),
                );
                // hash
                let mut conditions_hash = conditions.clone();
                conditions_hash[0] = '#';
                count +=
                    self._count_configs(conditions_hash, groups, alloc, position, full_conditions);
            }
            '#' => {
                if conditions[..groups[0]].iter().all(|e| "?#".contains(*e)) {
                    let mut alloc = alloc.clone();
                    alloc.push((position, groups[0]));
                    if groups[0] == conditions.len() {
                        if self.valid_alloc(alloc, full_conditions) {
                            return 1;
                        } else {
                            return 0;
                        }
                    }
                    if conditions[groups[0]] == '#' {
                        // now we will skip a hash, that is not allowed
                        return 0;
                    }
                    count += self._count_configs(
                        conditions[(groups[0] + 1)..].to_vec(),
                        groups[1..].to_vec(),
                        alloc,
                        position + groups[0] + 1,
                        full_conditions,
                    )
                }
            }
            _ => panic!("unexpected condition char!"),
        };
        count
    }
}

fn load_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
