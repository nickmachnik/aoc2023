use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day16input.txt");
    let sol2: u64 = solve2("./input/day16input.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    let map = load_map(filename);
    let init_ray = Ray {
        row: 0,
        col: 0,
        dir: Direction::Right,
    };
    ray_dfs(&map, init_ray)
}

fn solve2(filename: &str) -> u64 {
    let map = load_map(filename);
    let mut max_e = 0;
    for ray in map.init_rays() {
        max_e = max_e.max(ray_dfs(&map, ray));
    }
    max_e
}

fn ray_dfs(map: &Map, init_ray: Ray) -> u64 {
    let mut all_ray_states = HashSet::new();
    let mut q = vec![init_ray.clone()];
    all_ray_states.insert(init_ray);
    while let Some(ray) = q.pop() {
        for nb in ray.next(map) {
            if map.includes(&nb) && !all_ray_states.contains(&nb) {
                q.push(nb.clone());
                all_ray_states.insert(nb);
            }
        }
    }
    all_ray_states
        .iter()
        .map(|r| (r.row, r.col))
        .collect::<HashSet<(i32, i32)>>()
        .len() as u64
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Ray {
    row: i32,
    col: i32,
    dir: Direction,
}

impl Ray {
    fn next(&self, map: &Map) -> Vec<Ray> {
        let mut res = Vec::new();
        match map.get_symbol(self) {
            '.' => res.push(self.forward()),
            '|' => {
                if self.is_vertical() {
                    res.push(self.forward());
                } else {
                    res.push(self.dupl_with_dir(Direction::Up).forward());
                    res.push(self.dupl_with_dir(Direction::Down).forward());
                }
            }
            '-' => {
                if self.is_horizontal() {
                    res.push(self.forward());
                } else {
                    res.push(self.dupl_with_dir(Direction::Left).forward());
                    res.push(self.dupl_with_dir(Direction::Right).forward());
                }
            }
            '/' => {
                if self.is_horizontal() {
                    res.push(self.turn_left())
                } else {
                    res.push(self.turn_right())
                }
            }
            '\\' => {
                if self.is_horizontal() {
                    res.push(self.turn_right())
                } else {
                    res.push(self.turn_left())
                }
            }
            _ => panic!("Unexpected map symbol!"),
        }
        res
    }

    fn is_horizontal(&self) -> bool {
        self.dir == Direction::Left || self.dir == Direction::Right
    }

    fn is_vertical(&self) -> bool {
        self.dir == Direction::Up || self.dir == Direction::Down
    }

    fn dupl_with_dir(&self, dir: Direction) -> Ray {
        Ray {
            row: self.row,
            col: self.col,
            dir,
        }
    }

    fn turn_left(&self) -> Ray {
        match self.dir {
            Direction::Down => Ray {
                row: self.row,
                col: self.col + 1,
                dir: Direction::Right,
            },
            Direction::Up => Ray {
                row: self.row,
                col: self.col - 1,
                dir: Direction::Left,
            },
            Direction::Right => Ray {
                row: self.row - 1,
                col: self.col,
                dir: Direction::Up,
            },
            Direction::Left => Ray {
                row: self.row + 1,
                col: self.col,
                dir: Direction::Down,
            },
        }
    }

    fn turn_right(&self) -> Ray {
        match self.dir {
            Direction::Up => Ray {
                row: self.row,
                col: self.col + 1,
                dir: Direction::Right,
            },
            Direction::Down => Ray {
                row: self.row,
                col: self.col - 1,
                dir: Direction::Left,
            },
            Direction::Left => Ray {
                row: self.row - 1,
                col: self.col,
                dir: Direction::Up,
            },
            Direction::Right => Ray {
                row: self.row + 1,
                col: self.col,
                dir: Direction::Down,
            },
        }
    }

    fn forward(&self) -> Ray {
        match self.dir {
            Direction::Up => Ray {
                row: self.row - 1,
                col: self.col,
                dir: self.dir,
            },
            Direction::Down => Ray {
                row: self.row + 1,
                col: self.col,
                dir: self.dir,
            },
            Direction::Left => Ray {
                row: self.row,
                col: self.col - 1,
                dir: self.dir,
            },
            Direction::Right => Ray {
                row: self.row,
                col: self.col + 1,
                dir: self.dir,
            },
        }
    }
}

struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn get_symbol(&self, r: &Ray) -> char {
        // there should be a check on row and col bounds in here
        self.map[r.row as usize][r.col as usize]
    }

    fn nrow(&self) -> usize {
        self.map.len()
    }

    fn ncol(&self) -> usize {
        self.map[0].len()
    }

    fn includes(&self, r: &Ray) -> bool {
        r.row >= 0 && r.col >= 0 && (r.row as usize) < self.nrow() && (r.col as usize) < self.ncol()
    }

    fn init_rays(&self) -> Vec<Ray> {
        let mut res = Vec::new();
        // top row
        for c in 0..self.ncol() {
            res.push(Ray {
                row: 0,
                col: c as i32,
                dir: Direction::Down,
            });
        }
        // bottom row
        for c in 0..self.ncol() {
            res.push(Ray {
                row: self.nrow() as i32 - 1,
                col: c as i32,
                dir: Direction::Up,
            });
        }
        // left edge
        for r in 0..self.nrow() {
            res.push(Ray {
                row: r as i32,
                col: 0,
                dir: Direction::Right,
            });
        }
        // right edge
        for r in 0..self.nrow() {
            res.push(Ray {
                row: r as i32,
                col: self.ncol() as i32 - 1,
                dir: Direction::Left,
            });
        }
        res
    }
}

fn load_map(filename: &str) -> Map {
    Map {
        map: read_to_string(filename)
            .unwrap()
            .lines()
            .map(|l| l.chars().collect())
            .collect(),
    }
}
