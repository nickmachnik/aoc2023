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
    almanach.min_all_mapped_seeds_range()
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

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    len: u64,
}

impl Range {
    fn end(&self) -> u64 {
        self.start + self.len - 1
    }

    fn pre(&self, v: u64) -> Option<Range> {
        if self.start < v {
            return Some(Range {
                start: self.start,
                len: (v - self.start).min(self.end() - self.start + 1),
            });
        }
        None
    }

    fn post(&self, v: u64) -> Option<Range> {
        if self.end() > v {
            let post_start = (v + 1).max(self.start);
            return Some(Range {
                start: post_start,
                len: self.end() - post_start + 1,
            });
        }
        None
    }
}

#[derive(Default, Debug)]
struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    fn add(&mut self, v: MapEntry) {
        self.entries.push(v);
    }

    fn map_range_to_dst(&self, r: Range) -> Vec<Range> {
        let mut mapped = Vec::new();
        let mut to_map = vec![r];
        for map_entry in &self.entries {
            let mut leftovers = Vec::new();
            while let Some(cr) = to_map.pop() {
                let mut map_res = map_entry.map_range_to_dst(cr);
                if let Some(mapped_r) = map_res.mapped {
                    mapped.push(mapped_r);
                }
                leftovers.append(&mut map_res.not_mapped);
            }
            if leftovers.is_empty() {
                break;
            }
            to_map = leftovers;
        }
        // add everything that was not mapped, these values are 1-1 mapped
        mapped.append(&mut to_map);
        mapped
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

#[derive(Debug)]
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

struct MapRangeResult {
    mapped: Option<Range>,
    not_mapped: Vec<Range>,
}

impl MapEntry {
    fn new() -> Self {
        MapEntry {
            dst_start: 0,
            src_start: 0,
            range_len: 0,
        }
    }

    fn src_end(&self) -> u64 {
        self.src_start + self.range_len - 1
    }

    fn map_range_to_dst(&self, r: Range) -> MapRangeResult {
        if (r.end() >= self.src_start) & (r.start <= self.src_end()) {
            let mapped_start = self.src_start.max(r.start);
            let mapped_end = self.src_end().min(r.end());
            let mapped = Range {
                start: self.map_to_dst(mapped_start).unwrap(),
                len: mapped_end - mapped_start + 1,
            };
            let mut not_mapped = Vec::new();
            if let Some(p) = r.pre(self.src_start) {
                not_mapped.push(p);
            }
            if let Some(p) = r.post(self.src_end()) {
                not_mapped.push(p);
            }
            return MapRangeResult {
                mapped: Some(mapped),
                not_mapped,
            };
        }
        MapRangeResult {
            mapped: None,
            not_mapped: vec![r],
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

    fn min_all_mapped_seeds_range(&self) -> u64 {
        let mut min = u64::MAX;
        for range in self.seeds.chunks_exact(2).map(|pair| Range {
            start: pair[0],
            len: pair[1],
        }) {
            let mut mapped = vec![range];
            for map in &self.contig_maps {
                let mut next_mapped = Vec::new();
                for r in &mapped {
                    next_mapped.append(&mut map.map_range_to_dst(*r));
                }
                mapped = next_mapped;
            }
            for r in mapped {
                min = min.min(r.start);
            }
        }
        min
    }
}
