use crate::{Solution, SolutionPair};
use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

///////////////////////////////////////////////////////////////////////////////

// DP = {}
// def f(dots, blocks, i, bi, current):
//   key = (i, bi, current)
//   if key in DP:
//     return DP[key]
//   if i==len(dots):
//     if bi==len(blocks) and current==0:
//       return 1
//     elif bi==len(blocks)-1 and blocks[bi]==current:
//       return 1
//     else:
//       return 0
//   ans = 0
//   for c in ['.', '#']:
//     if dots[i]==c or dots[i]=='?':
//       if c=='.' and current==0:
//         ans += f(dots, blocks, i+1, bi, 0)
//       elif c=='.' and current>0 and bi<len(blocks) and blocks[bi]==current:
//         ans += f(dots, blocks, i+1, bi+1, 0)
//       elif c=='#':
//         ans += f(dots, blocks, i+1, bi, current+1)
//   DP[key] = ans
//   return ans

// for part2 in [False,True]:
//   ans = 0
//   for line in L:
//     dots,blocks = line.split()
//     if part2:
//       dots = '?'.join([dots, dots, dots, dots, dots])
//       blocks = ','.join([blocks, blocks, blocks, blocks, blocks])
//     blocks = [int(x) for x in blocks.split(',')]
//     DP.clear()
//     score = f(dots, blocks, 0, 0, 0)
//     #print(dots, blocks, score, len(DP))
//     ans += score
//   print(ans)

pub fn solve() -> SolutionPair {
    // Your solution here...
    // let sol1: u64 = solve1("./input/day12example1.txt");
    let sol1: u64 = solve1("./input/day12input.txt");
    // let sol2: u64 = solve2("./input/day12input.txt");
    let sol2: u64 = solve2("./input/day12example1.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    let mut res = 0;
    for line in load_lines(filename) {
        let mut spring_row = SpringRow::from_line(&line);
        let loc_count = spring_row.count_configs();
        res += loc_count;
    }
    res
}

fn solve2(filename: &str) -> u64 {
    let mut res = 0;
    for line in load_lines(filename) {
        let mut spring_row = SpringRow::from_line(&line);
        let loc_count = spring_row.count_configs_unfolded();
        // println!("{}: {}", line, loc_count);
        res += loc_count;
    }
    res
}

struct SpringRow {
    conditions: Vec<char>,
    group_sizes: Vec<usize>,
    mem: HashMap<String, u64>,
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
            mem: HashMap::new(),
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

    fn count_configs(&mut self) -> u64 {
        _count_configs(
            self.conditions.clone(),
            self.group_sizes.clone(),
            Vec::new(),
            0,
            self.conditions.clone(),
            &mut self.mem,
        )
    }

    fn count_configs_unfolded(&mut self) -> u64 {
        _count_configs(
            self.unfolded_conditions(),
            self.unfolded_group_sizes(),
            Vec::new(),
            0,
            self.conditions.clone(),
            &mut self.mem,
        )
    }
}

fn valid_alloc(alloc: Vec<(usize, usize)>, conditions: Vec<char>) -> bool {
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
                    // println!("missing # alloc");
                    return false;
                }
            }
            '.' => {
                if alloc_pos.contains(&pos) {
                    // println!("bad . alloc");
                    return false;
                }
            }
            '?' => continue,
            _ => panic!("unexpected condition char!"),
        }
    }
    true
}

fn _arg_str(conditions: &[char], groups: &Vec<usize>) -> String {
    format!("{:?}{:?}", conditions, groups)
}

fn _count_configs(
    conditions: Vec<char>,
    groups: Vec<usize>,
    alloc: Vec<(usize, usize)>,
    position: usize,
    full_conditions: Vec<char>,
    mem: &mut HashMap<String, u64>,
) -> u64 {
    if groups.len() == 0 {
        if valid_alloc(alloc, full_conditions) {
            return 1;
        } else {
            return 0;
        }
    } else if conditions.len() == 0 {
        return 0;
    } else if conditions.len() < (groups.iter().sum::<usize>() + groups.iter().count() - 1) {
        return 0;
    }
    let mut count: u64 = 0;
    match conditions[0] {
        '.' => {
            let k = _arg_str(&conditions[1..], &groups);
            if let Some(v) = mem.get(&k) {
                count += v;
            } else {
                let v = _count_configs(
                    conditions[1..].to_vec(),
                    groups.clone(),
                    alloc,
                    position + 1,
                    full_conditions,
                    mem,
                );
                mem.insert(k, v);
                count += v;
            }
        }
        '?' => {
            // dot
            let k = _arg_str(&conditions[1..], &groups);
            if let Some(v) = mem.get(&k) {
                count += v;
            } else {
                let v = _count_configs(
                    conditions[1..].to_vec(),
                    groups.clone(),
                    alloc.clone(),
                    position + 1,
                    full_conditions.clone(),
                    mem,
                );
                mem.insert(k, v);
                count += v;
            }
            // hash
            let mut conditions_hash = conditions.clone();
            conditions_hash[0] = '#';
            let k = _arg_str(&conditions_hash, &groups);
            if let Some(v) = mem.get(&k) {
                count += v;
            } else {
                let v = _count_configs(
                    conditions_hash.clone(),
                    groups.clone(),
                    alloc,
                    position,
                    full_conditions,
                    mem,
                );
                mem.insert(k, v);
                count += v;
            }
        }
        '#' => {
            if conditions[..groups[0]].iter().all(|e| "?#".contains(*e)) {
                let mut alloc = alloc.clone();
                alloc.push((position, groups[0]));
                if groups[0] == conditions.len() {
                    if valid_alloc(alloc, full_conditions) {
                        return 1;
                    } else {
                        return 0;
                    }
                }
                if conditions[groups[0]] == '#' {
                    // now we will skip a hash, that is not allowed
                    return 0;
                }
                let k = _arg_str(&conditions[(groups[0] + 1)..], &groups[1..].to_vec());
                if let Some(v) = mem.get(&k) {
                    count += v;
                } else {
                    let v = _count_configs(
                        conditions[(groups[0] + 1)..].to_vec(),
                        groups[1..].to_vec(),
                        alloc,
                        position + groups[0] + 1,
                        full_conditions,
                        mem,
                    );
                    mem.insert(k, v);
                    count += v;
                }
            }
        }
        _ => panic!("unexpected condition char!"),
    };
    count
}

fn load_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
