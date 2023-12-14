use super::utils::Result;


#[derive(Debug)]
struct Pattern {
    line: String,
    groups: Vec<usize>,
}

impl Pattern {
    fn new(s: &str, n: usize) -> Self {
        let parts = s.split(' ').collect::<Vec<_>>();
        let line = format!(".{}", vec![parts[0]; n].join("?").trim_end_matches('.'));
        let groups = parts[1]
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>()
            .repeat(n);

        Self{line, groups}
    }

    fn count(&self) -> usize {
        let mut dp = vec![0; self.line.len()+1];
        dp[0] = 1;
        for (i, _) in self.line.chars().take_while(|&c| c != '#').enumerate() {
            dp[i+1] = 1;
        }

        for &g in self.groups.iter() {
            let mut dp_next = vec![0; self.line.len()+1];
            let mut chunk = 0;
            for (i, c) in self.line.chars().enumerate() {
                if c != '.' {
                    chunk += 1;
                } else {
                    chunk = 0;
                }

                if c != '#' {
                    dp_next[i+1] += dp_next[i];
                }

                if chunk >= g && i >= g && self.line.chars().nth(i-g).unwrap() != '#' {
                    dp_next[i+1] += dp[i-g];
                }
            }
            dp = dp_next;
        }

        *dp.last().unwrap()
    }
}


fn solve(lines: &Vec<&str>, n: usize) -> usize {
    lines
        .iter()
        .map(|&line| Pattern::new(line, n).count())
        .sum()
}


pub fn run(data: &str, check: bool) -> Result {
    let s = data.to_string();
    let lines: Vec<&str> = s.split('\n').filter(|&s| !s.is_empty()).collect();

    let ans1 = solve(&lines, 1);
    println!("Part1: {}", ans1);

    let ans2 = solve(&lines, 5);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 7599 && ans2 == 15454556629917) {
        Ok(())
    } else {
        Err(())
    }
}
