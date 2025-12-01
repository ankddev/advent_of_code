//! # Secret Entrance
//!
//! It's currently not optimised at all
use crate::utils::parse::ParseOps;

type Input = Vec<Move>;

pub enum Move {
    Left(u32),
    Right(u32),
}

pub fn parse(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Move {
    let line = line.trim();
    let num = line.iter_unsigned::<u32>().next().unwrap();
    if line.starts_with("L") {
        Move::Left(num)
    } else {
        Move::Right(num)
    }
}

pub fn part1(input: &Input) -> u32 {
    let mut current = 50;
    input
        .iter()
        .map(|val| match val {
            Move::Right(num) => {
                for _ in 0..*num {
                    if current == 99 {
                        current = 0
                    } else {
                        current += 1;
                    }
                }
                current
            }
            Move::Left(num) => {
                for _ in 0..*num {
                    if current == 0 {
                        current = 99
                    } else {
                        current -= 1;
                    }
                }
                current
            }
        })
        .filter(|&num| num == 0)
        .count() as u32
}

pub fn part2(input: &Input) -> u32 {
    let mut current = 50;
    let mut count = 0;
    let _ = input
        .iter()
        .map(|val| match val {
            Move::Right(num) => {
                for _ in 0..*num {
                    if current == 0 {
                        count += 1;
                        current += 1;
                    } else if current == 99 {
                        current = 0;
                    } else {
                        current += 1;
                    }
                }
                current
            }
            Move::Left(num) => {
                for _ in 0..*num {
                    if current == 0 {
                        current = 99;
                        count += 1;
                    } else {
                        current -= 1;
                    }
                }
                current
            }
        })
        .collect::<Vec<_>>();
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
    L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82";

    #[test]
    fn part1_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn part2_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part2(&input), 6);
    }
}
