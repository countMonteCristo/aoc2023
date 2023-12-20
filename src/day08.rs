use std::collections::HashMap;

use super::utils::{Result, lcm};

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    l: &'a str,
    r: &'a str,
}

impl<'a> Node<'a> {
    fn new(s: &'a str) -> Self {
        let mut parts = s.split(" = ");
        let name = parts.next().unwrap();
        let choices = parts
            .next().unwrap()
            .trim_matches(|c| c=='(' || c==')')
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
        .collect();

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
    let lines = data.split('\n').filter(|&s| !s.is_empty()).collect();

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
