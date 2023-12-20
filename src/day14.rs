use std::collections::HashMap;

use super::utils::{Result, Point};

type I = i32;
type Pos = Point<I>;
type Diff = Point<I>;

const ROCK: char = 'O';
const EMPTY: char = '.';

#[derive(Clone)]
struct Platform {
    table: Vec<Vec<char>>,
    w: I,
    h: I,
}

impl Platform {
    fn new(lines: &Vec<&str>) -> Self {
        let table = lines
            .iter()
            .map(|&s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let w = table[0].len() as I;
        let h = table.len() as I;

        Self{table, w, h}
    }

    fn tilts() -> &'static [fn(&mut Self)] {
        &[Self::tilt_north, Self::tilt_west, Self::tilt_south, Self::tilt_east]
    }

    fn move_if_can(&mut self, from: &mut Pos, d: &Diff) -> bool {
        let to = from.add(d);
        let can = self.table[from.y as usize][from.x as usize] == ROCK && self.table[to.y as usize][to.x as usize] == EMPTY;
        if can {
            self.table[from.y as usize][from.x as usize] = EMPTY;
            self.table[to.y as usize][to.x as usize] = ROCK;
            *from = to;
        }
        can
    }

    fn tilt_north(&mut self) {
        let d = Diff{y: -1, x: 0};
        for row in 1..self.h {
            for col in 0..self.w {
                let mut p = Pos{y: row, x: col};
                while p.y > 0 && self.move_if_can(&mut p, &d) {}
            }
        }
    }

    fn tilt_south(&mut self) {
        let d = Diff{y: 1, x: 0};
        for row in (0..self.h).rev() {
            for col in 0..self.w {
                let mut p = Pos{y: row, x: col};
                while p.y < self.h - 1 && self.move_if_can(&mut p, &d) {}
            }
        }
    }

    fn tilt_west(&mut self) {
        let d = Diff{y: 0, x: -1};
        for col in 1..self.w {
            for row in 0..self.h {
                let mut p = Pos{y: row, x: col};
                while p.x > 0 && self.move_if_can(&mut p, &d) {}
            }
        }
    }

    fn tilt_east(&mut self) {
        let d = Diff{y: 0, x: 1};
        for col in (0..self.w).rev() {
            for row in 0..self.h {
                let mut p = Pos{y: row, x: col};
                while p.x < self.w - 1 && self.move_if_can(&mut p, &d) {}
            }
        }
    }

    fn cycle(&mut self) {
        Self::tilts().iter().for_each(|f| f(self));
    }

    fn calc_load(&self) -> usize {
        self.table
            .iter().enumerate()
            .map(|(i, l)| (self.h as usize - i) * l.iter().filter(|&&c| c == 'O').count())
            .sum()
    }

    fn hash(&self) -> String {
        self.table
            .iter()
            .flatten()
            .collect::<String>()
    }
}

fn solve1(p: &mut Platform) -> usize {
    p.tilt_north();
    p.calc_load()
}

fn solve2(p: &mut Platform) -> usize {
    let mut h = HashMap::<String, usize>::new();
    let mut v = Vec::<Platform>::new();

    #[allow(unused_assignments)]
    let mut period = 0;

    #[allow(unused_assignments)]
    let mut start = 0;

    let mut i = 0;
    loop {
        p.cycle();
        v.push(p.clone());
        let hash = p.hash();
        if let Some(&ip) = h.get(&hash) {
            start = ip;
            period = i - ip;
            break;
        } else {
            h.insert(hash, i);
        }
        i += 1;
    }

    let x = (1000000000 - start) % period;
    v[start + x -1].calc_load()
}

pub fn run(data: &str, check: bool) -> Result {
    let lines = data.split('\n').collect();
    let platform = Platform::new(&lines);

    let ans1 = solve1(&mut platform.clone());
    println!("Part1: {}", ans1);

    let ans2 = solve2(&mut platform.clone());
    println!("Part2: {}", ans2);

    if !check || (ans1 == 108889 && ans2 == 104671) {
        Ok(())
    } else {
        Err(())
    }
}
