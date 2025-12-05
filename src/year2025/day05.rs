//! # Cafeteria

use crate::utils::iter::{split_by_empty_lines, vec_to_tuple_2};
use crate::utils::parse::ParseOps;

pub struct Input {
    fresh: Vec<(usize, usize)>,
    available: Vec<usize>,
}

pub fn parse(input: &str) -> Input {
    let (ranges, available) = vec_to_tuple_2(split_by_empty_lines(input.trim()));

    let fresh = ranges
        .lines()
        .map(|line| line.trim())
        .map(|range| vec_to_tuple_2(range.iter_unsigned().collect()))
        .collect();

    let available = available.iter_unsigned().collect();

    Input { fresh, available }
}

pub fn part1(input: &Input) -> usize {
    input
        .available
        .iter()
        .filter(|&ingredient| {
            input
                .fresh
                .iter()
                .any(|range| (range.0..=range.1).contains(ingredient))
        })
        .count()
}

pub fn part2(input: &Input) -> usize {
    let mut intervals: Vec<(usize, usize)> = input.fresh.clone();
    intervals.sort_by_key(|r| r.0);

    let mut merged: Vec<(usize, usize)> = Vec::new();
    for interval in intervals {
        if let Some(last) = merged.last_mut() {
            if last.1 >= interval.0 {
                last.1 = last.1.max(interval.1);
            } else {
                merged.push(interval);
            }
        } else {
            merged.push(interval);
        }
    }

    merged.iter().map(|r| r.1 - r.0 + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
    3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32";

    #[test]
    fn part1_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn part2_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part2(&input), 14);
    }
}
