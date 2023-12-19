use crate::{Solution, SolutionPair};
use core::panic;
use std::{collections::HashMap, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

type Workflows = HashMap<String, Workflow>;

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day19input.txt");
    let sol2: u64 = solve2("./input/day19input.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    let (workflows, parts) = parse_input(filename);
    parts
        .iter()
        .filter(|p| p.accepted(&workflows))
        .map(|p| p.rating_sum())
        .sum::<usize>() as u64
}

fn solve2(filename: &str) -> u64 {
    let (workflows, _) = parse_input(filename);
    let mut accepted_count = 0;
    let mut q = vec![(PartRange::default(), "in".to_owned())];
    while let Some((r, dest)) = q.pop() {
        if dest == "A" {
            accepted_count += r.distinct_combs();
        } else if dest != "R" {
            q.extend(workflows.get(&dest).unwrap().process_range(r).into_iter())
        }
    }
    accepted_count
}

fn field2ix(field: char) -> usize {
    match field {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!("Unexpected field char!"),
    }
}

fn cmp(a: usize, b: usize, op: char) -> bool {
    match op {
        '>' => a > b,
        '<' => a < b,
        _ => panic!("Unexpected op!"),
    }
}

#[derive(Debug, Clone)]
struct PartRange {
    // both ends inclusive
    range: [(usize, usize); 4],
}

impl PartRange {
    fn default() -> Self {
        Self {
            range: [(1, 4000); 4],
        }
    }

    fn distinct_combs(&self) -> u64 {
        self.range.iter().map(|r| (r.1 - r.0 + 1) as u64).product()
    }

    fn in_range(&self, field: char, val: usize) -> bool {
        val >= self.range[field2ix(field)].0 && val <= self.range[field2ix(field)].1
    }

    fn below(&self, field: char, val: usize) -> bool {
        self.range[field2ix(field)].1 < val
    }

    fn split_at(&self, field: char, val: usize, op: char) -> [Self; 2] {
        let mut lo = self.clone();
        let mut hi = self.clone();
        if op == '>' {
            // lo includes val
            lo.range[field2ix(field)].1 = val;
            hi.range[field2ix(field)].0 = val + 1;
        } else {
            // hi includes var
            lo.range[field2ix(field)].1 = val - 1;
            hi.range[field2ix(field)].0 = val;
        }
        [lo, hi]
    }
}

#[derive(Debug)]
struct Rule {
    field: char,
    val: usize,
    op: char,
    dest: String,
}

impl Rule {
    fn default() -> Self {
        Self {
            field: 'x',
            val: 0,
            op: '>',
            dest: "A".to_owned(),
        }
    }

    fn process_part(&self, p: &Part) -> Option<String> {
        if cmp(p.rating[field2ix(self.field)], self.val, self.op) {
            Some(self.dest.clone())
        } else {
            None
        }
    }

    fn process_range(&self, r: &PartRange) -> Vec<(PartRange, Option<String>)> {
        let mut res = Vec::new();
        if r.in_range(self.field, self.val) {
            // split
            let [lo, hi] = r.split_at(self.field, self.val, self.op);
            match self.op {
                '>' => {
                    res.push((lo, None));
                    res.push((hi, Some(self.dest.clone())));
                }
                '<' => {
                    res.push((lo, Some(self.dest.clone())));
                    res.push((hi, None));
                }
                _ => panic!("Invalid op!"),
            }
        } else {
            let below = r.below(self.field, self.val);
            if (below && self.op == '<') || (!below && self.op == '>') {
                res.push((r.clone(), Some(self.dest.clone())));
            } else {
                res.push((r.clone(), None));
            }
        }
        res
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    final_dest: String,
}

impl Workflow {
    fn from_str(s: &str) -> Self {
        let mut buf = Vec::new();
        let mut name = "".to_owned();
        let mut curr_rule = Rule::default();
        let mut rules = Vec::new();
        let mut final_dest = "".to_owned();
        for c in s.chars() {
            match c {
                '{' => {
                    name = buf.iter().collect();
                    buf.clear();
                }
                '>' | '<' => {
                    curr_rule.field = buf[0];
                    curr_rule.op = c;
                    buf.clear();
                }
                ':' => {
                    curr_rule.val = buf.iter().collect::<String>().parse().unwrap();
                    buf.clear();
                }
                ',' => {
                    curr_rule.dest = buf.iter().collect();
                    rules.push(curr_rule);
                    curr_rule = Rule::default();
                    buf.clear();
                }
                '}' => {
                    final_dest = buf.iter().collect();
                }
                _ => buf.push(c),
            }
        }
        Self {
            name,
            rules,
            final_dest,
        }
    }

    fn process_range(&self, range: PartRange) -> Vec<(PartRange, String)> {
        let mut res = Vec::new();
        let mut q = vec![range];
        let mut next_q = Vec::new();
        for rule in &self.rules {
            while let Some(r) = q.pop() {
                for (nr, dest) in rule.process_range(&r) {
                    if let Some(d) = dest {
                        res.push((nr, d));
                    } else {
                        next_q.push(nr);
                    }
                }
            }
            q = next_q.clone();
            next_q.clear();
        }
        // leftover ranges go to final dest
        for r in q {
            res.push((r, self.final_dest.clone()));
        }
        res
    }

    fn process_part(&self, p: &Part) -> String {
        for rule in &self.rules {
            if let Some(dest) = rule.process_part(p) {
                return dest;
            }
        }
        self.final_dest.clone()
    }
}

#[derive(Debug)]
struct Part {
    rating: [usize; 4],
}

impl Part {
    fn from_str(s: &str) -> Self {
        let mut rating = [0; 4];
        let mut rating_ix = 0;
        let mut buf = String::new();
        for c in s.chars() {
            if c.is_ascii_digit() {
                buf.push(c);
            } else if ",}".contains(c) {
                rating[rating_ix] = buf.parse().unwrap();
                rating_ix += 1;
                buf.clear();
            }
        }
        Self { rating }
    }

    fn rating_sum(&self) -> usize {
        self.rating.iter().sum()
    }

    fn accepted(&self, workflows: &Workflows) -> bool {
        let mut curr_dest = "in".to_owned();
        loop {
            if curr_dest == "A" {
                return true;
            } else if curr_dest == "R" {
                return false;
            } else {
                curr_dest = workflows.get(&curr_dest).unwrap().process_part(self)
            }
        }
    }
}

fn parse_input(filename: &str) -> (Workflows, Vec<Part>) {
    let mut workflows: Workflows = HashMap::new();
    let mut parts = Vec::new();
    let mut read_workflows = true;
    for line in read_to_string(filename).unwrap().lines() {
        if line.is_empty() {
            read_workflows = false;
        } else if read_workflows {
            let workflow = Workflow::from_str(line);
            workflows.insert(workflow.name.clone(), workflow);
        } else {
            let part = Part::from_str(line);
            parts.push(part);
        }
    }
    (workflows, parts)
}
