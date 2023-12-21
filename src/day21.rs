use std::collections::HashSet;

use super::utils::{Result, Point};

type I = i32;
type Pos = Point<I>;

const LEFT : Pos = Pos{x: -1, y:  0};
const RIGHT: Pos = Pos{x:  1, y:  0};
const UP   : Pos = Pos{x:  0, y: -1};
const DOWN : Pos = Pos{x:  0, y:  1};

fn nbrs(p: &Pos) -> Vec<Pos> {
    vec![p.add(&UP), p.add(&LEFT), p.add(&DOWN), p.add(&RIGHT)]
}

fn available1(p: &Pos, f: &Vec<Vec<char>>) -> bool {
    p.x >= 0 && p.x < f[0].len() as I && p.y >=0 && p.y < f.len() as I && f[p.y as usize][p.x as usize] != '#'
}

fn available2(p: &Pos, f: &Vec<Vec<char>>) -> bool {
    wrapped(p, f) != '#'
}

fn wrapped(p: &Pos, f: &Vec<Vec<char>>) -> char {
    let h = f.len() as I;
    let w = f[0].len() as I;
    let x = ((p.x % w) + w) % w;
    let y = ((p.y % h) + h) % h;
    f[y as usize][x as usize]
}

fn get_start(field: &Vec<Vec<char>>) -> Pos {
    for r in 0..field.len() {
        for c in 0..field[0].len() {
            if field[r][c] == 'S' {
                return Pos{x: c as I, y: r as I};
            }
        }
    }
    unreachable!();
}

fn count_steps(field: &Vec<Vec<char>>, start: &Pos, av: fn(&Pos, &Vec<Vec<char>>)->bool, n: usize) -> Vec<usize> {
    let mut res = Vec::new();

    let mut edge = HashSet::<Pos>::new();
    edge.insert(start.clone());
    for _ in 1..=n {
        edge = edge
            .iter()
            .map(|p| nbrs(&p))
            .flatten()
            .filter(|p| av(p, &field))
            .collect::<HashSet<_>>();

        res.push(edge.len());
    }

    res
}

fn solve1(field: &Vec<Vec<char>>, n: usize) -> usize {
    count_steps(&field, &get_start(field), available1, n)[n-1]
}

// f(xi) = a*xi*xi + b*xi + c = vi
// xi = h//2 + (i-1)*h
// f(x) = ?
fn quad(h: f64, v1: f64, v2: f64, v3: f64, x: f64) -> usize {
    let hh2 = 2. * h * h;
    let half = (h - 1.) / 2.;
    let a = (v3 - 2.*v2 + v1) / hh2;
    let b = ((v2 - v1)*(4.*h-1.) - (v3 - v2)*(2.*h-1.))/ hh2;
    let c = v1 - a * half * half - b * half;
    (a*x*x + b*x + c).round() as usize
}

fn solve2(field: &Vec<Vec<char>>, n: usize) -> usize {
    let start = get_start(field);

    let h = field.len();
    let half = (h - 1) / 2;
    let maxn = 2*h + half;

    let res = count_steps(&field, &start, available2, maxn);
    quad(h as f64, res[half-1] as f64, res[half-1+h] as f64, res[half-1+2*h] as f64, n as f64)
}

pub fn run(data: &str, check: bool) -> Result {
    let field = data.split('\n').map(|s| s.chars().collect()).collect();

    let ans1 = solve1(&field, 64);
    println!("Part1: {}", ans1);

    let ans2 = solve2(&field, 26501365);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 3594 && ans2 == 605247138198755) {
        Ok(())
    } else {
        Err(())
    }
}
