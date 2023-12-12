use std::collections::{HashMap, HashSet};

use super::utils::Result;

type I = i16;
type Pos = (I, I);

fn get_next(p: &Pos, dir: usize, size: &Pos) -> Option<Pos> {
    let dp = match dir {
        0 => (-1, 0),
        1 => (0, -1),
        2 => (1, 0),
        3 => (0, 1),
        _ => panic!("unhandled direction")
    };
    let np = (p.0 + dp.0, p.1 + dp.1);
    if np.0 < 0 || np.0 >= size.0 || np.1 < 0 || np.1 >= size.1 {
        None
    } else {
        Some(np)
    }
}

#[derive(Debug)]
struct Pipe {
    ends: Vec<I>,
}

impl Pipe {
    fn create() -> HashMap<char, Self> {
        HashMap::from([
            ('|', Pipe{ends: vec![0, 1, 0, 1]}),
            ('-', Pipe{ends: vec![1, 0, 1, 0]}),
            ('L', Pipe{ends: vec![0, 1, 1, 0]}),
            ('J', Pipe{ends: vec![1, 1, 0, 0]}),
            ('7', Pipe{ends: vec![1, 0, 0, 1]}),
            ('F', Pipe{ends: vec![0, 0, 1, 1]}),
            ('.', Pipe{ends: vec![0, 0, 0, 0]}),
        ])
    }

    fn connected(&self, o: &Pipe, dir: usize) -> bool {
        self.ends[dir] == 1 && o.ends[(dir + 2)%4] == 1
    }
}

struct Maze {
    data: Vec<Vec<char>>,
    start: Pos,
    size: Pos,
    start_char: char,
    pipes: HashMap<char, Pipe>,
}

impl Maze {
    fn new(data: &str) -> Self {
        let lines = data
            .split('\n')
            .filter(|s| s.len() > 0)
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let pipes = Pipe::create();
        let start = Self::get_start(&lines);
        let size = (lines[0].len() as I, lines.len() as I);

        let start_ends = (0..4)
            .map(|dir| {
                if let Some(next) = get_next(&start, dir, &size) {
                    let c = lines[next.1 as usize].iter().nth(next.0 as usize).unwrap();
                    let p = pipes.get(&c).unwrap();
                    p.ends[(dir + 2) % 4]
                } else {
                    0
                }
            })
            .collect::<Vec<I>>();

        let start_char = pipes
            .iter()
            .filter(|(_, v)| v.ends == start_ends)
            .map(|(&k, _)| k)
            .collect::<Vec<char>>()[0];

        Maze {
            data: lines,
            size, start, start_char, pipes,
        }
    }

    fn get_start(maze: &Vec<Vec<char>>) -> Pos {
        for (y, l) in maze.iter().enumerate() {
            for (x, &c) in l.iter().enumerate() {
                if c == 'S' {
                    return (x as I, y as I);
                }
            }
        }
        panic!("can not fing start position");
    }

    fn get_pipe(&self, p: &Pos) -> &Pipe {
        if *p == self.start {
            self.pipes.get(&self.start_char).unwrap()
        } else {
            let c = self.data[p.1 as usize][p.0 as usize];
            self.pipes.get(&c).unwrap()
        }
    }

    fn collect_cycle(&self) -> Vec<Pos> {
        let mut path = vec![self.start.clone()];

        let mut cur = self.start.clone();
        loop {
            let cur_p = self.get_pipe(&cur);
            let next = (0..4)
                .filter_map(|dir| {
                    if let Some(next) = get_next(&cur, dir, &self.size) {
                        let p = self.get_pipe(&next);
                        if cur_p.connected(p, dir) {
                            if path.len() == 1 || (path.len() > 1 && path[path.len()-2] != next) {
                                return Some(next);
                            }
                        }
                    }
                    None
                })
                .next().unwrap();

            path.push(next.clone());
            cur = next;
            if cur == self.start {
                return path;
            }
        }
    }

    fn cast_ray(&self, p: &Pos, path: &HashSet<Pos>) -> usize {
        let mut count = 0;
        for x in p.0+1..self.size.0 {
            let np = (x, p.1);
            let c = if np == self.start {
                self.start_char
            } else {
                if path.contains(&np) {
                    self.data[p.1 as usize][x as usize]
                } else {
                    '.'
                }
            };

            match c {
                '|'| 'J' | 'L' => count += 1,
                _ => {}
            }
        }
        count
    }
}


fn solve1(maze: &Maze) -> usize {
    let p = maze.collect_cycle();
    (p.len() - 1) / 2
}

fn solve2(maze: &Maze) -> usize {
    let path = maze.collect_cycle();
    let path_points = path.iter().map(|(x, y)| (x.to_owned(), y.to_owned())).collect::<HashSet<Pos>>();

    // let mut inner = HashSet::<Pos>::new();
    let mut res = 0;
    for (y, l) in maze.data.iter().enumerate() {
        for (x, _) in l.iter().enumerate() {
            let cur = (x as I, y as I);
            if !path_points.contains(&cur) && maze.cast_ray(&cur, &path_points) % 2 == 1 {
                // inner.insert(cur);
                res += 1;
            }
        }
    }
    // print(maze, &inner, &p);
    res
}

#[allow(dead_code)]
fn print(m: &Maze, inner: &HashSet<Pos>, path: &HashSet<Pos>) {
    for (y, l) in m.data.iter().enumerate() {
        for (x, &c) in l.iter().enumerate() {
            let p = (x as I, y as I);
            let s = match c {
                '|' => "┃",
                '-' => "━",
                'F' => "┎",
                'L' => "┗",
                '7' => "┓",
                'J' => "┚",
                _ => " ",
            };
            if path.contains(&p) {
                print!("{s}");
            } else {
                if inner.contains(&p) {
                    print!("I");
                } else {
                    print!(" ");
                }
            }

        }
        println!();
    }
}

pub fn run(data: &str, check: bool) -> Result {
    let maze = Maze::new(data);

    let ans1 = solve1(&maze);
    println!("Part1: {}", ans1);

    let ans2 = solve2(&maze);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 6649 && ans2 == 601) {
        Ok(())
    } else {
        Err(())
    }
}
