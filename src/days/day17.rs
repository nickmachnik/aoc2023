use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

type Map = Vec<Vec<u32>>;
const MAX_STRAIGHT: u8 = 3;
const MIN_STRT_ULTRA: u8 = 4;
const MAX_STRT_ULTRA: u8 = 10;

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day17example1.txt");
    let sol2: u64 = solve2("./input/day17input.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    let map = load_map(filename);
    cheapest_path(&map) as u64
}

fn solve2(filename: &str) -> u64 {
    let map = load_map(filename);
    cheapest_path_ultra(&map) as u64
}

#[derive(PartialEq, Default, Debug, Clone, Copy, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    #[default]
    None,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct NodeSig {
    in_dir: Direction,
    row: usize,
    col: usize,
    straight_count: u8,
}

#[derive(Default, Debug, Clone, Copy)]
struct Node {
    cumsum: u32,
    straight_count: u8,
    in_dir: Direction,
    row: usize,
    col: usize,
}

impl Node {
    fn signature(&self) -> NodeSig {
        NodeSig {
            in_dir: self.in_dir,
            row: self.row,
            col: self.col,
            straight_count: self.straight_count,
        }
    }

    fn valid_neighbors(&self, map: &Map) -> Vec<Node> {
        let mut res = Vec::new();
        if self.in_dir != Direction::Down
            && (self.straight_count < MAX_STRAIGHT || self.in_dir != Direction::Up)
        {
            if let Some(n) = self.neighbor(map, Direction::Up) {
                res.push(n);
            }
        }
        if self.in_dir != Direction::Right
            && (self.straight_count < MAX_STRAIGHT || self.in_dir != Direction::Left)
        {
            if let Some(n) = self.neighbor(map, Direction::Left) {
                res.push(n);
            }
        }
        if self.in_dir != Direction::Left
            && (self.straight_count < MAX_STRAIGHT || self.in_dir != Direction::Right)
        {
            if let Some(n) = self.neighbor(map, Direction::Right) {
                res.push(n);
            }
        }
        if self.in_dir != Direction::Up
            && (self.straight_count < MAX_STRAIGHT || self.in_dir != Direction::Down)
        {
            if let Some(n) = self.neighbor(map, Direction::Down) {
                res.push(n);
            }
        }
        res
    }

