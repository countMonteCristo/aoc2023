use std::collections::{HashSet, VecDeque};

use crate::utils;

use super::utils::Result;

type Pos = utils::Point<i32>;

const LEFT : Pos = Pos{x: -1, y:  0};
const RIGHT: Pos = Pos{x:  1, y:  0};
const UP   : Pos = Pos{x:  0, y: -1};
const DOWN : Pos = Pos{x:  0, y:  1};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Cell {
    p: Pos,
    d: &'static Pos,
}

impl Cell {
    fn add(&self, d: &'static Pos) -> Self { Self{ p: self.p.add(d), d} }

    fn next(&self, table: &Vec<Vec<char>>) -> Vec<Cell> {
        let mut res = Vec::new();
        match table[self.p.y as usize][self.p.x as usize] {
            '.' => res.push(self.add(self.d)),
            '|' => {
                if self.d.x == 0 {
                    res.push(self.add(self.d))
                } else {
                    res.push(self.add(&DOWN));
                    res.push(self.add(&UP));
                }
            },
            '-' => {
                if self.d.y == 0 {
                    res.push(self.add(self.d))
                } else {
                    res.push(self.add(&LEFT));
                    res.push(self.add(&RIGHT));
                }
            },
            '/' => {
                let d = match self.d {
                    &DOWN  => &LEFT,
                    &LEFT  => &DOWN,
                    &UP    => &RIGHT,
                    &RIGHT => &UP,
                    _ => unreachable!("")
                };
                res.push(self.add(d));
            },
            '\\' => {
                let d = match self.d {
                    &DOWN  => &RIGHT,
                    &RIGHT => &DOWN,
                    &UP    => &LEFT,
                    &LEFT  => &UP,
                    _ => unreachable!("")
                };
                res.push(self.add(d));
            },
            _ => unreachable!("")
        }

        res
    }
}

fn solve(table: &Vec<Vec<char>>, start: Cell) -> usize {
    let mut h = HashSet::<Cell>::new();
    let mut q = VecDeque::<Cell>::new();

    q.push_back(start);
    while !q.is_empty() {
        let cell = q.pop_back().unwrap();
        if cell.p.x < 0 || cell.p.x >= table[0].len() as i32 || cell.p.y < 0 || cell.p.y >= table.len() as i32 {
            continue;
        }
        if h.contains(&cell) {
            continue;
        }

        q.extend(cell.next(&table));
        h.insert(cell);
    }

    HashSet::<Pos>::from_iter(h.iter().map(|c| c.p.clone())).len()
}

fn solve1(table: &Vec<Vec<char>>) -> usize {
    solve(table, Cell{
        p: Pos{x: 0, y: 0},
        d: &RIGHT,
    })
}

fn solve2(table: &Vec<Vec<char>>) -> usize {
    let mut res = 0;

    for c in 0..table[0].len()-1 {
        res = res.max(solve(table, Cell {
            p: Pos{x: c as i32, y: 0}, d: &DOWN
        }));
        res = res.max(solve(table, Cell {
            p: Pos{x: c as i32, y: (table.len()-1) as i32}, d: &UP
        }));
    }
    for r in 0..table.len()-1 {
        res = res.max(solve(table, Cell {
            p: Pos{x: 0, y: r as i32}, d: &LEFT
        }));
        res = res.max(solve(table, Cell {
            p: Pos{x: (table[0].len()-1) as i32, y: r as i32}, d: &RIGHT
        }));
    }

    res
}

pub fn run(data: &str, check: bool) -> Result {
    let s = data.to_string();
    let table = s.split('\n').map(|s| s.chars().collect()).collect();

    let ans1 = solve1(&table);
    println!("Part1: {}", ans1);

    let ans2 = solve2(&table);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 8112 && ans2 == 8314) {
        Ok(())
    } else {
        Err(())
    }
}
