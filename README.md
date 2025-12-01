# AoC solutions

My solutions to [Advent of Code](https://adventofcode.com/) in Rust

## Thanks to

Project structure is heavily inspired by [`advent-of-code-rust`](https://github.com/maneatingape/advent-of-code-rust).

## Getting started

1. Install Rust with rustup if you haven't already
2. Clone repo
3. Place input files in `input/yearYYYY/dayDD.txt` including leading zeroes (e.g. `input/year2025/day01.txt` and `input/year2025/day12.txt`)
4. Run
    1. Run everything: `cargo run`
    2. Run specific year: `cargo run year2025` or `cargo run 2025`
    3. Run specific day: `cargo run year2025::day01` or `cargo run year2025 day01` or `cargo run 2025 01` or `cargo run 2025::01` and so on
5. Run test on examples
    1. Test everything: `cargo test`
    2. Test specific year: `cargo test year2025`
    3. Test specific day: `cargo test year2025::day01`
