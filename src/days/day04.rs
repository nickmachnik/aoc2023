use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day4input1.txt");
    let sol2: u64 = solve2("./input/day4input1.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    let mut res = 0;
    for card in get_cards(filename) {
        let winning: HashSet<u64> = HashSet::from_iter(card[0].iter().cloned());
        let having: HashSet<u64> = HashSet::from_iter(card[1].iter().cloned());
        let isct: u32 = winning.intersection(&having).count().try_into().unwrap();
        if isct > 0 {
            res += 2u64.pow(isct - 1);
        }
    }
    res
}

fn solve2(filename: &str) -> u64 {
    let cards = get_cards(filename);
    let mut card_counts = vec![1; cards.len()];
    for (card_ix, card) in get_cards(filename).iter().enumerate() {
        let winning: HashSet<u64> = HashSet::from_iter(card[0].iter().cloned());
        let having: HashSet<u64> = HashSet::from_iter(card[1].iter().cloned());
        let isct: usize = winning.intersection(&having).count();
        for i in card_ix + 1..card_ix + 1 + isct {
            card_counts[i] += card_counts[card_ix];
        }
    }
    card_counts.iter().sum()
}

fn get_cards(filename: &str) -> Vec<Vec<Vec<u64>>> {
    load_lines(filename)
        .iter()
        .map(|s| s.split(": ").nth(1).unwrap())
        .map(|s| {
            s.split(" | ")
                .map(|s| {
                    s.split_ascii_whitespace()
                        .map(|e| e.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect()
        })
        .collect()
}

fn load_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
