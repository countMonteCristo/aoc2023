use super::utils::Result;


fn get_dt(t: f64, s: f64) -> u64 {
    let d = t*t - 4_f64*s;
    let t1 = ((t - d.sqrt()) / 2.0_f64).floor() as u64 + 1;
    let t2 = ((t + d.sqrt()) / 2.0_f64).ceil() as u64 - 1;
    t2 - t1 + 1
}

fn solve(parts: Vec<Vec<u64>>) -> u64 {
    parts[0]
        .iter()
        .zip(parts[1].iter())
        .map(|(&t, &s)| get_dt(t as f64, s as f64))
        .product()
}

fn solve1(lines: &Vec<&str>) -> u64 {
    let parts = lines
        .iter()
        .map(|&l| {
            l
                .split(' ')
                .filter(|&x| !x.is_empty())
                .skip(1)
                .map(|v| v.parse().unwrap())
                .collect::<Vec<_>>()}
        )
        .collect();

    solve(parts)
}

fn solve2(lines: &Vec<&str>) -> u64 {
    let parts = lines
        .iter()
        .map(|&l| {
            vec![l
                .replace(" ", "")
                .split(':')
                .last().unwrap()
                .parse().unwrap()
            ]
        })
        .collect();

    solve(parts)
}

pub fn run(data: &str, check: bool) -> Result {
    let lines = data.split('\n').collect();

    let ans1 = solve1(&lines);
    println!("Part1: {}", ans1);

    let ans2 = solve2(&lines);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 1155175 && ans2 == 35961505) {
        Ok(())
    } else {
        Err(())
    }
}
