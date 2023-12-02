use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

const RED_MAX: u64 = 12;
const GREEN_MAX: u64 = 13;
const BLUE_MAX: u64 = 14;

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day2input1.txt");
    let sol2: u64 = solve2("./input/day2input1.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    parse_games(filename)
        .iter()
        .filter(|g| g.is_possible())
        .map(|g| g.id)
        .sum()
}

fn solve2(filename: &str) -> u64 {
    parse_games(filename)
        .iter()
        .map(|g| g.min_set_power())
        .sum()
}

struct CubeSet {
    cubes: HashMap<String, u64>,
}

impl CubeSet {
    fn new() -> Self {
        let mut res = Self {
            cubes: HashMap::new(),
        };
        res.cubes.insert("blue".to_string(), 0);
        res.cubes.insert("red".to_string(), 0);
        res.cubes.insert("green".to_string(), 0);
        res
    }

    fn add_count(&mut self, color: String, count: u64) {
        self.cubes.insert(color, count);
    }

    fn get_count(&self, color: &str) -> &u64 {
        self.cubes.get(color).unwrap()
    }

    fn is_possible(&self) -> bool {
        (*self.cubes.get("blue").unwrap() <= BLUE_MAX)
            & (*self.cubes.get("red").unwrap() <= RED_MAX)
            & (*self.cubes.get("green").unwrap() <= GREEN_MAX)
    }
}

struct Game {
    id: u64,
    sets: Vec<CubeSet>,
}

impl Game {
    fn new(id: u64) -> Self {
        Game {
            id,
            sets: Vec::new(),
        }
    }

    fn is_possible(&self) -> bool {
        self.sets.iter().all(|s| s.is_possible())
    }

    fn min_set_power(&self) -> u64 {
        let a = self.sets.iter().map(|s| s.get_count("red")).max().unwrap();
        let b = self.sets.iter().map(|s| s.get_count("blue")).max().unwrap();
        let c = self
            .sets
            .iter()
            .map(|s| s.get_count("green"))
            .max()
            .unwrap();
        a * b * c
    }
}

fn parse_games(filename: &str) -> Vec<Game> {
    let mut res = Vec::new();
    for (line_ix, line) in load_lines(filename).iter().enumerate() {
        let game_id = (line_ix + 1) as u64;
        let mut game = Game::new(game_id);
        for set_str in line
            .strip_prefix(&format!("Game {}: ", game_id))
            .unwrap()
            .split("; ")
            .collect::<Vec<&str>>()
        {
            let mut set = CubeSet::new();
            for (count_str, color_str) in set_str
                .split(", ")
                .map(|s| s.split_ascii_whitespace())
                .map(|mut l| (l.next().unwrap(), l.next().unwrap()))
            {
                set.add_count(color_str.to_owned(), count_str.parse().unwrap())
            }
            game.sets.push(set);
        }
        res.push(game);
    }
    res
}

fn load_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
