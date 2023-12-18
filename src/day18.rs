use super::utils::{Result, Point};

type N = i64;
type Pos = Point::<N>;

const LEFT : Pos = Pos{x: -1, y:  0};
const RIGHT: Pos = Pos{x:  1, y:  0};
const UP   : Pos = Pos{x:  0, y: -1};
const DOWN : Pos = Pos{x:  0, y:  1};

fn dir_by_char(c: char) -> &'static Pos {
    match c {
        '0' | 'R' => &RIGHT,
        '1' | 'D' => &DOWN,
        '2' | 'L' => &LEFT,
        '3' | 'U' => &UP,
        _ => unreachable!()
    }
}

struct Move {
    d: &'static Pos,
    n: N,
    color: String,
}

impl Move {
    fn new(line: &str) -> Self {
        let mut parts = line.split(" ");
        let d = dir_by_char(parts.next().unwrap().chars().nth(0).unwrap());
        let n = parts.next().unwrap().parse().unwrap();
        let color = parts.next().unwrap().to_string();

        Self{d, n, color}
    }

    fn colored(&self) -> Self {
        let n = N::from_str_radix(&self.color[2..7], 16).unwrap();
        let d = dir_by_char(self.color.chars().nth(7).unwrap());

        Self{n, d, color: String::new()}
    }
}

fn solve(moves: &Vec<Move>) -> N {
    let mut edge = Vec::<Pos>::new();
    let mut cur = Pos{x: 0, y: 0};
    moves
        .iter()
        .for_each(|m| {
            edge.push(cur.clone());
            cur.iadd(&m.d.mul(m.n));
        });

    let s = edge
        .iter().enumerate()
        .map(|(i, e)| {
            let ni = (i + 1) % edge.len();
            e.x * edge[ni].y -e.y * edge[ni].x
        })
        .sum::<N>()
        .abs() / 2;

    s + moves.iter().map(|m| m.n).sum::<N>() / 2 + 1
}

pub fn run(data: &str, check: bool) -> Result {
    let s = data.to_string();
    let moves = s.split('\n').map(|s| Move::new(s)).collect();

    let ans1 = solve(&moves);
    println!("Part1: {}", ans1);

    let colored_moves = moves.iter().map(|m| m.colored()).collect();

    let ans2 = solve(&colored_moves);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 108909 && ans2 == 133125706867777) {
        Ok(())
    } else {
        Err(())
    }
}
