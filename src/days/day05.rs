use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

const NUM_MAPS: usize = 7;

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day5input1.txt");
    let sol2: u64 = solve2("./input/day5input1.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    let almanach = parse_input_lines(filename);
    *almanach.map_all_seeds().iter().min().unwrap()
}

fn solve2(filename: &str) -> u64 {
    let almanach = parse_input_lines(filename);
    almanach.min_all_mapped_seeds()
}

fn parse_input_lines(filename: &str) -> Almanach {
    let mut almanach = Almanach::default();
    let file = read_to_string(filename).unwrap();
    let mut lines = file.lines();
    // first line has seeds:
    almanach.seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|e| e.parse::<u64>().unwrap())
        .collect();
    // skip empty line
    lines.next();
    for map_ix in 0..NUM_MAPS {
        // skip map description
        lines.next();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            almanach.contig_maps[map_ix].add(
                line.split_ascii_whitespace()
                    .map(|e| e.parse::<u64>().unwrap())
                    .collect(),
            );
        }
    }
    almanach
}

#[derive(Default)]
struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    fn add(&mut self, v: MapEntry) {
        self.entries.push(v);
    }

    fn map_to_dst(&self, v: u64) -> u64 {
        for map_entry in &self.entries {
            if let Some(dst) = map_entry.map_to_dst(v) {
                return dst;
            }
        }
        v
    }
}

struct MapEntry {
    dst_start: u64,
    src_start: u64,
    range_len: u64,
}

impl FromIterator<u64> for MapEntry {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        let mut c = MapEntry::new();
        let mut i = iter.into_iter();
        c.dst_start = i.next().unwrap();
        c.src_start = i.next().unwrap();
        c.range_len = i.next().unwrap();
        c
    }
}

impl MapEntry {
    fn new() -> Self {
        MapEntry {
            dst_start: 0,
            src_start: 0,
            range_len: 0,
        }
    }

    fn map_to_dst(&self, v: u64) -> Option<u64> {
        if (v >= self.src_start) & (v < self.src_start + self.range_len) {
            return Some(self.dst_start + (v - self.src_start));
        }
        None
    }
}

#[derive(Default)]
struct Almanach {
    seeds: Vec<u64>,
    contig_maps: [Map; NUM_MAPS],
}

impl Almanach {
    fn map_seed(&self, seed: u64) -> u64 {
        let mut v = seed;
        for map in &self.contig_maps {
            v = map.map_to_dst(v);
        }
        v
    }

    fn map_all_seeds(&self) -> Vec<u64> {
        self.seeds.iter().map(|s| self.map_seed(*s)).collect()
    }

    fn min_all_mapped_seeds(&self) -> u64 {
        let mut min = self.map_seed(self.seeds[0]);
        for pair in self.seeds.chunks_exact(2) {
            println!("at seed pair: {:?}", pair);
            for seed in pair[0]..pair[0] + pair[1] {
                min = min.min(self.map_seed(seed));
            }
        }
        min
    }
}
