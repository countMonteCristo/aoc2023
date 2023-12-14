use super::utils::Result;

type Pos = (usize, usize);

struct Universe {
    rows: Vec<usize>,
    cols: Vec<usize>,
    galaxies: Vec<Pos>,
}

impl Universe {
    fn new(lines: &Vec<&str>) -> Self {
        let rows = lines
            .iter().enumerate()
            .filter(|(_, &s)| !s.contains('#'))
            .map(|(i, _)| i)
            .collect();

        let cols = (0..lines[0].len())
            .map(|i: usize| {
                (i, lines
                    .iter()
                    .map(|&s| s.chars().nth(i).unwrap())
                    .collect::<String>())
            })
            .filter(|(_, s)| !s.contains('#'))
            .map(|t| t.0)
            .collect();

        let mut galaxies = Vec::<Pos>::new();
        for (y, &l) in lines.iter().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c == '#' {
                    galaxies.push((x, y));
                }
            }
        }

        Universe{rows, cols, galaxies}
    }

    fn get_dist(&self, g1: &Pos, g2: &Pos, m: usize) -> usize {
        let xmin = g1.0.min(g2.0);
        let xmax = g1.0.max(g2.0);
        let ymin = g1.1.min(g2.1);
        let ymax = g1.1.max(g2.1);

        let dx = (xmin..xmax)
            .filter(|i| self.cols.contains(i)).count();
        let dy = (ymin..ymax)
            .filter(|i| self.rows.contains(i)).count();

        xmax - xmin + ymax - ymin + (dx + dy)*(m-1)
    }

}


fn solve(universe: &Universe, m: usize) -> usize {
    universe.galaxies
        .iter().enumerate()
        .map(|(i, g1)| {
            universe.galaxies.iter().skip(i+1)
                .map(|g2| universe.get_dist(g1, g2, m))
        })
        .flatten()
        .sum()
}

pub fn run(data: &str, check: bool) -> Result {
    let s = data.to_string();
    let lines: Vec<&str> = s.split('\n').filter(|&s| !s.is_empty()).collect();
    let universe = Universe::new(&lines);

    let ans1 = solve(&universe, 2);
    println!("Part1: {}", ans1);

    let ans2 = solve(&universe, 1000000);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 9684228 && ans2 == 483844716556) {
        Ok(())
    } else {
        Err(())
    }
}
