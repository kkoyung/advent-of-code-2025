use std::{collections::HashSet, env, fs};

fn collect_invalid_ids(invalid_ids: &mut HashSet<u64>, times: u32, start: u64, end: u64) {
    for width in 1..=10u32 / times {
        let multiplier: u64 = (0..times).map(|t| 10u64.pow(width * t)).sum();
        let halves = 10u64.pow(width - 1)..=10u64.pow(width) - 1;
        for half in halves {
            let invalid_id = multiplier * half;
            if start <= invalid_id && invalid_id <= end {
                invalid_ids.insert(invalid_id);
            }
        }
    }
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(",")
        .map(|range| {
            range
                .split_once("-")
                .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
                .unwrap()
        })
        .collect()
}

fn part1(input: &str) -> u64 {
    let input = parse(input);

    let mut invalid_ids = HashSet::<u64>::new();
    input
        .iter()
        .for_each(|(start, end)| collect_invalid_ids(&mut invalid_ids, 2, *start, *end));
    invalid_ids.iter().sum()
}

fn part2(input: &str) -> u64 {
    let input = parse(input);

    let mut invalid_ids = HashSet::<u64>::new();
    for times in 2..=10 {
        input
            .iter()
            .for_each(|(start, end)| collect_invalid_ids(&mut invalid_ids, times, *start, *end));
    }
    invalid_ids.iter().sum()
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
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
"#;

    #[test]
    fn test_part1_example() {
        let output = 1227775554;
        assert_eq!(part1(SHARED_INPUT.trim()), output);
    }

    #[test]
    fn test_part2_example() {
        let output = 4174379265;
        assert_eq!(part2(SHARED_INPUT.trim()), output);
    }
}
