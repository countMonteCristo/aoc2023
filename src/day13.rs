use super::utils::Result;

struct Pattern {
    field: Vec<Vec<char>>,
    w: usize,
    h: usize,
}

impl Pattern {
    fn new(data: Vec<&str>) -> Self {
        let field = data
            .iter()
            .map(|&s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let w = field[0].len();
        let h = field.len();
        Self{field, w, h}
    }

    fn rows_diff(&self, r1: usize, r2: usize) -> usize {
        self.field[r1]
            .iter()
            .zip(self.field[r2].iter())
            .filter(|(&c1, &c2)| c1 != c2 )
            .count()
    }

    fn cols_diff(&self, c1: usize, c2: usize) -> usize {
        let mut res = 0;
        for r in 0..self.h {
            if self.field[r][c1] != self.field[r][c2] {
                res += 1;
            }
        }
        res
    }

    fn is_reflected(&self, x: usize, max: usize, edge: usize, f: fn(&Self, usize, usize)->usize) -> bool {
        let mut n = 0;
        for d in 0..=x {
            if x+1+d >= edge { break; }
            n += f(self, x-d, x+1+d);
            if n > max {
                return false;
            }
        }
        n == max
    }

    fn get_reflection(&self, max: usize) -> usize {
        let mut row = self.h;
        let mut col = self.w;

        for r in 0..self.h-1 {
            if self.is_reflected(r, max, self.h, Self::rows_diff) {
                row = r;
                break;
            }
        }
        for c in 0..self.w-1 {
            if self.is_reflected(c, max, self.w, Self::cols_diff) {
                col = c;
                break;
            }
        }

        if row == self.h && col == self.w {
            panic!("can not find reflection");
        }

        if row < self.h { 100*(row+1) } else { col+1 }
    }
}

fn read(data: &str) -> Vec<Pattern> {
    let mut res = Vec::<Pattern>::new();
    let mut cur = Vec::<&str>::new();
    for line in data.split('\n') {
        if line.is_empty() {
            res.push(Pattern::new(cur));
            cur = Vec::<&str>::new();
        } else {
            cur.push(line);
        }
    }
    if !cur.is_empty() {
        res.push(Pattern::new(cur));
    }

    res
}

fn solve1(data: &str) -> usize {
    read(data)
        .iter()
        .map(|p| p.get_reflection(0))
        .sum()
}

fn solve2(data: &str) -> usize {
    read(data)
        .iter()
        .map(|p| p.get_reflection(1))
        .sum()
}

pub fn run(data: &str, check: bool) -> Result {
    let ans1 = solve1(data);
    println!("Part1: {}", ans1);

    let ans2 = solve2(data);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 30575 && ans2 == 37478) {
        Ok(())
    } else {
        Err(())
    }
}
