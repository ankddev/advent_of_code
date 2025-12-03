//! # Lobby

type Input = Vec<Vec<u8>>;

pub fn parse(input: &str) -> Input {
    input
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

fn largest_joltage(bank: &Vec<u8>, limit: usize) -> u64 {
    let mut max = 0;
    let mut start = 0;

    (0..limit).fold(0, |joltage, digit| {
        let end = bank.len() - limit + digit + 1;

        (max, start) = (start..end).fold((0, 0), |(max, start), i| {
            if bank[i] > max {
                (bank[i], i + 1)
            } else {
                (max, start)
            }
        });

        10 * joltage + max as u64
    })
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
