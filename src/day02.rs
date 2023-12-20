use super::utils::Result;

struct CubeSet {
    v: Vec<u64>
}

impl CubeSet {
    fn new(s: &str) -> Self {
        let mut v: Vec<u64> = vec![0; 3];
        for str_set in s.split(", ") {
            let desc = str_set.split(' ').collect::<Vec<_>>();
            let count: u64 = desc[0].parse().expect("Can not parse cube count");
            let color = desc[1];
            let id = match color {
                "red" =>   0,
                "green" => 1,
                "blue" =>  2,
                _ => unreachable!()
            };
            v[id] = count;
        }

        Self { v }
    }
}

struct Game {
    id: u64,
    sets: Vec<CubeSet>,
}

impl Game {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(": ").collect();

        let id: u64 = parts[0].split(' ').last().unwrap().parse().expect("Failed to parse game id");
        let sets: Vec<CubeSet> = parts[parts.len() - 1]
            .split("; ")
            .map(CubeSet::new)
            .collect();

        Self{ id, sets }
    }

    fn get_power_set(&self, _: &CubeSet) -> u64 {
        (0..3)
            .map(|id| self.max_per_game(id))
            .product()
    }

    fn max_per_game(&self, id: usize) -> u64 {
        self.sets
            .iter()
            .map(|s| s.v[id])
            .max()
            .expect("")
    }

    fn is_game_possible(&self, s: &CubeSet) -> u64 {
        if (0..3).all(|id| self.max_per_game(id) <= s.v[id]) { self.id } else { 0 }
    }
}

fn solve(games: &Vec<Game>, s: &CubeSet, f: fn(&Game, &CubeSet)->u64) -> u64 {
    games.iter().map(|game| f(game, s)).sum()
}

pub fn run(data: &str, check: bool) -> Result {
    let lines = data.split('\n').collect::<Vec<_>>();
    let games = lines
        .iter()
        .map(|&l| Game::new(l))
        .collect();

    let ans1 = solve(&games, &CubeSet{v: vec![12, 13, 14]}, Game::is_game_possible);
    println!("Part1: {}", ans1);

    let ans2 = solve(&games, &CubeSet{v: vec![]}, Game::get_power_set);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 2563 && ans2 == 70768) {
        Ok(())
    } else {
        Err(())
    }
}
