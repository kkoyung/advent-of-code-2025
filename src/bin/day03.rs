use std::{env, fs};

fn max_position(digits: &[u64], skip_last: usize) -> (u64, usize) {
    let max_digit = digits[..digits.len() - skip_last].iter().max().unwrap();
    let position = digits.iter().position(|digit| digit == max_digit).unwrap();
    (*max_digit, position)
}

fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> u64 {
    let input = parse(input);

    input
        .iter()
        .map(|bank| {
            let (first_digit, position) = max_position(bank, 1);
            let (second_digit, _) = max_position(&bank[position + 1..], 0);
            first_digit * 10 + second_digit
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let input = parse(input);

    input
        .iter()
        .map(|bank| {
            let mut bank: &[u64] = bank;
            let mut joltage = 0u64;
            for round in (0..12).rev() {
                let (digit, position) = max_position(bank, round);
                joltage = joltage * 10 + digit;
                bank = &bank[position + 1..];
            }
            joltage
        })
        .sum()
}

// =====================================================================

fn main() {
    let part = env::args().nth(1).expect("Missing <part>");
    let filename = env::args().nth(2).expect("Missing <input>");
    let input = fs::read_to_string(filename).expect("Unable to read file");

    match part.as_str() {
        "part1" => println!("Part1: {}", part1(&input)),
        "part2" => println!("Part2: {}", part2(&input)),
        _ => panic!("Unknown part: {}", part),
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SHARED_INPUT: &str = r#"
987654321111111
811111111111119
234234234234278
818181911112111
"#;

    #[test]
    fn test_part1_example() {
        let output = 357;
        assert_eq!(part1(SHARED_INPUT.trim()), output);
    }

    #[test]
    fn test_part2_example() {
        let output = 3121910778619;
        assert_eq!(part2(SHARED_INPUT.trim()), output);
    }
}
