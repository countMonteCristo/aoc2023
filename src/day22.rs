use std::collections::{HashMap, HashSet};

use super::utils::Result;

type U = u32;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Pos {
    x: U,
    y: U,
    z: U,
}

impl Pos {
    fn new(mut v: impl Iterator<Item=U>) -> Self {
        Self{x: v.next().unwrap(),y: v.next().unwrap(), z: v.next().unwrap()}
    }
}

#[derive(Debug)]
struct Brick {
    p1: Pos,
    p2: Pos,
    above: HashSet<usize>,
    below: HashSet<usize>,
}

impl Brick {
    fn new(s: &str) -> Self {
        let mut parts = s.split("~");
        let first = parts.next().unwrap().split(",").map(|t| t.parse::<U>().unwrap());
        let second = parts.next().unwrap().split(",").map(|t| t.parse::<U>().unwrap());

        Self{p1: Pos::new(first), p2: Pos::new(second), above: HashSet::new(), below: HashSet::new()}
    }

    fn prepare(&mut self, h: &HashMap<Pos, usize>) {
        self.above = self.above_ids(h);
        self.below = self.below_ids(h);
    }

    fn drop(&mut self, h: &mut HashMap<Pos, usize>, id: &usize) {
        while self.p1.z > 1 {
            let mut stop = false;

            for x in self.p1.x..=self.p2.x {
                for y in self.p1.y..=self.p2.y {
                    let p = Pos{x,y,z: self.p1.z-1};
                    if h.contains_key(&p) {
                        stop = true;
                        break;
                    }
                }
                if stop { break; }
            }
            if stop { break; }

            self.p1.z -= 1;
            self.p2.z -= 1;
        }

        for x in self.p1.x..=self.p2.x {
            for y in self.p1.y..=self.p2.y {
                for z in self.p1.z..=self.p2.z {
                    let p = Pos{x,y,z};
                    h.insert(p, id.clone());
                }
            }
        }
    }

    fn above_ids(&self, h: &HashMap<Pos, usize>) -> HashSet<usize> {
        let mut res = HashSet::new();

        for x in self.p1.x..=self.p2.x {
            for y in self.p1.y..=self.p2.y {
                let p = Pos{x, y, z: self.p2.z+1};
                if h.contains_key(&p) {
                    let id = h.get(&p).unwrap().clone();
                    res.insert(id);
                }
            }
        }

        res
    }
    fn below_ids(&self, h: &HashMap<Pos, usize>) -> HashSet<usize> {
        let mut res = HashSet::new();

        for x in self.p1.x..=self.p2.x {
            for y in self.p1.y..=self.p2.y {
                let p = Pos{x, y, z: self.p1.z-1};
                if h.contains_key(&p) {
                    let id = h.get(&p).unwrap().clone();
                    res.insert(id);
                }
            }
        }

        res
    }

    fn can_be_desintegrated(&self, bricks: &HashMap<usize, Brick>, h: &HashMap<Pos, usize>) -> bool {
        self.above
            .iter()
            .all(|id| {
                let b = bricks.get(id).unwrap();
                b.below_ids(h).len() > 1
            })
    }
}

fn prepare(lines: &Vec<&str>) -> (HashMap<usize, Brick>, Vec<usize>, HashMap::<Pos, usize>) {
    let mut bricks = lines
        .iter().enumerate()
        .map(|(id, &s)| (id, Brick::new(s/* , id */)))
        .collect::<HashMap<usize, Brick>>();

    let mut sorted_ids  = bricks
        .iter()
        .map(|(&id, _)| id)
        .collect::<Vec<usize>>();
    sorted_ids.sort_by(|i1, i2| {
        let b1 = bricks.get(i1).unwrap();
        let b2 = bricks.get(i2).unwrap();
        b1.p1.z.min(b1.p2.z).cmp(&b2.p1.z.min(b2.p2.z))
    });

    let mut h = HashMap::<Pos, usize>::new();

    sorted_ids
        .iter()
        .for_each(|id| bricks.get_mut(id).unwrap().drop(&mut h, id));

    sorted_ids
        .iter()
        .for_each(|id| bricks.get_mut(id).unwrap().prepare(&mut h));

    (bricks, sorted_ids, h)
}

fn solve1(bricks: &HashMap<usize, Brick>, sorted_ids: &Vec<usize>, h: &HashMap<Pos, usize>) -> usize {
    sorted_ids
        .iter()
        .map(|id| bricks.get(id).unwrap().can_be_desintegrated(&bricks, &h))
        .filter(|&f| f)
        .count()
}

fn count_falls(bricks: &HashMap<usize, Brick>, id: &usize) -> usize {
    let mut falls = HashSet::<usize>::from_iter([id.clone()]);

    let mut q = vec![id.clone()];
    while !q.is_empty() {
        let i = q.pop().unwrap();
        for above_id in bricks.get(&i).unwrap().above.iter() {
            if falls.is_superset(&bricks.get(&above_id).unwrap().below) {
                q.push(above_id.clone());
                falls.insert(above_id.clone());
            }
        }
    }
    falls.len() - 1
}

fn solve2(bricks: &HashMap<usize, Brick>, sorted_ids: &Vec<usize>) -> usize {
    sorted_ids
        .iter()
        .map(|id| count_falls(bricks, id))
        .sum()
}

pub fn run(data: &str, check: bool) -> Result {
    let lines = data.split('\n').collect();
    let (bricks, sorted_ids, h) = prepare(&lines);

    let ans1 = solve1(&bricks, &sorted_ids, &h);
    println!("Part1: {}", ans1);

    let ans2 = solve2(&bricks, &sorted_ids);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 507 && ans2 == 51733) {
        Ok(())
    } else {
        Err(())
    }
}
