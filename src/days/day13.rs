use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day13input.txt");
    let sol2: u64 = solve2("./input/day13input.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    let mut res = 0;
    for map in load_maps(filename) {
        if let Some(v) = map.find_sym_vertical() {
            res += v * 100;
        } else {
            let mapt = map.transpose();
            res += mapt.find_sym_vertical().unwrap();
        }
    }
    res
}

fn solve2(filename: &str) -> u64 {
    let mut res = 0;
    for map in load_maps(filename) {
        if let Some(v) = map.find_sym_vertical_smudge() {
            res += v * 100;
        } else {
            let mapt = map.transpose();
            res += mapt.find_sym_vertical_smudge().unwrap();
        }
    }
    res
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
}

fn hd(a: &[char], b: &[char]) -> usize {
    a.iter().zip(b).filter(|p| p.0 != p.1).count()
}

impl Map {
    fn nrows(&self) -> usize {
        self.map.len()
    }

    fn ncols(&self) -> usize {
        self.map[0].len()
    }

    fn find_sym_vertical_smudge(&self) -> Option<u64> {
        for (sym_ix, w) in self.map.windows(2).enumerate() {
            let d = hd(&w[0], &w[1]);
            if d <= 1 && self.valid_vertical_sym_smudge(sym_ix, d) {
                return Some(sym_ix as u64 + 1);
            }
        }
        None
    }

    fn valid_vertical_sym_smudge(&self, sym_ix: usize, mut d: usize) -> bool {
        let start_ix = if sym_ix < self.nrows() / 2 {
            0
        } else {
            sym_ix - self.nrows() / 2
        };
        for a_ix in start_ix..sym_ix {
            // TODO: check that this is correct
            let b_ix = sym_ix + (sym_ix - a_ix) + 1;
            if b_ix < self.nrows() {
                d += hd(&self.map[a_ix], &self.map[b_ix]);
                if d > 1 {
                    return false;
                }
            }
        }
        if d == 1 {
            return true;
        }
        false
    }

    fn find_sym_vertical(&self) -> Option<u64> {
        for (sym_ix, w) in self.map.windows(2).enumerate() {
            if w[0] == w[1] && self.valid_vertical_sym(sym_ix) {
                return Some(sym_ix as u64 + 1);
            }
        }
        None
    }

    fn valid_vertical_sym(&self, sym_ix: usize) -> bool {
        let start_ix = if sym_ix < self.nrows() / 2 {
            0
        } else {
            sym_ix - self.nrows() / 2
        };
        for a_ix in start_ix..sym_ix {
            let b_ix = sym_ix + (sym_ix - a_ix) + 1;
            if b_ix < self.nrows() && self.map[a_ix] != self.map[b_ix] {
                return false;
            }
        }
        true
    }

    fn transpose(&self) -> Map {
        let mut map = vec![vec!['.'; self.nrows()]; self.ncols()];
        for (j, v) in self.map.iter().enumerate() {
            for (i, e) in v.iter().enumerate() {
                map[i][j] = *e;
            }
        }
        Map { map }
    }
}

fn load_maps(filename: &str) -> Vec<Map> {
    let mut res = Vec::new();
    let mut line_buf = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        if line.is_empty() {
            res.push(Map {
                map: line_buf.clone(),
            });
            line_buf.clear();
        } else {
            line_buf.push(line.chars().collect());
        }
    }
    res.push(Map {
        map: line_buf.clone(),
    });
    res
}
