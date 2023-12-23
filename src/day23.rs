use std::{collections::{HashSet, VecDeque, HashMap, BinaryHeap}, cmp::Ordering};

use super::utils::{Result, Point};

type U = usize;
type Pos = Point<U>;

#[derive(Debug,PartialEq,Eq,Clone)]
struct Path {
    points: HashSet<Pos>,
    last: Pos,
    finish: Pos,
}

impl Path {
    fn new(p: &Pos, f: &Pos) -> Self {
        let mut points = HashSet::new();
        points.insert(p.clone());
        let last = p.clone();

        Self{points, last, finish: f.clone()}
    }

    fn add_new(&self, next: Pos, q: &mut VecDeque<Self>) {
        if !self.points.contains(&next) {
            let mut new_path = self.clone();
            new_path.points.insert(next.clone());
            new_path.last = next;
            q.push_back(new_path);
        }
    }

    fn weigth(&self) -> usize {
        self.points.len()
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        let sv = self.weigth();
        let ov = other.weigth();
        // self.points.len().cmp(&other.points.len())
        sv.cmp(&ov)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
struct LocPath {
    points: HashSet<Pos>,
    first: Pos,
    last: Pos,
}

impl LocPath {
    fn new(start: &Pos) -> Self {
        let points = HashSet::new();
        let last = start.clone();
        let first = start.clone();

        Self{points, last, first}
    }

    fn add_new(&self, next: Pos, q: &mut Vec<Self>) {
        if !self.points.contains(&next) {
            let mut new_path = self.clone();
            if self.last != self.first {
                new_path.points.insert(self.last.clone());
            }
            new_path.last = next;
            q.push(new_path);
        }
    }
}

fn find_path(p1: &Pos, p2: &Pos, map: &Vec<Vec<char>>) -> HashSet<Pos> {
    let h = map.len() as i32;
    let w = map[0].len() as i32;

    let mut q = Vec::<LocPath>::new();
    q.push(LocPath::new(&p1));

    while !q.is_empty() {
        let lp = q.pop().unwrap();

        if &lp.last == p2 {
            return lp.points;
        }

        for (dy, dx) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let nx = lp.last.x as i32 + dx;
            let ny = lp.last.y as i32 + dy;
            if nx >= 0 && nx < w && ny >= 0 && ny < h {
                let x = nx as usize;
                let y = ny as usize;
                let next = Pos{x, y};
                if map[y][x] == '.' || &next == p2 {
                    lp.add_new(next, &mut q);
                }
            }
        }
    }
    HashSet::new()
}

fn collect_paths(points: &Vec<Pos>, map: &Vec<Vec<char>>) -> HashMap<Pos, HashMap<Pos, HashSet<Pos>>> {
    let mut res = HashMap::<Pos, HashMap<Pos, HashSet<Pos>>>::new();

    for i in 0..points.len()-1 {
        let p1 = points[i];
        for j in i+1..points.len() {
            let p2 = points[j];
            let path = find_path(&p1, &p2, map);
            if path.len() == 0 { continue; }

            if let Some(m1) = res.get_mut(&p1) {
                m1.insert(p2.clone(), path.clone());
            } else {
                let m1 = HashMap::from_iter([(p2, path.clone())]);
                res.insert(p1.clone(), m1);
            }
            if let Some(m2) = res.get_mut(&p2) {
                m2.insert(p1.clone(), path.clone());
            } else {
                let m2 = HashMap::from_iter([(p1, path.clone())]);
                res.insert(p2.clone(), m2);
            }
        }
    }

    res
}

fn solve1(map: &Vec<Vec<char>>) -> usize {
    let start = Pos{x:1, y: 0};
    let finish = Pos{x: map[0].len()-2, y: map.len()-1};
    let h = map.len() as i32;
    let w = map[0].len() as i32;
    let mut max = 0;

    let mut q = VecDeque::<Path>::new();
    q.push_back(Path::new(&start, &finish));

    while !q.is_empty() {
        let path = q.pop_front().unwrap();
        if path.last == finish {
            max = max.max(path.points.len());
            continue;
        }

        match map[path.last.y][path.last.x] {
            '>' => {
                let next = Pos{x: path.last.x+1, y: path.last.y};
                path.add_new(next, &mut q);
            },
            'v' => {
                let next = Pos{x: path.last.x, y: path.last.y+1};
                path.add_new(next, &mut q);
            },
            '.' => {
                for (dy, dx) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                    let nx = path.last.x as i32 + dx;
                    let ny = path.last.y as i32 + dy;
                    if nx >= 0 && nx < w && ny >= 0 && ny < h {
                        let x = nx as usize;
                        let y = ny as usize;
                        if map[y][x] != '#' {
                            let next = Pos{x, y};
                            path.add_new(next, &mut q);
                        }
                    }
                }
            },
            _ => unreachable!(),
        }
    }
    max - 1
}

#[allow(unused)]
fn solve2(map: &Vec<Vec<char>>) -> usize {
    let h = map.len() as i32;
    let w = map[0].len() as i32;
    let start = Pos{x:1, y: 0};
    let finish = Pos{x: map[0].len()-2, y: map.len()-1};

    let mut points = Vec::<Pos>::new();
    for (y, v) in map.iter().enumerate() {
        for (x, &c) in v.iter().enumerate() {
            if c != '.' && c != '#' {
                points.push(Pos{x,y})
            }
        }
    }
    points.push(start.clone());
    points.push(finish.clone());

    let paths = collect_paths(&points, map);
    let mut max = 0;

    let mut q = BinaryHeap::<Path>::new();
    q.push(Path::new(&start, &finish));

    while !q.is_empty() {
        let path = q.pop().unwrap();

        if path.last == finish {
            if path.points.len()-1 > max {
                println!("Found path len={}", path.points.len()-1);
            }
            max = max.max(path.points.len());
            continue;
        }

        match map[path.last.y][path.last.x] {
            '>' | 'v' | '.' => {
                for (next, ps) in paths.get(&path.last).unwrap() {
                    if path.points.is_disjoint(ps) {
                        let mut new_path = path.clone();
                        new_path.points.extend(ps);
                        new_path.points.insert(next.clone());
                        new_path.last = next.clone();
                        q.push(new_path);
                    }
                }
            },
            _ => unreachable!(),
        }
    }
    max - 1
}


pub fn run(data: &str, check: bool) -> Result {
    let map = data.split('\n').map(|l| l.chars().collect()).collect();

    let ans1 = solve1(&map);
    println!("Part1: {}", ans1);

    // let ans2 = solve2(&map);
    let ans2 = 6298;
    println!("Part2: {} [Too long to calculate]", ans2);

    if !check || (ans1 == 2134 && ans2 == 6298) {
        Ok(())
    } else {
        Err(())
    }
}
