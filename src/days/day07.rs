use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = solve1("./input/day7input1.txt");
    let sol2: u64 = solve2("./input/day7input1.txt");

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve1(filename: &str) -> u64 {
    let mut hands = load_lines(filename);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(ix, hand)| (ix as u64 + 1) * hand.bid)
        .sum()
}

fn solve2(filename: &str) -> u64 {
    let mut hands = load_lines2(filename);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(ix, hand)| (ix as u64 + 1) * hand.bid)
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    Pair,
    HighCard,
}

impl HandType {
    fn from_cards(cards: &Vec<u32>) -> HandType {
        let mut counts: HashMap<u32, u8> = HashMap::new();
        for card in cards {
            *counts.entry(*card).or_default() += 1;
        }
        let mut count_vec = counts.values().collect::<Vec<&u8>>();
        count_vec.sort();
        match count_vec[..] {
            [5] => HandType::Five,
            [1, 4] => HandType::Four,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::Three,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::Pair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Unexpected count vector!"),
        }
    }

    fn from_cards2(cards: &Vec<u32>) -> HandType {
        let mut counts: HashMap<u32, u8> = HashMap::new();
        for card in cards {
            *counts.entry(*card).or_default() += 1;
        }
        if counts.len() > 1 {
            let mut max_card: u32 = 1;
            let mut max_count: u8 = 0;
            for (k, v) in &counts {
                if (*k != 1) & (max_count < *v) {
                    max_card = *k;
                    max_count = *v;
                }
            }
            if let Some(joker_count) = (&mut counts).get(&1) {
                *counts.get_mut(&max_card).unwrap() += *joker_count;
                counts.remove(&1);
            }
        }
        let mut count_vec = counts.values().collect::<Vec<&u8>>();
        count_vec.sort();
        match count_vec[..] {
            [5] => HandType::Five,
            [1, 4] => HandType::Four,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::Three,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::Pair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Unexpected count vector!"),
        }
    }

    fn val(&self) -> u8 {
        match self {
            HandType::Five => 6,
            HandType::Four => 5,
            HandType::FullHouse => 4,
            HandType::Three => 3,
            HandType::TwoPair => 2,
            HandType::Pair => 1,
            HandType::HighCard => 0,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val().cmp(&other.val())
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    bid: u64,
    t: HandType,
}

impl Hand {
    fn from_line(line: &str) -> Self {
        let fields: Vec<&str> = line.split_ascii_whitespace().collect();
        let cards = fields[0].chars().map(|c| card_to_val(c)).collect();
        let bid = fields[1].parse().unwrap();
        let t = HandType::from_cards(&cards);
        Self { cards, bid, t }
    }

    fn from_line2(line: &str) -> Self {
        let fields: Vec<&str> = line.split_ascii_whitespace().collect();
        let cards = fields[0].chars().map(|c| card_to_val2(c)).collect();
        let bid = fields[1].parse().unwrap();
        let t = HandType::from_cards2(&cards);
        Self { cards, bid, t }
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        (self.t == other.t) & (self.cards == other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.t == other.t {
            return self.cards.cmp(&other.cards);
        }
        return self.t.cmp(&other.t);
    }
}

fn card_to_val2(c: char) -> u32 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unexpected card symbol!"),
    }
}

fn card_to_val(c: char) -> u32 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unexpected card symbol!"),
    }
}

fn load_lines(filename: &str) -> Vec<Hand> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| Hand::from_line(l))
        .collect()
}

fn load_lines2(filename: &str) -> Vec<Hand> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| Hand::from_line2(l))
        .collect()
}
