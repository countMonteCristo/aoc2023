use std::collections::HashMap;

use super::utils::Result;

struct Xmas {
    d: HashMap<char, u64>,
}
impl Xmas {
    fn new(s: &str) -> Self {
        let b: &[_] = &['{', '}'];
        let d = s
            .trim_matches(b)
            .split(",")
            .map(|p| {
                let mut q = p.split("=");
                let c = q.next().unwrap().chars().nth(0).unwrap();
                let v = q.next().unwrap().parse::<u64>().unwrap();
                (c, v)
            })
            .collect::<HashMap<_, _>>();

        Self{d}
    }
}

#[derive(Clone)]
struct Range {
    l: u64,
    r: u64,
}
impl Range {
    fn new(l: u64, r: u64) -> Option<Self> {
        if l <= r { Some(Self{l, r}) } else { None }
    }
    fn len(&self) -> u64 {
        self.r - self.l + 1
    }
    fn split(&self, r: &Rule) -> (Option<Self>, Option<Self>) {
        match r.op {
            '>' => (Self::new(r.v+1, self.r), Self::new(self.l, r.v)),
            '<' => (Self::new(self.l, r.v-1), Self::new(r.v, self.r)),
            _   => (Self::new(self.l, self.r),   None),
        }
    }
}

type XmasRanges = HashMap<char, Range>;

struct Rule {
    c: char,
    op: char,
    v: u64,
    to: String,
}
impl Rule {
    fn new(s: &str) -> Self {
        if s.contains(':') {
            let mut parts = s.split(':');
            let r = parts.next().unwrap();

            let c= r.chars().nth(0).unwrap();
            let op = r.chars().nth(1).unwrap();
            let v = r[2..].parse().unwrap();
            let to = parts.next().unwrap().to_string();

            Self{c, op, v, to}
        } else {
            Self{to: s.to_string(), c: 'x', op: '\0', v: 0}
        }
    }
    fn apply(&self, p: &Xmas) -> Option<&String> {
        let v = *p.d.get(&self.c).unwrap();
        let ok = match self.op {
            '>' => v > self.v,
            '<' => v < self.v,
            _ => true,
        };
        if ok { Some(&self.to) } else { None }
    }

    fn from_str(s: &str) -> (String, Vec<Self>) {
        let mut parts = s.trim_matches('}').split('{');
        let name = parts.next().unwrap().to_string();
        let rules_str = parts.next().unwrap();

        let rules = rules_str
            .split(",")
            .map(|s| Rule::new(s))
            .collect();

        (name, rules)
    }

    fn collect_ranges(i: impl Iterator<Item=(char, Option<Range>)> + Clone) -> Option<XmasRanges> {
        if i.clone().any(|(_, r)| r.is_none()) {
            None
        } else {
            Some(i.map(|(c, r)| (c, r.unwrap())).collect())
        }
    }

    fn split_ranges(&self, pr: &XmasRanges) -> (Option<XmasRanges>, Option<XmasRanges>) {
        if self.op == '\0' {
            return (Some(pr.clone()), None);
        }
        let iter = pr
            .iter()
            .map(|(&k, r)| {
                if k == self.c { (k, r.split(self)) } else { (k, (Some(r.clone()), Some(r.clone()))) }
            });
        (
            Self::collect_ranges(iter.clone().map(|(k, (r1, _))| (k, r1))),
            Self::collect_ranges(iter.clone().map(|(k, (_, r2))| (k, r2)))
        )
    }
}

type Workflows = HashMap<String, Vec<Rule>>;

fn apply_rules(mut ranges: XmasRanges, rules: &Vec<Rule>) -> Vec<(String, XmasRanges)> {
    let mut res = Vec::new();

    for rule in rules.iter() {
        let (matched, tail) = rule.split_ranges(&ranges);
        if let Some(matched) = matched {
            res.push((rule.to.clone(), matched));
        }
        if let Some(tail) = tail {
            ranges = tail;
        } else {
            break;
        }
    }
    res
}


fn solve1(workflows: &Workflows, items: &Vec<Xmas>) -> u64 {
    let mut s = 0;
    for xmas in items.iter() {
        let mut cur = &"in".to_string();
        loop {
            let rules = workflows.get(cur).unwrap();
            for r in rules.iter() {
                if let Some(next) = r.apply(xmas) {
                    cur = next;
                    break;
                }
            }
            if cur.as_str() == "A" || cur.as_str() == "R" { break; }
        }

        if cur.as_str() == "A" {
            s += xmas.d.values().sum::<u64>();
        }
    }

    s
}

fn solve2(workflows: &Workflows, min: u64, max: u64) -> u64 {
    let start = HashMap::from([
        ('x', Range{l: min, r: max}),
        ('m', Range{l: min, r: max}),
        ('a', Range{l: min, r: max}),
        ('s', Range{l: min, r: max}),
    ]);

    let mut pool = vec![("in".to_string(), start)];
    let mut res = 0;
    while !pool.is_empty() {
        let (name, pr) = pool.pop().unwrap();

        if name.as_str() == "A" {
            res += pr.values().map(|r| r.len()).product::<u64>();
            continue;
        }
        if name.as_str() == "R" { continue; }

        pool.extend(apply_rules(pr, &workflows.get(&name).unwrap()));
    }
    res
}


fn parse(data: &str) -> (Workflows, Vec<Xmas>) {
    let mut p = data.split("\n\n");

    let workflows = p
        .next().unwrap()
        .split('\n')
        .map(|s| Rule::from_str(s))
        .collect();

    let items = p
        .next().unwrap()
        .split('\n')
        .map(|s| Xmas::new(s))
        .collect();

    (workflows, items)
}

pub fn run(data: &str, check: bool) -> Result {
    let (workflows, items) = parse(data);

    let ans1 = solve1(&workflows, &items);
    println!("Part1: {}", ans1);

    let ans2 = solve2(&workflows, 1, 4000);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 377025 && ans2 == 135506683246673) {
        Ok(())
    } else {
        Err(())
    }
}
