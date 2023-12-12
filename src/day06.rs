use super::utils::Result;


// x(T -x) > S
// x^2 - xT + S < 0
// D = T^2 - 4S
// x1,2 = (T +- sqrt(T^2 - 4S)) / 2

fn get_dt(t: u64, s: u64) -> u64 {
    let d = (t*t - 4*s) as f64;
    let t1 = (((t as f64) - d.sqrt()) / 2.0_f64).floor() as u64 + 1;
    let t2 = (((t as f64) + d.sqrt()) / 2.0_f64).ceil() as u64 - 1;
    t2 - t1 + 1
}

fn solve(parts: Vec<Vec<u64>>) -> u64 {
    parts[0]
        .iter()
        .zip(parts[1].iter())
        .map(|(&t, &s)| get_dt(t, s))
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
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<_>>()}
        )
        .collect::<Vec<_>>();

    solve(parts)
}

fn solve2(lines: &Vec<&str>) -> u64 {
    let parts = lines
        .iter()
        .map(|l| {
            vec![l
                .replace(" ", "")
                .split(':')
                .last().unwrap()
                .parse::<u64>().unwrap()
            ]
        })
        .collect::<Vec<_>>();

    solve(parts)
}

pub fn run(data: &str, check: bool) -> Result {
    let s = data.to_string();
    let lines: Vec<&str> = s.split('\n').filter(|s| s.len() > 0).collect();

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
