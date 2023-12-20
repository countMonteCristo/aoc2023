use super::utils::Result;


#[derive(Debug, Clone)]
struct Range {
    start: u64,
    size: u64,
}


impl Range {
    fn intersect(&self, r: &Range) -> Option<Range> {
        if self.start >= r.start + r.size || r.start >= self.start + self.size {
            return None;
        }

        let start = self.start.max(r.start);
        let end = (self.start + self.size - 1).min(r.start + r.size - 1);
        Some(Range{start, size: end-start+1})
    }

    fn map_rng(&self, map: &Map) -> Vec<Range> {
        let mut res = Vec::<Range>::new();
        let mut cr = self.clone();

        for rm in map.iter() {
            if let Some(i) = cr.intersect(&rm.from) {
                if i.start != self.start {
                    res.push(Range{start: cr.start, size: i.start-cr.start});
                }
                res.push(rm.map(&i));
                let s = cr.start.clone();
                cr.start = i.start + i.size;
                cr.size = cr.size - (cr.start - s);
            }
        }
        if cr.size > 0 {
            res.push(cr);
        }

        res
    }

}


#[derive(Debug, Clone)]
struct RangeMap {
    from: Range,
    to: Range,
}

impl RangeMap {
    fn new(v: Vec<u64>) -> Self {
        Self { from: Range{start: v[1], size: v[2]}, to: Range{start: v[0], size: v[2]} }
    }

    fn map(&self, r: &Range) -> Range {
        if r.start >= self.from.start && r.start + r.size <= self.from.start + self.from.size {
            Range{
                start: self.to.start + r.start - self.from.start,
                size: (self.from.start + self.from.size).min(r.start + r.size) - r.start
            }
        } else {
            panic!("map range from different scope!");
        }
    }
}

type Map = Vec<RangeMap>;
type Alm = Vec<Map>;

struct Almanac {
    maps: Alm,
    raw_seeds: Vec<u64>,
}

impl Almanac {
    fn new(lines: &Vec<&str>) -> Self {
        let mut maps = Alm::new();
        let mut raw_seeds = Vec::<u64>::new();

        let mut ranges = Vec::<RangeMap>::new();

        for &l in lines.iter() {
            if l.len() == 0 {
                if !ranges.is_empty() {
                    let mut copy = ranges.clone();
                    copy.sort_by(|a, b| {
                        a.from.start.cmp(&b.from.start)
                    });
                    maps.push(copy);
                    ranges.clear();
                }
                continue;
            }
            if l.starts_with("seeds:") {
                raw_seeds = l
                    .split(": ")
                    .last().unwrap()
                    .split(' ')
                    .map(|x| x.parse().unwrap())
                    .collect();
                continue;
            }
            if l.ends_with("map:") {
                continue;
            }

            ranges.push(
                RangeMap::new(
                    l
                    .split(' ')
                    .map(|x| x.parse().unwrap())
                    .collect()
                )
            );
        }
        maps.push(ranges.clone());

        Almanac{maps, raw_seeds}
    }

}

fn solve(seeds: &Vec<Range>, a: &Almanac) -> u64 {
    let mut cur = seeds.clone();
    for m in a.maps.iter() {
        cur = cur
            .iter()
            .map(|r| r.map_rng(m))
            .flatten()
            .collect::<Vec<_>>();
    }

    cur.iter().map(|r| r.start).min().unwrap()
}

fn solve1(a: &Almanac) -> u64 {
    let seeds = a.raw_seeds
        .iter()
        .map(|&v| Range{start: v, size: 1})
        .collect();

    solve(&seeds, a)
}

fn solve2(a: &Almanac) -> u64 {
    let seeds = a.raw_seeds
        .chunks(2)
        .map(|v| Range{start: v[0], size: v[1]})
        .collect();

    solve(&seeds, a)
}


pub fn run(data: &str, check: bool) -> Result {
    let lines = data.split('\n').collect();
    let almanac = Almanac::new(&lines);

    let ans1 = solve1(&almanac);
    println!("Part1: {}", ans1);

    let ans2 = solve2(&almanac);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 462648396 && ans2 == 2520479) {
        Ok(())
    } else {
        Err(())
    }
}
