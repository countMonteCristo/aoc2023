use super::utils::Result;

use std::{collections::HashMap, cmp::Ordering};

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    counts: Vec<u8>,
    counts_j: Vec<u8>,
    bid: u64
}

impl<'a> Hand<'a> {
    fn new(s: &'a str) -> Self {
        let p = s.split(' ').collect::<Vec<_>>();
        let mut counts_map = HashMap::<char, u8>::new();
        let mut counts_j_map = HashMap::<char, u8>::new();
        let mut nj: u8 = 0;
        for c in p[0].chars() {
            if let Some(n) = counts_map.get_mut(&c) {
                *n += 1 ;
            } else {
                counts_map.insert(c, 1);
            }
            if c == 'J' {
                nj += 1;
            } else {
                if let Some(n) = counts_j_map.get_mut(&c) {
                    *n += 1 ;
                } else {
                    counts_j_map.insert(c, 1);
                }
            }
        }
        let counts = counts_map.into_values().collect::<Vec<_>>();
        let mut counts_j = counts_j_map.into_values().collect::<Vec<_>>();

        if nj > 0 {
            if counts_j.len() == 0 {
                counts_j.push(5);
            } else {
                let max = counts_j.iter().max().unwrap().to_owned();
                let max_pos = counts_j.iter().position(|&x| x == max).unwrap();
                counts_j[max_pos] += nj as u8;
            }
        } else {
            counts_j = counts.clone();
        }

        Hand{cards: p[0], counts, bid: p[1].parse::<u64>().unwrap(), counts_j}
    }
}


struct HandComaprator {
    cards: Vec<char>,
    get_counts_: for<'a> fn(h: &'a Hand<'a>) -> &'a Vec<u8>,
}

impl HandComaprator {
    fn get_counts<'a>(&'a self, h: &'a Hand) -> &Vec<u8> {
        (self.get_counts_)(h)
    }
}


fn compare(c1: char, c2: char, cmp: &HandComaprator) -> i8 {
    let i1 = cmp.cards.iter().position(|&c| c == c1).unwrap();
    let i2 = cmp.cards.iter().position(|&c| c == c2).unwrap();
    if i1 < i2 {
        return 1;
    }
    if i1 > i2 {
        return -1
    }
    0
}

fn order(cards1: &str, cards2: &str, cmp: &HandComaprator) -> Ordering {
    let order = cards1
        .chars()
        .zip(cards2.chars())
        .map(|(c1, c2)| compare(c1, c2, cmp))
        .filter(|&i| i != 0)
        .next().unwrap();
    order.cmp(&1)
}

fn is_five(counts: &Vec<u8>) -> u8 {
    let five: u8 = 5;
    if counts.contains(&five) {
        7
    } else { 0 }
}
fn is_four(counts: &Vec<u8>) -> u8 {
    let four: u8 = 4;
    if counts.contains(&four) {
        6
    } else { 0 }
}
fn is_full(counts: &Vec<u8>) -> u8 {
    let three: u8 = 3;
    let two: u8 = 2;
    if counts.contains(&three) && counts.contains(&two) {
        5
    } else { 0 }
}
fn is_three(counts: &Vec<u8>) -> u8 {
    let one: u8 = 1;
    let three: u8 = 3;
    if counts.iter().filter(|&x| x.to_owned() == one).count() == 2 && counts.iter().filter(|&x| x.to_owned() == three).count() == 1 {
        4
    } else { 0 }
}
fn is_two_pairs(counts: &Vec<u8>) -> u8 {
    let two: u8 = 2;
    if counts.iter().filter(|&x| x.to_owned() == two).count() == 2 {
        3
    } else { 0 }
}
fn is_one_pair(counts: &Vec<u8>) -> u8 {
    let two: u8 = 2;
    if counts.iter().filter(|&x| x.to_owned() == two).count() == 1 {
        2
    } else { 0 }
}
fn is_high(counts: &Vec<u8>) -> u8 {
    let one: u8 = 1;
    if counts.iter().filter(|&x| x.to_owned() == one).count() == 5 {
        1
    } else { 0 }
}

fn get_rank(h: &Hand, cmp: &HandComaprator) -> u8 {
    let fs = vec![is_five, is_four, is_full, is_three, is_two_pairs, is_one_pair, is_high];

    fs
        .iter()
        .map(|f| f(cmp.get_counts(h)))
        .filter(|&x| x > 0)
        .collect::<Vec<_>>()
        .first().unwrap().to_owned()
}

fn solve(hands: & mut Vec<Hand>, cmp: &HandComaprator) -> u64 {
    hands
        .sort_by(|h1, h2| {
            let r1 = get_rank(h1, cmp);
            let r2 = get_rank(h2, cmp);
            if r1 != r2 { r1.cmp(&r2) } else { order(h1.cards, h2.cards, cmp) }
        });

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i as u64 + 1))
        .sum()
}


pub fn run(data: &str, check: bool) -> Result {
    let s = data.to_string();
    let lines: Vec<&str> = s.split('\n').filter(|s| s.len() > 0).collect();

    let mut hands = lines
        .iter()
        .map(|&s| Hand::new(s))
        .collect::<Vec<_>>();

    let cmp1 = HandComaprator{
        cards: vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'],
        get_counts_: |h: &Hand| &h.counts,
    };

    let ans1 = solve(&mut hands, &cmp1);
    println!("Part1: {}", ans1);

    let cmp2 = HandComaprator{
        cards: vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'],
        get_counts_: |h: &Hand| &h.counts_j,
    };

    let ans2 = solve(&mut hands, &cmp2);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 250453939 && ans2 == 248652697) {
        Ok(())
    } else {
        Err(())
    }
}
