use std::{collections::HashSet, env, fs};

type Range = (u64, u64);

fn parse(input: &str) -> (Vec<Range>, Vec<u64>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .trim()
        .split("\n")
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();
    let ingredients = ingredients
        .trim()
        .split("\n")
        .map(|ingredient| ingredient.parse().unwrap())
        .collect();

    (ranges, ingredients)
}

fn part1(input: &str) -> usize {
    let (ranges, ingredients) = parse(input);

    ingredients
        .iter()
        .filter(|&ingredient| {
            ranges
                .iter()
                .any(|(start, end)| start <= ingredient && ingredient <= end)
        })
        .count()
}

fn find_mergeables(mut range_set: HashSet<Range>, next: Range) -> (HashSet<Range>, Vec<Range>) {
    let mut mergeables = vec![next];
    for range in &range_set {
        if !(next.1 + 1 < range.0 || range.1 + 1 < next.0) {
            mergeables.push(*range);
        }
    }

    mergeables.iter().for_each(|range| {
        range_set.remove(range);
    });

    (range_set, mergeables)
}

fn merge_mergeables(mergables: Vec<Range>) -> Range {
    let start = mergables.iter().map(|(start, _)| start).min().unwrap();
    let end = mergables.iter().map(|(_, end)| end).max().unwrap();
    (*start, *end)
}

fn part2(input: &str) -> u64 {
    let (mut ranges, _) = parse(input);

    let mut mergables;
    let mut range_set = HashSet::new();
    range_set.insert(ranges.pop().unwrap());

    for next in ranges {
        (range_set, mergables) = find_mergeables(range_set, next);
        let merged = merge_mergeables(mergables);
        range_set.insert(merged);
    }

    range_set.iter().map(|range| range.1 - range.0 + 1).sum()
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
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

    #[test]
    fn test_part1_example() {
        let output = 3;
        assert_eq!(part1(SHARED_INPUT.trim()), output);
    }

    #[test]
    fn test_part2_example() {
        let output = 14;
        assert_eq!(part2(SHARED_INPUT.trim()), output);
    }
}
