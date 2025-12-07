//! # Trash Compactor
//!
//! It's a bit hacky, but at least working ))

use crate::utils::parse::ParseOps;

pub type Input = (usize, usize);

#[derive(Debug)]
pub struct Problem {
    pub numbers: Vec<String>,
    pub operator: Operator,
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Multiply,
}

pub fn parse(input: &str) -> Input {
    let lines = input.lines().map(|line| line.trim()).collect::<Vec<_>>();

    let operators = lines
        .last()
        .unwrap_or(&"")
        .split_ascii_whitespace()
        .map(|op| {
            if op == "*" {
                Operator::Multiply
            } else {
                Operator::Add
            }
        })
        .collect::<Vec<_>>();

    let num_lines = lines[..lines.len() - 1]
        .into_iter()
        .map(|line| {
            split_preserve_leading_spaces(line)
                .into_iter()
                .map(|element| element)
                .collect()
        })
        .collect::<Vec<Vec<String>>>();

    let mut numbers = vec![vec![String::new(); num_lines.len()]; num_lines[0].len()];

    for (x, line) in num_lines.iter().enumerate() {
        for (y, num) in line.iter().enumerate() {
            numbers[y][x] = num.to_owned();
        }
    }

    let parsed: Vec<Problem> = numbers
        .into_iter()
        .zip(operators)
        .map(|(numbers, operator)| Problem { numbers, operator })
        .collect();

    let part1 = parsed.iter().fold(0, |acc, Problem { numbers, operator }| match operator {
        Operator::Add => numbers.iter().map(|num| num.unsigned::<usize>()).sum::<usize>(),
        Operator::Multiply => numbers.iter().map(|num| num.unsigned::<usize>()).product()
    } + acc);

    let max_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut last_op = ' ';
    let mut part2 = 0;
    let mut current_group = Vec::new();

    for col in 0..max_len {
        let mut number = 0;
        let mut multiplier = 1;
        let mut found_digit = false;

        // Read column from bottom to top
        for row in (0..lines.len()).rev() {
            let line = lines[row].trim();
            if line.is_empty() {
                continue;
            }

            if col < line.len() {
                let ch = line.chars().nth(col).unwrap();

                if ch == '*' || ch == '+' {
                    last_op = ch;
                } else if ch.is_ascii_digit() {
                    number += ch.to_digit(10).unwrap() as usize * multiplier;
                    multiplier *= 10;
                    found_digit = true;
                }
            }
        }

        if found_digit {
            current_group.push(number);
        } else if !current_group.is_empty() {
            // End of a group
            let group_result = if last_op == '*' {
                current_group.iter().product()
            } else if last_op == '+' {
                current_group.iter().sum()
            } else {
                0
            };
            part2 += group_result;
            current_group.clear();
        }
    }

    // Handle last group
    if !current_group.is_empty() {
        let group_result = if last_op == '*' {
            current_group.iter().product()
        } else if last_op == '+' {
            current_group.iter().sum()
        } else {
            0
        };
        part2 += group_result;
    }

    (part1, part2)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

fn split_preserve_leading_spaces(line: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c != ' ' {
            // Collect consecutive non-space characters into current
            current.push(c);
            while let Some(&next_c) = chars.peek() {
                if next_c != ' ' {
                    current.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            // Now collect trailing spaces to shift forward to next segment
            let mut spaces = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c == ' ' {
                    spaces.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            // Push current segment without trailing spaces
            result.push(current);
            // The spaces become start of next segment
            current = spaces;
        } else {
            // For leading spaces (at start or between segments)
            current.push(c);
        }
    }
    if !current.is_empty() {
        result.push(current);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
    123 328  51 64
     45 64  387 23
      6 98  215 314
    *   +   *   +  ";

    #[test]
    fn part1_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part1(&input), 4277556);
    }
}
