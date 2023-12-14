use std::collections::HashMap;

use super::utils::Result;

#[derive(Clone)]
struct Platform {
    table: Vec<Vec<char>>,
    w: usize,
    h: usize,
}

impl Platform {
    fn new(lines: &Vec<&str>) -> Self {
        let table = lines
            .iter()
            .map(|&s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let w = table[0].len();
        let h = table.len();

        Self{table, w, h}
    }

    fn move_rock(&mut self, from: (usize, usize), to: (usize, usize)) {
        self.table[from.0][from.1] = '.';
        self.table[to.0][to.1] = 'O';
    }

    fn can_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        self.table[from.0][from.1] == 'O' && self.table[to.0][to.1] == '.'
    }

    fn tilt_north(&mut self) {
        for row in 1..self.h {
            for col in 0..self.w {
                let mut cur_row = row;
                while cur_row > 0 && self.can_move((cur_row, col), (cur_row-1, col)){
                    self.move_rock((cur_row, col), (cur_row-1, col));
                    cur_row -= 1;
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for row in (0..=self.h-1).rev() {
            for col in 0..self.w {
                let mut cur_row = row;
                while cur_row < self.h - 1 && self.can_move((cur_row, col), (cur_row+1, col)) {
                    self.move_rock((cur_row, col), (cur_row+1, col));
                    cur_row += 1;
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for col in 1..self.w {
            for row in 0..self.h {
                let mut cur_col = col;
                while cur_col > 0 && self.can_move((row, cur_col), (row, cur_col-1)) {
                    self.move_rock((row, cur_col), (row, cur_col-1));
                    cur_col -= 1;
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for col in (0..=self.w-1).rev() {
            for row in 0..self.h {
                let mut cur_col = col;
                while cur_col < self.w - 1 && self.can_move((row, cur_col), (row, cur_col+1)) {
                    self.move_rock((row, cur_col), (row, cur_col+1));
                    cur_col += 1;
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn calc_load(&self) -> usize {
        self.table
            .iter().enumerate()
            .map(|(i, l)| (self.h - i) * l.iter().filter(|&&c| c == 'O').count())
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
    let s = data.to_string();
    let lines: Vec<&str> = s.split('\n').filter(|s| s.len() > 0).collect();
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
