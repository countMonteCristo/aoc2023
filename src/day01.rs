use super::utils::Result;

fn get_calibration_value(line: &str, digits: &Vec<(&str, i32)>) -> i32 {
    let first = digits
        .iter()
        .map(|&(pat, val)| (line.find(pat).or(Some(usize::MAX)).unwrap(), val))
        .reduce(|(min_index, d), (index, c)| {
            if index < min_index {
                (index, c)
            } else {
                (min_index, d)
            }
        })
        .map(|(_, d)| d).expect("ERROR!");

    let last = digits
        .iter()
        .map(|&(p, v)| (line.rfind(p).map(|x| x as i32).or(Some(-1)).unwrap(), v))
        .reduce(|(max_index, d), (index, c)| {
            if index > max_index {
                (index, c)
            } else {
                (max_index, d)
            }
        })
        .map(|(_, d)| d).expect("ERROR!");

    first*10 + last
}

fn solve(lines: &Vec<&str>, digits: Vec<(&str, i32)>) -> i32 {
    lines
        .iter()
        .map(|&s| get_calibration_value(s, &digits))
        .sum()
}

pub fn run(data: &str, check: bool) -> Result {
    let lines = data.split('\n').collect();

    let ans1 = solve(&lines, vec![
        ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9), ("0", 0),
    ]);
    println!("Part1: {}", ans1);

    let ans2 = solve(&lines, vec![
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
        ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9), ("0", 0),
    ]);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 54927 && ans2 == 54581) {
        Ok(())
    } else {
        Err(())
    }
}
