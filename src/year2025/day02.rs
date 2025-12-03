//! # Gift Shop

use std::collections::HashSet;

use crate::utils::parse::ParseOps;

type Input = (u64, u64);
type Ranges = Vec<(u64, u64)>;

pub fn parse(input: &str) -> Input {
    let ranges = parse_ranges(input);

    let part1 = sum(&ranges, &invalid_first_part());

    let part2 = sum(&ranges, &invalid_second_part());

    (part1, part2)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}

fn invalid_first_part() -> Vec<u64> {
    let mut invalid = Vec::new();

    for digits in (2..=10).step_by(2) {
        let size = digits / 2;
        let start = 10_u64.pow(size - 1);
        let end = start * 10;
        let factor = end + 1;

        invalid.extend((start..end).map(|n| factor * n));
    }

    invalid
}

fn invalid_second_part() -> Vec<u64> {
    let mut invalid = invalid_first_part();

    for digits in 2_u32..=10 {
        for size in 1..=digits / 2 {
            let repeat = digits / size;
            if !digits.is_multiple_of(size) || repeat == 2 {
                continue;
            }

            let start = 10_u64.pow(size - 1);
            let end = start * 10;
            let factor = (0..repeat).fold(0, |acc, _| acc * end + 1);

            invalid.extend((start..end).map(|n| factor * n));
        }
    }

    invalid.sort_unstable();
    invalid.dedup();

    invalid
}

fn sum(input: &Ranges, invalid: &Vec<u64>) -> u64 {
    let invalid_set: HashSet<u64> = invalid.iter().copied().collect();
    input
        .iter()
        .flat_map(|r| r.0..=r.1)
        .filter(|id| invalid_set.contains(id))
        .sum()
}

fn parse_ranges(input: &str) -> Ranges {
    input
        .trim()
        .split(",")
        .map(|range| {
            crate::utils::iter::vec_to_tuple_2(range.split("-").map(|el| el.unsigned()).collect())
        })
        .collect::<Vec<(u64, u64)>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,\
    222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
    824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part1(&input), 1227775554);
    }

    #[test]
    fn part2_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part2(&input), 4174379265);
    }
}
