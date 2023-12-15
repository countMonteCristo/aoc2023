use super::utils::Result;


const HASH_MOD: usize = 256;
const HASH_P: usize = 17;

#[derive(Debug)]
struct Lens<'a> {
    name: &'a str,
    focal: usize,
}

#[derive(Debug)]
struct Op<'a> {
    name: &'a str,
    op: char,
    val: usize,
}

impl<'a> Op<'a> {
    fn new(s: &'a str) -> Self {
        let last = s.chars().last().unwrap();
        if last.is_digit(10) {
            Op{name: &s[..s.len()-2], op: '=', val: last.to_digit(10).unwrap() as usize}
        } else {
            Op{name: &s[..s.len()-1], op: '-', val: 0}
        }
    }

    fn lens(&self) -> Lens<'a> {
        Lens{name: self.name, focal: self.val}
    }
}

type LensBox<'a> = Vec<Lens<'a>>;

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize) * HASH_P)%HASH_MOD)
}

fn solve1(data: &str) -> usize {
    data
        .split(",")
        .map(|s| hash(s))
        .sum()
}

fn solve2(data: &str) -> usize {
    let mut boxes = Vec::<LensBox>::new();
    for _ in 0..HASH_MOD {
        boxes.push(LensBox::new());
    }

    for op in data.split(",").map(|s| Op::new(s)) {
        let b = &mut boxes[hash(op.name)];

        let opt_id = b.iter().position(|l| l.name == op.name);
        match op.op {
            '-' => {
                if let Some(i) = opt_id { b.remove(i); }
            },
            '=' => {
                if let Some(i) = opt_id {
                    b[i].focal = op.val;
                } else {
                    b.push(op.lens());
                }
            }
            _ => panic!("unreachable")
        }
    }

    boxes
        .iter().enumerate()
        .map(|(bid, b)| {
            (1 + bid) * b.iter().enumerate().map(|(lid, lens)| (lid + 1) * lens.focal).sum::<usize>()
        })
        .sum()
}

pub fn run(data: &str, check: bool) -> Result {
    let ans1 = solve1(&data);
    println!("Part1: {}", ans1);

    let ans2 = solve2(&data);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 517965 && ans2 == 267372) {
        Ok(())
    } else {
        Err(())
    }
}
