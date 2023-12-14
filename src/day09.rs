use super::utils::Result;

type I = i32;

fn diff(nums: &Vec<I>) -> Vec<I> {
    nums
        .iter()
        .skip(1)
        .zip(nums.iter())
        .map(|(&a, &b)| a - b)
        .collect()
}

fn get_diffs(nums: &Vec<I>) -> Vec<Vec<I>> {
    let mut res = vec![nums.clone()];

    let mut cur = nums.clone();
    loop {
        cur = diff(&cur);
        res.push(cur.clone());
        if cur.iter().all(|&x| x == 0) { return res; }
    }
}

fn get_next(diffs: &Vec<Vec<I>>) -> I {
    let mut n = 0;
    for v in diffs.iter().rev() {
        n += v.last().unwrap();
    }
    n
}

fn get_prev(diffs: &Vec<Vec<I>>) -> I {
    let mut n = 0;
    for v in diffs.iter().rev() {
        n = v[0] - n;
    }
    n
}

fn solve(nums: &Vec<Vec<I>>, get: fn(&Vec<Vec<I>>)->I) -> I {
    nums
        .iter()
        .map(|nums| get(&get_diffs(nums)))
        .sum()
}

pub fn run(data: &str, check: bool) -> Result {
    let s = data.to_string();
    let lines: Vec<&str> = s.split('\n').filter(|&s| !s.is_empty()).collect();

    let nums = lines
        .iter()
        .map(|&s| s.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect();

    let ans1 = solve(&nums, get_next);
    println!("Part1: {}", ans1);

    let ans2 = solve(&nums, get_prev);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 2005352194 && ans2 == 1077) {
        Ok(())
    } else {
        Err(())
    }
}
