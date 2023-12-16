use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

type Pos = (usize, usize);

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day10input1.txt");
    let sol2: String = format!("{:?}", solve2("./input/day10input1.txt"));

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    let map = Map::from_file(filename);
    map.find_loop1()
}

fn solve2(filename: &str) -> (u64, u64) {
    let map = Map::from_file(filename);
    map.enclosed_count()
}

struct Map {
    map: Vec<Vec<char>>,
    nrow: usize,
    ncol: usize,
}

impl Map {
    fn from_file(filename: &str) -> Self {
        let map = load_lines(filename);
        Self {
            ncol: map[0].len(),
            nrow: map.len(),
            map,
        }
    }

    fn enclosed_count(&self) -> (u64, u64) {
        let (loop_tiles, lhs_tiles, rhs_tiles) = self.find_loop1_set();

        // let mut outer_tile: Pos = (0, 0);
        // for i in 0..self.nrow {
        //     if !loop_tiles.contains(&(i, 0)) {
        //         outer_tile = (i, 0);
        //         break;
        //     }
        // }

        // println!(
        //     "lhs search: {:?}",
        //     self.dfs_search(outer_tile, &lhs_tiles, &loop_tiles)
        // );

        // println!(
        //     "rhs search: {:?}",
        //     self.dfs_search(outer_tile, &rhs_tiles, &loop_tiles)
        // );

        // println!(
        //     "|rhs dfs & lhs dfs| = {:?}",
        //     (&self.dfs(&lhs_tiles, &loop_tiles) & &self.dfs(&rhs_tiles, &loop_tiles)).len()
        // );

        // println!(
        //     "total tiles: {}, rhs dfs + lhs dfs + loop: {}",
        //     self.size(),
        //     self.dfs_count(&lhs_tiles, &loop_tiles)
        //         + self.dfs_count(&rhs_tiles, &loop_tiles)
        //         + loop_tiles.len() as u64
        // );

        // println!(
        //     "total tiles - lhs - loop: {}",
        //     self.size() as u64 - self.dfs_count(&lhs_tiles, &loop_tiles) - loop_tiles.len() as u64,
        // );

        // let lhs_dfs = self.dfs(&lhs_tiles, &loop_tiles);
        // let rhs_dfs = self.dfs(&rhs_tiles, &loop_tiles);
        // let mut painted_map = self.map.clone();
        // for i in 0..self.nrow {
        //     for j in 0..self.ncol {
        //         if lhs_dfs.contains(&(i, j)) {
        //             painted_map[i][j] = 'o';
        //         } else if rhs_dfs.contains(&(i, j)) {
        //             painted_map[i][j] = 'i';
        //         } else if loop_tiles.contains(&(i, j)) {
        //             // painted_map[i][j] = '.';
        //         } else {
        //             painted_map[i][j] = 'X';
        //         }
        //     }
        // }

        // for row in painted_map {
        //     let s: String = row.iter().collect();
        //     println!("{}", s);
        // }

        // println!("rhs lhs overlap: {:?}", (&rhs_tiles & &lhs_tiles).len());
        (
            self.dfs_count(&lhs_tiles, &loop_tiles),
            self.dfs_count(&rhs_tiles, &loop_tiles),
        )
    }

    fn dfs_count(&self, h: &HashSet<Pos>, l: &HashSet<Pos>) -> u64 {
        let mut count = 0;
        let mut queued = h.clone();
        let mut q: Vec<Pos> = h.clone().into_iter().collect();
        while let Some(cn) = q.pop() {
            count += 1;
            for nn in self.adj_pos(cn) {
                if !queued.contains(&nn) & !l.contains(&nn) {
                    queued.insert(nn);
                    q.push(nn);
                }
            }
        }
        count
    }

    fn find_loop1_set(&self) -> (HashSet<Pos>, HashSet<Pos>, HashSet<Pos>) {
        let mut res = HashSet::new();
        let mut rhs_nodes = HashSet::new();
        let mut lhs_nodes = HashSet::new();

        let mut curr = self.starting_pos().expect("No starting position found?");
        let mut prev = curr;
        res.insert(curr);
        let mut arrived = false;

        while !arrived {
            for n in self.neighbors(curr) {
                if n == prev {
                    continue;
                }
                prev = curr;
                curr = n;
                break;
            }
            if self.map[curr.0][curr.1] == 'S' {
                arrived = true;
            }
            res.insert(curr);
            for n in self.node_rhs(curr, prev).into_iter().flatten() {
                rhs_nodes.insert(n);
            }
            for n in self.node_lhs(curr, prev).into_iter().flatten() {
                lhs_nodes.insert(n);
            }
        }
        lhs_nodes = &lhs_nodes - &res;
        rhs_nodes = &rhs_nodes - &res;
        (res, lhs_nodes, rhs_nodes)
    }

    fn node_lhs(&self, curr: Pos, prev: Pos) -> [Option<Pos>; 2] {
        if curr.0 == prev.0 + 1 {
            // moving down
            return [
                self.validate_pos((prev.0, curr.1 + 1)),
                self.validate_pos((curr.0, curr.1 + 1)),
            ];
        } else if curr.0 == prev.0 - 1 {
            // moving up
            return [
                self.validate_pos((prev.0, curr.1 - 1)),
                self.validate_pos((curr.0, curr.1 - 1)),
            ];
        } else if curr.1 == prev.1 + 1 {
            // moving right
            return [
                self.validate_pos((curr.0 - 1, prev.1)),
                self.validate_pos((curr.0 - 1, curr.1)),
            ];
        } else if curr.1 == prev.1 - 1 {
            // moving left
            return [
                self.validate_pos((curr.0 + 1, prev.1)),
                self.validate_pos((curr.0 + 1, curr.1)),
            ];
        }
        [None, None]
    }