    fn neighbor(&self, map: &Map, dir: Direction) -> Option<Node> {
        let nrow = map.len();
        let ncol = map[0].len();
        match dir {
            Direction::Down => {
                if self.row < nrow - 1 {
                    Some(Node {
                        cumsum: self.cumsum + map[self.row + 1][self.col],
                        straight_count: self.new_straight_count(Direction::Down),
                        in_dir: Direction::Down,
                        row: self.row + 1,
                        col: self.col,
                    })
                } else {
                    None
                }
            }
            Direction::Up => {
                if self.row > 0 {
                    Some(Node {
                        cumsum: self.cumsum + map[self.row - 1][self.col],
                        straight_count: self.new_straight_count(Direction::Up),
                        in_dir: Direction::Up,
                        row: self.row - 1,
                        col: self.col,
                    })
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.col > 0 {
                    Some(Node {
                        cumsum: self.cumsum + map[self.row][self.col - 1],
                        straight_count: self.new_straight_count(Direction::Left),
                        in_dir: Direction::Left,
                        row: self.row,
                        col: self.col - 1,
                    })
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.col < ncol - 1 {
                    Some(Node {
                        cumsum: self.cumsum + map[self.row][self.col + 1],
                        straight_count: self.new_straight_count(Direction::Right),
                        in_dir: Direction::Right,
                        row: self.row,
                        col: self.col + 1,
                    })
                } else {
                    None
                }
            }
            Direction::None => panic!("No Direction::None allowed in move!"),
        }
    }

    fn valid_neighbors_ultra(&self, map: &Map) -> Vec<Node> {
        let mut res = Vec::new();
        if self.straight_count < MIN_STRT_ULTRA {
            if self.in_dir == Direction::None {
                res.push(self.neighbor(map, Direction::Down).unwrap());
                res.push(self.neighbor(map, Direction::Right).unwrap());
            } else if let Some(n) = self.neighbor(map, self.in_dir) {
                res.push(n);
            }
            return res;
        }
        if self.in_dir != Direction::Down
            && (self.straight_count < MAX_STRT_ULTRA || self.in_dir != Direction::Up)
        {
            if let Some(n) = self.neighbor(map, Direction::Up) {
                res.push(n);
            }
        }
        if self.in_dir != Direction::Right
            && (self.straight_count < MAX_STRT_ULTRA || self.in_dir != Direction::Left)
        {
            if let Some(n) = self.neighbor(map, Direction::Left) {
                res.push(n);
            }
        }
        if self.in_dir != Direction::Left
            && (self.straight_count < MAX_STRT_ULTRA || self.in_dir != Direction::Right)
        {
            if let Some(n) = self.neighbor(map, Direction::Right) {
                res.push(n);
            }
        }
        if self.in_dir != Direction::Up
            && (self.straight_count < MAX_STRT_ULTRA || self.in_dir != Direction::Down)
        {
            if let Some(n) = self.neighbor(map, Direction::Down) {
                res.push(n);
            }
        }
        res
    }

    fn new_straight_count(&self, dir: Direction) -> u8 {
        if self.in_dir == dir {
            self.straight_count + 1
        } else {
            1
        }
    }

    fn is_exit(&self, map: &Map) -> bool {
        if self.row == map.len() - 1 && self.col == map[0].len() - 1 {
            return true;
        }
        false
    }
}

fn cheapest_path(map: &Map) -> u32 {
    let mut front: Vec<Node> = Vec::new();
    let mut new_front: Vec<Node> = Vec::new();
    let mut visited: HashSet<NodeSig> = HashSet::new();
    let init_node = Node {
        // do not include first blocks heat loss
        cumsum: 0,
        straight_count: 0,
        in_dir: Direction::None,
        row: 0,
        col: 0,
    };
    front.push(init_node);

    loop {
        let mut min_cost = u32::MAX;
        let mut next_node = Node::default();
        while let Some(node) = front.pop() {
            let neighbors = node.valid_neighbors(map);
            let mut any_not_visited = false;
            for n in neighbors {
                if visited.contains(&n.signature()) {
                    continue;
                }
                any_not_visited = true;
                let cost = n.cumsum;
                if cost < min_cost {
                    min_cost = cost;
                    next_node = n;
                }
            }
            if any_not_visited {
                new_front.push(node);
            }
        }
        // println!("{:?}", next_node);
        if next_node.is_exit(map) {
            return next_node.cumsum;
        }
        new_front.push(next_node);
        visited.insert(next_node.signature());
        front = new_front;
        new_front = Vec::new();
    }
}

fn cheapest_path_ultra(map: &Map) -> u32 {
    let mut front: Vec<Node> = Vec::new();
    let mut new_front: Vec<Node> = Vec::new();
    let mut visited: HashSet<NodeSig> = HashSet::new();
    let init_node = Node {
        // do not include first blocks heat loss
        cumsum: 0,
        straight_count: 0,
        in_dir: Direction::None,
        row: 0,
        col: 0,
    };
    front.push(init_node);

    loop {
        let mut min_cost = u32::MAX;
        let mut next_node = Node::default();
        while let Some(node) = front.pop() {
            let neighbors = node.valid_neighbors_ultra(map);
            let mut any_not_visited = false;
            for n in neighbors {
                if visited.contains(&n.signature()) {
                    continue;
                }
                any_not_visited = true;
                let cost = n.cumsum;
                if cost < min_cost {
                    min_cost = cost;
                    next_node = n;
                }
            }
            if any_not_visited {
                new_front.push(node);
            }
        }
        if next_node.is_exit(map) && next_node.straight_count >= MIN_STRT_ULTRA {
            return next_node.cumsum;
        }
        new_front.push(next_node);
        visited.insert(next_node.signature());
        front = new_front;
        new_front = Vec::new();
    }
}

fn load_map(filename: &str) -> Map {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|e| e.to_digit(10).unwrap()).collect())
        .collect()
}
