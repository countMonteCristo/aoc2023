use std::collections::{HashMap, HashSet};

use super::utils::Result;

#[derive(Clone, PartialEq, Hash, Eq)]
struct Number {
    x: usize,
    y: usize,
    value: u64,
}


#[derive(Debug, PartialEq, Hash, Eq)]
struct Symbol {
    x: i32,
    y: i32,
    c: char,
}

type Symbols = HashMap<Symbol, Vec<Number>>;

fn get_symbols(lines: &Vec<&str>) -> Symbols {
    let mut gs: Symbols = HashMap::new();

    for (y, &line) in lines.iter().enumerate() {
        let mut x = 0;
        while x < line.len() {
            let c = line.chars().nth(x).unwrap();
            if c.is_digit(10) {
                let mut cur = x + 1;
                while cur < line.len() && line.chars().nth(cur).unwrap().is_digit(10) {
                    cur += 1;
                }

                let number = line[x..cur].parse().expect("Can not parse number");
                let n = Number{x, y, value: number};

                for ty in (y as i32)-1..=(y as i32)+1 {
                    if ty < 0 { continue; }
                    if ty >= lines.len() as i32 { break; }

                    let &l = lines.iter().nth(ty as usize).unwrap();
                    for tx in (x as i32)-1..=(cur as i32) {
                        if tx >= 0 && tx < l.len() as i32 {
                            let cc = l.chars().nth(tx as usize).unwrap();
                            if cc != '.' && !cc.is_digit(10) {
                                let g = Symbol{x: tx, y: ty, c: cc};
                                if let Some(v) = gs.get_mut(&g) {
                                    v.push(n.clone());
                                } else {
                                    gs.insert(g, vec![n.clone()]);
                                }
                            }
                        }
                    }
                }
                x = cur + 1;
            } else {
                x += 1;
            }
        }
    }

    return gs;
}

fn solve1(symbols: &Symbols) -> u64 {
    HashSet::<Number>::from_iter(symbols.values().flatten().cloned())
        .iter()
        .map(|n| n.value)
        .sum()
}

fn solve2(symbols: &Symbols) -> u64 {
    symbols
        .values()
        .filter(|&v| v.len() == 2)
        .map(|v| v[0].value * v[1].value)
        .sum()
}


pub fn run(data: &str, check: bool) -> Result {
    let lines = data.split('\n').collect();
    let symbols = get_symbols(&lines);

    let ans1 = solve1(&symbols);
    println!("Part1: {}", ans1);

    let ans2 = solve2(&symbols);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 528819 && ans2 == 80403602) {
        Ok(())
    } else {
        Err(())
    }
}
