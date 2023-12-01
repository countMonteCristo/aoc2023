# aoc2022
[Advent of Code 2023](https://adventofcode.com/2023) solved (probably) in Rust

Usage
-----

Make sure you have Rust and Git installed, then open a terminal and run:

```bash
git clone https://github.com/countMonteCristo/aoc2023
cd aoc2023
cargo build --release
```

Run first `N=10` days:
```bash
cargo run --release run 10
```

Run all days:
```bash
cargo run --release check
```

Run specified days:
```bash
cargo run --release check 2 3 5 7 11
```

Run single day with custom file as an input:
```bash
cargo run --release test 13 path/to/custom/file.txt
```
