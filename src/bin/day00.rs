use std::{env, fs};

fn parse(input: &str) -> &str {
    input
}

fn part1(input: &str) -> u32 {
    let input = parse(input);

    // Return number of bytes
    input.len() as u32
}

fn part2(input: &str) -> u32 {
    let input = parse(input);

    // Return number of characters
    input.chars().count() as u32
}

// =====================================================================

fn main() {
    let part = env::args().nth(1).expect("Missing <part>");
    let filename = env::args().nth(2).expect("Missing <input>");
    let input = fs::read_to_string(filename).expect("Unable to read file");

    match part.as_str() {
        "part1" => println!("Part1: {}", part1(&input)),
        "part2" => println!("Part2: {}", part2(&input)),
        _ => panic!("Unknown part: {}", part)
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1_example() {
        let input = "Hello👋🏻, World🌏";
        let output = 24;
        assert_eq!(part1(input), output);
    }

    #[test]
    fn test_part2_example() {
        let input = "Hello👋🏻, World🌏";
        let output = 15;
        assert_eq!(part2(input), output);
    }
}
