use std::collections::HashSet;

use super::utils::Result;

type N = i32;

#[derive(Debug)]
struct Card {
    id: u32,
    n: u32,
}

fn get_cards(lines: &Vec<&str>) -> Vec<Card> {
    lines
        .iter()
        .map(|&m| {
            let parts = m.split(':').collect::<Vec<_>>();
            let id = parts[0].split(' ').last().unwrap().parse::<u32>().unwrap() - 1;
            (id, parts[1])
        })
        .map(|(id, l)|
            (id, l
                .split('|')
                .map(|s|
                    HashSet::<N>::from_iter(s
                        .trim()
                        .split(' ')
                        .filter(|&x| !x.is_empty())
                        .map(|x| x.parse().unwrap())
                    )
                )
                .collect::<Vec<HashSet<_>>>())
        )
        .map(|(id, v)| {
            let n = v[0].intersection(&v[1]).count() as u32;
            Card{id, n}
        })
        .collect()
}

fn solve1(cards: &Vec<Card>) -> i32 {
    cards
        .iter()
        .map(|c| {
            if c.n == 0 {0} else {2_i32.pow(c.n - 1)}
        })
        .sum()
}

fn solve2(cards: &Vec<Card>) -> u32 {
    let mut counts: Vec<u32> = vec![1; cards.len()];

    for c in cards {
        for i in (c.id+1)..=(c.id+c.n) {
            counts[i as usize] += counts[c.id as usize];
        }
    }

    counts.iter().sum()
}

pub fn run(data: &str, check: bool) -> Result {
    let lines = data.split('\n').collect();
    let cards = get_cards(&lines);

    let ans1 = solve1(&cards);
    println!("Part1: {}", ans1);

    let ans2 = solve2(&cards);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 25004 && ans2 == 14427616) {
        Ok(())
    } else {
        Err(())
    }
}
