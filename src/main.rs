use crate::utils::parse::ParseOps;
use std::time::{Duration, Instant};

mod utils;

struct Solution {
    year: u32,
    day: u32,
    wrapper: fn(&str) -> (String, String),
}

fn main() {
    let mut args = std::env::args().flat_map(|arg| arg.iter_unsigned().collect::<Vec<u32>>());

    let (year, day) = (args.next(), args.next());

    let solutions = [year2025()];

    let (stars, duration) = solutions
        .iter()
        .flatten()
        .filter(|s| year.is_none_or(|y| y == s.year))
        .filter(|s| day.is_none_or(|d| d == s.day))
        .fold((0, Duration::ZERO), run_solution);

    println!("* {stars} stars");
    println!(" {} ms", duration.as_millis());
}

fn run_solution((stars, duration): (u32, Duration), solution: &Solution) -> (u32, Duration) {
    let Solution { year, day, wrapper } = solution;
    let path = format!("input/year{year}/day{day:02}.txt");

    if let Ok(data) = std::fs::read_to_string(&path) {
        let instant = Instant::now();
        let (part1, part2) = wrapper(&data);
        let elapsed = instant.elapsed();

        println!("{year} Day {day}");
        println!("    Part 1: {part1}");
        println!("    Part 2: {part2}");

        (stars + 2, duration + elapsed)
    } else {
        eprintln!("{year} Day {day}");
        eprintln!("    Missing input!");
        eprintln!("    Place input file in {path}");

        (stars, duration)
    }
}

macro_rules! year {
    ($year:tt $description:literal $($day:tt),*) => {
        #[doc = concat!("# ", $description)]
        pub mod $year {
            $(pub mod $day;)*
        }

        fn $year() -> Vec<Solution> {
            vec![$(
                Solution {
                    year: stringify!($year).unsigned(),
                    day: stringify!($day).unsigned(),
                    wrapper: |data: &str| {
                        use crate::$year::$day::*;

                        let input = parse(data);
                        let part1 = part1(&input).to_string();
                        let part2 = part2(&input).to_string();

                        (part1, part2)
                    }
                }
            ,)*]
        }
    }
}

year!(year2025 "Year 2025"
    day01, day02, day03
);
