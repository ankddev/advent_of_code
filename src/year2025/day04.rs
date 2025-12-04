//! # Printing Department

type Input = Vec<Vec<Cell>>;

#[derive(Clone)]
pub enum Cell {
    Roll,
    None,
}

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            line.chars()
                .map(|c| if c == '@' { Cell::Roll } else { Cell::None })
                .collect()
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    let input = &mut input.into_iter().cloned().collect();

    replace_and_count(input).1
}

fn replace_and_count(input: &mut Input) -> (&Input, u32) {
    let mut rolls = Vec::new();

    for (n, row) in input.iter().enumerate() {
        for (m, el) in row.iter().enumerate() {
            if matches!(el, Cell::None) {
                continue;
            }

            let n = n as isize;
            let m = m as isize;
            let mut neighbours = 0;
            for x in n - 1..=n + 1 {
                for y in m - 1..=m + 1 {
                    if x < 0
                        || y < 0
                        || x >= input.len() as isize
                        || y >= row.len() as isize
                        || (x, y) == (n, m)
                    {
                        continue;
                    }

                    if matches!(input[x as usize][y as usize], Cell::Roll) {
                        neighbours += 1;
                    }
                }
            }

            if neighbours < 4 {
                rolls.push((n as usize, m as usize));
            }
        }
    }

    for (x, y) in &rolls {
        input[*x][*y] = Cell::None;
    }

    (input, rolls.len() as u32)
}

pub fn part2(input: &Input) -> u32 {
    let mut result = 0;
    let input = &mut input.into_iter().cloned().collect();

    loop {
        let count = replace_and_count(input).1;
        if count == 0 {
            break;
        }

        result += count;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.";

    #[test]
    fn part1_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn part2_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part2(&input), 43);
    }
}
