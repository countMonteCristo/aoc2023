use std::collections::HashMap;

use super::utils::Result;

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    l: &'a str,
    r: &'a str,
}

impl<'a> Node<'a> {
    fn new(s: &'a str) -> Self {
        let mut parts = s
            .split(" = ");
        let name = parts.next().unwrap();
        let choices = parts
            .next().unwrap()
            .strip_prefix('(').unwrap()
            .strip_suffix(')').unwrap()
            .split(", ")
            .collect::<Vec<_>>();
        Node{name, l: choices[0], r:choices[1]}
    }
}

fn parse_input<'a>(lines: &'a Vec<&'a str>) -> (&'a str, HashMap<&'a str, Node<'a>>) {
    let nodes = lines
        .iter()
        .skip(1)
        .map(|&s| {
            let node = Node::new(s);
            (node.name, node)
        })
        .collect::<HashMap<&str, Node>>();

    (lines[0], nodes)
}

fn count(path: &str, nodes: &HashMap<&str, Node>, start: &str, is_end: fn(&str)->bool) -> u64 {
    let mut cur = start.clone();
    let mut n: u64 = 0;
    for s in path.chars().cycle() {
        let node = nodes.get(&cur).unwrap();
        if s == 'L' { cur = node.l; } else { cur = node.r; }
        n += 1;
        if is_end(cur) {
            break;
        }
    }

    n
}

fn gcd(x: u64, y: u64) -> u64 {
    let mut n1 = x.max(y);
    let mut n2 = x.min(y);

    loop {
        let r = n1 % n2;
        if r == 0 { return n2; }
        (n1, n2) = (n2, r);
    }
}

fn lcm(x: u64, y: u64) -> u64 {
    (x * y) / gcd(x, y)
}

fn solve1(path: &str, nodes: &HashMap<&str, Node>) -> u64 {
    count(path, &nodes, "AAA", |n| n == "ZZZ")
}

fn solve2(path: &str, nodes: &HashMap<&str, Node>) -> u64 {
    let starts = nodes.keys().filter(|&&n| n.ends_with('A')).map(|&s| s).collect::<Vec<_>>();

    starts
        .iter()
        .map(|&s| count(path, &nodes, s, |n| n.ends_with('Z')))
        .reduce(|acc, e| lcm(acc, e))
        .unwrap()
}

pub fn run(data: &str, check: bool) -> Result {
    let s = data.to_string();
    let lines: Vec<&str> = s.split('\n').filter(|s| s.len() > 0).collect();

    let (path, nodes) = parse_input(&lines);

    let ans1 = solve1(path, &nodes);
    println!("Part1: {}", ans1);

    let ans2 = solve2(path, &nodes);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 15517 && ans2 == 14935034899483) {
        Ok(())
    } else {
        Err(())
    }
}