    fn node_rhs(&self, curr: Pos, prev: Pos) -> [Option<Pos>; 2] {
        if curr.0 == prev.0 + 1 {
            // moving down
            return [
                self.validate_pos((prev.0, curr.1 - 1)),
                self.validate_pos((curr.0, curr.1 - 1)),
            ];
        } else if curr.0 == prev.0 - 1 {
            // moving up
            return [
                self.validate_pos((prev.0, curr.1 + 1)),
                self.validate_pos((curr.0, curr.1 + 1)),
            ];
        } else if curr.1 == prev.1 + 1 {
            // moving right
            return [
                self.validate_pos((curr.0 + 1, prev.1)),
                self.validate_pos((curr.0 + 1, curr.1)),
            ];
        } else if curr.1 == prev.1 - 1 {
            // moving left
            return [
                self.validate_pos((curr.0 - 1, prev.1)),
                self.validate_pos((curr.0 - 1, curr.1)),
            ];
        }
        [None, None]
    }

    fn validate_pos(&self, p: Pos) -> Option<Pos> {
        if (p.0 <= self.max_row()) & (p.1 <= self.max_col()) {
            return Some(p);
        }
        None
    }

    fn find_loop1(&self) -> u64 {
        let mut num_steps = 0;
        let mut curr = self.starting_pos().expect("No starting position found?");
        let mut prev = curr;
        let mut arrived = false;

        while !arrived {
            num_steps += 1;
            for n in self.neighbors(curr) {
                if n == prev {
                    continue;
                }
                prev = curr;
                curr = n;
                // being lazy here;
                break;
            }
            if self.map[curr.0][curr.1] == 'S' {
                arrived = true;
            }
        }
        num_steps / 2
    }

    fn starting_pos(&self) -> Option<Pos> {
        for i in 0..self.nrow {
            for j in 0..self.ncol {
                if self.map[i][j] == 'S' {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn max_col(&self) -> usize {
        self.ncol - 1
    }

    fn max_row(&self) -> usize {
        self.nrow - 1
    }

    fn neighbors(&self, p: Pos) -> Vec<Pos> {
        match self.map[p.0][p.1] {
            '|' => self.neighbors_pipe_horizontal(p),
            '-' => self.neighbors_pipe_vertical(p),
            'L' => self.neighbors_l(p),
            'J' => self.neighbors_j(p),
            '7' => self.neighbors_7(p),
            'F' => self.neighbors_f(p),
            'S' => self.neighbors_s(p),
            '.' => vec![],
            _ => panic!("unexpected map symbol!"),
        }
    }

    fn neighbor_above(&self, p: Pos) -> Option<Pos> {
        if p.0 > 0 && "|7FS".contains(self.map[p.0 - 1][p.1]) {
            return Some((p.0 - 1, p.1));
        }
        None
    }

    fn neighbor_below(&self, p: Pos) -> Option<Pos> {
        if p.0 < self.max_row() && "|LJS".contains(self.map[p.0 + 1][p.1]) {
            return Some((p.0 + 1, p.1));
        }
        None
    }

    fn neighbor_right(&self, p: Pos) -> Option<Pos> {
        if p.1 < self.max_col() && "-J7S".contains(self.map[p.0][p.1 + 1]) {
            return Some((p.0, p.1 + 1));
        }
        None
    }

    fn neighbor_left(&self, p: Pos) -> Option<Pos> {
        if p.1 > 0 && "-FLS".contains(self.map[p.0][p.1 - 1]) {
            return Some((p.0, p.1 - 1));
        }
        None
    }

    fn neighbors_s(&self, p: Pos) -> Vec<Pos> {
        [
            self.neighbor_below(p),
            self.neighbor_right(p),
            self.neighbor_above(p),
            self.neighbor_left(p),
        ]
        .iter()
        .filter_map(|e| *e)
        .collect()
    }

    fn neighbors_f(&self, p: Pos) -> Vec<Pos> {
        [self.neighbor_below(p), self.neighbor_right(p)]
            .iter()
            .filter_map(|e| *e)
            .collect()
    }

    fn neighbors_7(&self, p: Pos) -> Vec<Pos> {
        [self.neighbor_below(p), self.neighbor_left(p)]
            .iter()
            .filter_map(|e| *e)
            .collect()
    }

    fn neighbors_j(&self, p: Pos) -> Vec<Pos> {
        [self.neighbor_above(p), self.neighbor_left(p)]
            .iter()
            .filter_map(|e| *e)
            .collect()
    }

    fn neighbors_l(&self, p: Pos) -> Vec<Pos> {
        [self.neighbor_above(p), self.neighbor_right(p)]
            .iter()
            .filter_map(|e| *e)
            .collect()
    }

    fn neighbors_pipe_vertical(&self, p: Pos) -> Vec<Pos> {
        [self.neighbor_left(p), self.neighbor_right(p)]
            .iter()
            .filter_map(|e| *e)
            .collect()
    }

    fn neighbors_pipe_horizontal(&self, p: Pos) -> Vec<Pos> {
        [self.neighbor_above(p), self.neighbor_below(p)]
            .iter()
            .filter_map(|e| *e)
            .collect()
    }

    fn adj_pos(&self, p: Pos) -> Vec<Pos> {
        let mut res = Vec::new();
        if p.0 < self.max_row() {
            res.push((p.0 + 1, p.1))
        }
        if p.1 < self.max_col() {
            res.push((p.0, p.1 + 1))
        }
        if p.0 > 0 {
            res.push((p.0 - 1, p.1))
        }
        if p.1 > 0 {
            res.push((p.0, p.1 - 1))
        }
        res
    }
}

fn load_lines(filename: &str) -> Vec<Vec<char>> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}
