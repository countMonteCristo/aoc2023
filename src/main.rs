use std::collections::VecDeque;
use std::env;
use std::fs;

use aoc2023::utils::Result;


fn run_day(day: u8, path: String, check: bool) {
    let input = fs::read_to_string(&path);
    let res = match input {
        Ok(data) => {
            let data = data.trim_end();
            let day_func = match day {
                1 => aoc2023::day01::run,
                // 2 => aoc2023::day02::run,
                // 3 => aoc2023::day03::run,
                // 4 => aoc2023::day04::run,
                // 5 => aoc2023::day05::run,
                // 6 => aoc2023::day06::run,
                // 7 => aoc2023::day07::run,
                // 8 => aoc2023::day08::run,
                // 9 => aoc2023::day09::run,
                // 10 => aoc2023::day10::run,
                // 11 => aoc2023::day11::run,
                // 12 => aoc2023::day12::run,
                // 13 => aoc2023::day13::run,
                // 14 => aoc2023::day14::run,
                // 15 => aoc2023::day15::run,
                // 16 => aoc2023::day16::run,
                // 17 => aoc2023::day17::run,
                // 18 => aoc2023::day18::run,
                // 19 => aoc2023::day19::run,
                // 20 => aoc2023::day20::run,
                // 21 => aoc2023::day21::run,
                // 22 => aoc2023::day22::run,
                // 23 => aoc2023::day23::run,
                // 24 => aoc2023::day24::run,
                // 25 => aoc2023::day25::run,
                _ => unreachable!(),
            };
            day_func(data, check)
        }
        Err(_) => {
            panic!("ERROR: Can not read data from {}", path);
        }
    };

    match res {
        Ok(_) => {},
        Err(_) => println!("ERROR: wrong answer at day {}", day),
    };
}

fn main() -> Result {
    let mut args: VecDeque<String> = env::args().collect();
    let _ = args.pop_front();   // skip executable filename

    let cmd = args.pop_front().expect("Command argument expected: run|check|test");
    match cmd.as_str() {
        "run" => {      // run first N days
            let count: u8 = args.pop_front()
                .expect("Expected days count but got nothing")
                .parse()
                .expect("Expected days count as a number");
            for day in 1..=count {
                println!("Day {}:", day);
                let path = format!("./data/day{:02}.txt", day);
                run_day(day, path, true);
                println!();
            }
        }
        "check" => {    // check all days or specified ones
            let days: Vec<u8> = match args.len() {
                0 => (1..=25).collect(),
                _ => args.iter().map(|d| d.parse().expect("Expected day number as a number")).collect(),
            };
            for day in days {
                println!("Day {}:", day);
                let path = format!("./data/day{:02}.txt", day);
                run_day(day, path, true);
                println!();
            }
        },
        "test" => {     // test day specified with custom input file
            let day: u8 = args.pop_front()
                .expect("Expected day number but got nothing")
                .parse()
                .expect("Expected day number as a number");
            let path = args.pop_front().expect("Expected day faile path but got nothing");
            run_day(day, path, false);
        },
        _ => panic!("Unknown command: {}. Supported commands: run|check|test", cmd)
    }


    Ok(())
}
