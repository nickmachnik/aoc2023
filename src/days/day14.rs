use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////
const NUM_CYCLES: usize = 1000000000;
type CompMap = Vec<u128>;

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day14input.txt");
    let sol2: u64 = solve2("./input/day14input.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve2(filename: &str) -> u64 {
    let mut dish = Dish::from_file(filename);
    let mut mem: HashSet<CompMap> = HashSet::new();
    let mut loop_loads = Vec::new();
    let mut num_cycled = 0;
    let mut in_loop = false;
    let mut loop_start = 0;
    loop {
        let init_state = dish.compress();
        dish.cycle();
        num_cycled += 1;

        if !in_loop {
            if mem.contains(&init_state) {
                in_loop = true;
                loop_start = num_cycled;
                mem.clear();
            }
            mem.insert(init_state);
        } else {
            if mem.contains(&init_state) {
                let sol_ix = (NUM_CYCLES - loop_start - 1) % (num_cycled - loop_start);
                return loop_loads[sol_ix];
            }
            loop_loads.push(dish.load());
        }
    }
}
struct Dish {
    map: Vec<Vec<char>>,
}

impl Dish {
    fn load(&self) -> u64 {
        let mut load = 0;
        for (line_ix, line) in self.map.iter().enumerate() {
            for c in line {
                if *c == 'O' {
                    load += self.nrows() - line_ix;
                }
            }
        }
        load.try_into().unwrap()
    }

    fn cycle(&mut self) {
        for _ in 0..4 {
            self.roll_north();
            self.rotate_clockwise();
        }
    }

    fn compress(&mut self) -> Vec<u128> {
        let mut res = vec![0u128; self.nrows()];
        for (rix, r) in self.map.iter().enumerate() {
            for c in r {
                res[rix] <<= 1;
                if *c == 'O' {
                    res[rix] += 1;
                }
            }
        }
        res
    }

    fn from_file(filename: &str) -> Self {
        Self {
            map: read_to_string(filename)
                .unwrap()
                .lines()
                .map(|l| l.chars().collect())
                .collect(),
        }
    }

    fn ncols(&self) -> usize {
        self.map[0].len()
    }

    fn nrows(&self) -> usize {
        self.map.len()
    }

    fn rotate_clockwise(&mut self) {
        let mut new_map = vec![vec!['.'; self.nrows()]; self.ncols()];
        for (i, r) in self.map.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                new_map[j][self.nrows() - i - 1] = *c;
            }
        }
        self.map = new_map;
    }

    fn roll_north(&mut self) {
        let mut nxt_insert = vec![0; self.ncols()];
        let mut new_map = self.map.clone();
        for (line_ix, line) in self.map.iter().enumerate() {
            for (cix, c) in line.iter().enumerate() {
                match c {
                    '#' => nxt_insert[cix] = line_ix + 1,
                    'O' => {
                        new_map[line_ix][cix] = '.';
                        new_map[nxt_insert[cix]][cix] = 'O';
                        nxt_insert[cix] += 1;
                    }
                    _ => continue,
                }
            }
        }
        self.map = new_map;
    }
}

fn solve1(filename: &str) -> u64 {
    let file = read_to_string(filename).unwrap();
    let mut line_ix = 0;
    let mut lines = file.lines();
    let mut pos_sums = Vec::new();
    let mut nxt_insert = Vec::new();
    let mut num_round = Vec::new();
    for c in lines.next().unwrap().chars() {
        match c {
            '#' => {
                pos_sums.push(0);
                num_round.push(0);
                nxt_insert.push(1);
            }
            'O' => {
                pos_sums.push(0);
                num_round.push(1);
                nxt_insert.push(1);
            }
            '.' => {
                pos_sums.push(0);
                num_round.push(0);
                nxt_insert.push(0);
            }
            _ => continue,
        }
    }
    for line in lines {
        line_ix += 1;
        for (cix, c) in line.chars().enumerate() {
            match c {
                '#' => nxt_insert[cix] = line_ix + 1,
                'O' => {
                    pos_sums[cix] += nxt_insert[cix];
                    num_round[cix] += 1;
                    nxt_insert[cix] += 1;
                }
                _ => continue,
            }
        }
    }
    num_round
        .iter()
        .zip(pos_sums)
        .map(|z| z.0 * (line_ix + 1) - z.1)
        .sum()
}
