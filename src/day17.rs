use std::{cmp::Ordering, collections::{BinaryHeap, HashSet}};

use super::utils::{Result, Point};


type N = i32;
type Pos = Point::<N>;

const LEFT : Pos = Pos{x: -1, y:  0};
const RIGHT: Pos = Pos{x:  1, y:  0};
const UP   : Pos = Pos{x:  0, y: -1};
const DOWN : Pos = Pos{x:  0, y:  1};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cell {
    p: Pos,
    d: &'static Pos,
    n: usize,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    loss: N,
    h: N,
    cell: Cell,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.loss + other.h).cmp(&(self.loss + self.h))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn rot_clockwise(d: &'static Pos) -> &'static Pos {
    match d {
        &RIGHT => &DOWN,
        &DOWN => &LEFT,
        &LEFT => &UP,
        &UP => &RIGHT,
        _ => unreachable!()
    }
}
fn rot_cnt_clockwise(d: &'static Pos) -> &'static Pos {
    match d {
        &RIGHT => &UP,
        &UP => &LEFT,
        &LEFT => &DOWN,
        &DOWN => &RIGHT,
        _ => unreachable!()
    }
}

fn at(table: &Vec<Vec<N>>, p: &Pos) -> N {
    table[p.y as usize][p.x as usize]
}
fn is_inside(p: &Pos, table: &Vec<Vec<N>>) -> bool {
    p.x >= 0 && p.x < table[0].len() as N && p.y >= 0 && p.y < table.len() as N
}

fn get_rotated(s: &State, table: &Vec<Vec<N>>, finish: &Pos, maxn: usize) -> (Option<State>, Option<State>) {
    let d1 = rot_clockwise(s.cell.d);
        let d2 = rot_cnt_clockwise(s.cell.d);
        let p1 = s.cell.p.add(d1);
        let p2 = s.cell.p.add(d2);
        let h1 = finish.x - p1.x + finish.y - p1.y;
        let h2 = finish.x - p2.x + finish.y - p2.y;
        let s1 = if is_inside(&p1, table) {
            Some( State{ loss: s.loss + at(table, &p1), h: h1, cell: Cell{p: p1, d: d1, n: maxn-1}} )
        } else { None };
        let s2 = if is_inside(&p2, table) {
            Some( State{ loss: s.loss + at(table, &p2), h: h2, cell: Cell{p: p2, d: d2, n: maxn-1}} )
        } else { None };
        (s1, s2)
}
fn get_next(s: &State, table: &Vec<Vec<N>>, finish: &Pos) -> Option<State> {
    let p = s.cell.p.add(s.cell.d);
    let h = finish.x - p.x + finish.y - p.y;
    if is_inside(&p, table) {
        Some(State { loss: s.loss + at(table, &p), h, cell: Cell{p, d: s.cell.d,  n: s.cell.n-1}, })
    } else { None }
}

fn solve(table: &Vec<Vec<N>>, minn: usize, maxn: usize) -> N {
    let mut heap = BinaryHeap::<State>::new();
    let mut visited = HashSet::<Cell>::new();
    let finish = Pos{
        x: (table[0].len() - 1) as N,
        y: (table.len() - 1) as N,
    };
    let h = (table.len() + table[0].len() - 3) as N;

    heap.push(State { loss: table[0][1], h, cell: Cell{p: Pos{x: 1, y: 0}, n: maxn-1, d: &RIGHT} } );
    heap.push(State { loss: table[1][0], h, cell: Cell{p: Pos{x: 0 ,y: 1}, n: maxn-1, d: &DOWN} } );

    while !heap.is_empty() {
        let s = heap.pop().unwrap();

        if visited.contains(&s.cell) {
            continue;
        }
        if s.cell.p == finish {
            return s.loss;
        }

        if s.cell.n > 0 {
             if let Some(ns) = get_next(&s, table, &finish) { heap.push(ns); }
        }
        if s.cell.n <= maxn - minn {
            let (s1_opt, s2_opt) = get_rotated(&s, table, &finish, maxn);
            if let Some(s1) = s1_opt { heap.push(s1); }
            if let Some(s2) = s2_opt { heap.push(s2); }
        }

        visited.insert(s.cell);
    }
    unreachable!()
}

pub fn run(data: &str, check: bool) -> Result {
    let s = data.to_string();
    let table = s
        .split('\n')
        .map(|s|
            s.chars().map(|c| c.to_digit(10).unwrap() as N).collect()
        )
        .collect();

    let ans1 = solve(&table, 1, 3);
    println!("Part1: {}", ans1);

    let ans2 = solve(&table, 4, 10);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 1238 && ans2 == 1362) {
        Ok(())
    } else {
        Err(())
    }
}
