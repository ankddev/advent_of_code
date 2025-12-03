//! # Lobby
//!
//! Note that it's unoptimized too )

type Input = Vec<Vec<u32>>;

pub fn parse(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|bank| {
            bank.chars()
                .map(|character| {
                    character
                        .to_string()
                        .parse()
                        .expect("Couldn't parse number")
                })
                .collect()
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    input.into_iter().map(|bank| largest_joltage(bank, 2)).sum()
}

pub fn part2(input: &Input) -> u64 {
    input
        .into_iter()
        .map(|bank| largest_joltage(bank, 12))
        .sum()
}

fn largest_joltage(bank: &Vec<u32>, n: usize) -> u64 {
    let mut largest = 0u64;

    for (pos, _) in bank.iter().enumerate() {
        // Generate all combinations of n elements starting from this position
        let mut combo = Vec::with_capacity(n);
        generate_combos(&bank[(pos)..], n, 0, &mut combo, &mut |digits: &[u32]| {
            let mut joltage = 0u64;
            for &digit in digits {
                joltage = joltage * 10 + (digit as u64);
            }
            if joltage > largest {
                largest = joltage;
            }
        });
    }

    largest
}

// Helper function to generate all combinations of n digits
fn generate_combos(
    slice: &[u32],
    n: usize,
    start: usize,
    current: &mut Vec<u32>,
    process: &mut dyn FnMut(&[u32]),
) {
    if current.len() == n {
        process(&current);
        return;
    }

    for i in start..slice.len() {
        current.push(slice[i]);
        generate_combos(slice, n, i + 1, current, process);
        current.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
    987654321111111
    811111111111119
    234234234234278
    818181911112111";

    #[test]
    fn part1_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part1(&input), 357);
    }

    #[test]
    fn part2_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part2(&input), 3121910778619);
    }
}
