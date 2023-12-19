use crate::{Solution, SolutionPair};
use core::panic;
use std::collections::HashSet;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day18input.txt");
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    let instr = load_instr(filename);
    let mut ter = Terrain::from_instr(&instr);
    ter.pit_vol(&instr)
}

struct Terrain {
    map: Vec<Vec<char>>,
    init_row: usize,
    init_col: usize,
}

impl Terrain {
    fn from_instr(instr: &Vec<Instr>) -> Self {
        let mut row = vec![0];
        let mut col = vec![0];
        for i in instr {
            match i.dir {
                'U' => row.push(row.last().unwrap() - i.len as i32),
                'D' => row.push(row.last().unwrap() + i.len as i32),
                'L' => col.push(col.last().unwrap() - i.len as i32),
                'R' => col.push(col.last().unwrap() + i.len as i32),
                _ => panic!("Invalid direction!"),
            }
        }
        let nrows = (row.iter().max().unwrap() - row.iter().min().unwrap()) as usize;
        let ncols = (col.iter().max().unwrap() - col.iter().min().unwrap()) as usize;
        Self {
            map: vec![vec!['.'; ncols + 1]; nrows + 1],
            init_row: row.iter().min().unwrap().unsigned_abs() as usize,
            init_col: col.iter().min().unwrap().unsigned_abs() as usize,
        }
    }

    fn print(&self) {
        for row in &self.map {
            let s: String = row.iter().collect();
            println!("{}", s);
        }
    }

    fn ncol(&self) -> usize {
        self.map[0].len()
    }

    fn nrow(&self) -> usize {
        self.map.len()
    }

    fn pit_vol(&mut self, instr: &Vec<Instr>) -> u64 {
        // dig outline
        let mut row = self.init_row;
        let mut col = self.init_col;
        self.map[row][col] = '#';
        let mut pit_vol = 1;
        for i in instr {
            pit_vol += i.len;
            match i.dir {
                'U' => {
                    for _ in 0..i.len {
                        row -= 1;
                        self.map[row][col] = '#';
                    }
                }
                'D' => {
                    for _ in 0..i.len {
                        row += 1;
                        self.map[row][col] = '#';
                    }
                }
                'L' => {
                    for _ in 0..i.len {
                        col -= 1;
                        self.map[row][col] = '#';
                    }
                }
                'R' => {
                    for _ in 0..i.len {
                        col += 1;
                        self.map[row][col] = '#';
                    }
                }
                _ => panic!("Invalid direction!"),
            }
        }
        // we touch every map outline;
        // just use the top row to find a spot inside the pit
        let mut col_inside = 0;
        for (ix, e) in self.map[0].iter().enumerate() {
            if *e == '#' && self.map[1][ix] == '.' {
                col_inside = ix;
                break;
            }
        }
        // graph search to fill the pit
        let mut q = Vec::new();
        let mut visited = HashSet::new();
        q.push((1, col_inside));
        visited.insert((1, col_inside));
        while let Some(n) = q.pop() {
            for n in self.neighbors(n) {
                if !visited.contains(&n) {
                    q.push(n);
                    visited.insert(n);
                    pit_vol += 1
                }
            }
        }
        pit_vol as u64
    }

    fn neighbors(&self, p: (usize, usize)) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        // above
        if p.0 > 0 && self.map[p.0 - 1][p.1] == '.' {
            res.push((p.0 - 1, p.1))
        }
        // below
        if p.0 < self.nrow() - 1 && self.map[p.0 + 1][p.1] == '.' {
            res.push((p.0 + 1, p.1))
        }
        // left
        if p.1 > 0 && self.map[p.0][p.1 - 1] == '.' {
            res.push((p.0, p.1 - 1))
        }
        // right
        if p.1 < self.ncol() - 1 && self.map[p.0][p.1 + 1] == '.' {
            res.push((p.0, p.1 + 1))
        }
        res
    }
}

struct Instr {
    dir: char,
    len: u32,
    color: String,
}

impl Instr {
    fn from_line(l: &str) -> Self {
        let fields: Vec<&str> = l.split_ascii_whitespace().collect();
        let dir = fields[0].chars().next().unwrap();
        let len = fields[1].parse().unwrap();
        let color = fields[2].to_owned();
        Self { dir, len, color }
    }
}

fn load_instr(filename: &str) -> Vec<Instr> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(Instr::from_line)
        .collect()
}
