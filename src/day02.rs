use super::utils::Result;

struct CubeSet {
    v: Vec<u64>
}

struct Game {
    id: u64,
    sets: Vec<CubeSet>,
}

fn solve1(lines: &Vec<&str>, s: CubeSet) -> u64 {
    lines
        .iter()
        .map(parse_game)
        .map(|game| is_game_possible(game, &s))
        .sum()
}

fn solve2(lines: &Vec<&str>) -> u64 {
    lines
        .iter()
        .map(parse_game)
        .map(get_power_set)
        .sum()
}

fn parse_cubeset(s: &str) -> CubeSet {
    let parts: Vec<&str> = s.split(", ").collect();
    let mut v: Vec<u64> = vec![0, 0, 0];
    for str_set in parts {
        let desc: Vec<&str> = str_set.split(' ').collect();
        let count: u64 = desc.first().unwrap().parse().expect("Can nnot parse cube count");
        let &color = desc.last().unwrap();
        let id = match color {
            "red" =>   0,
            "green" => 1,
            "blue" =>  2,
            _ => unreachable!()
        };
        v[id] = count;
    }

    CubeSet { v }
}

fn parse_game(line: &&str) -> Game {
    let parts: Vec<&str> = line.split(": ").collect();

    let id: u64 = parts.first().unwrap().split(' ').last().unwrap().parse().expect("Failed to parse game id");
    let sets: Vec<CubeSet> = parts
        .last()
        .unwrap()
        .split("; ")
        .map(parse_cubeset)
        .collect();

    Game { id, sets }
}

fn max_per_game(game: &Game, id: usize) -> u64 {
    game.sets
        .iter()
        .map(|s| s.v[id])
        .max()
        .expect("")
}

fn is_game_possible(game: Game, s: &CubeSet) -> u64 {
    let is_possible = (0..=2)
        .map(|id| max_per_game(&game, id) <= s.v[id])
        .reduce(|acc, x| acc && x)
        .unwrap();

    if is_possible {
        game.id
    } else {
        0
    }
}

fn get_power_set(game: Game) -> u64 {
    (0..=2)
        .map(|id| max_per_game(&game, id))
        .reduce(|acc, x| acc * x)
        .unwrap()
}

pub fn run(data: &str, check: bool) -> Result {
    let s = data.to_string();
    let lines: Vec<&str> = s.split('\n').filter(|s| s.len() > 0).collect();

    let ans1 = solve1(&lines, CubeSet{v: vec![12, 13, 14]});
    println!("Part1: {}", ans1);

    let ans2 = solve2(&lines);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 2563 && ans2 == 70768) {
        Ok(())
    } else {
        Err(())
    }
}
