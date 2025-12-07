use std::{env, fs};

type Map = Vec<Vec<char>>;

fn parse(input: &str) -> Map {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> usize {
    let mut map = parse(input);
    let width = map.first().unwrap().len();

    let mut total_split = 0;

    for cell in map.get_mut(0).unwrap() {
        if *cell == 'S' {
            *cell = '|';
        }
    }

    let mut last_row = map.first().unwrap().clone();
    for row in map.iter_mut().skip(1) {
        for (position, cell) in last_row.iter().enumerate() {
            if *cell == '|' {
                if row[position] == '.' {
                    row[position] = '|';
                }
                if row[position] == '^' {
                    total_split += 1;
                    if position > 0 {
                        row[position - 1] = '|';
                    }
                    if position < width - 1 {
                        row[position + 1] = '|';
                    }
                }
            }
        }
        last_row = row.clone();
    }

    total_split
}

fn part2(input: &str) -> u64 {
    let mut map = parse(input);
    // let height = map.len();
    let width = map.first().unwrap().len();

    for cell in map.get_mut(0).unwrap() {
        if *cell == 'S' {
            *cell = '|';
        }
    }

    let mut last_row = map.first().unwrap().clone();
    for row in map.iter_mut().skip(1) {
        for (position, cell) in last_row.iter().enumerate() {
            if *cell == '|' {
                if row[position] == '.' {
                    row[position] = '|';
                }
                if row[position] == '^' {
                    if position > 0 {
                        row[position - 1] = '|';
                    }
                    if position < width - 1 {
                        row[position + 1] = '|';
                    }
                }
            }
        }
        last_row = row.clone();
    }

    let mut timeline = vec![0u64; width];
    timeline[map
        .first()
        .unwrap()
        .iter()
        .position(|cell| *cell == '|')
        .unwrap()] = 1;

    for row_pair in map.windows(2) {
        let mut new_timeline = vec![0u64; width];
        for j in 0..width {
            if row_pair[0][j] == '|' {
                new_timeline[j] = timeline[j];
            }
            if j > 0 && row_pair[1][j-1] == '^' {
                new_timeline[j] += timeline[j-1];
            }
            if j < width-1 && row_pair[1][j+1] == '^' {
                new_timeline[j] += timeline[j+1];
            }
        }
        timeline = new_timeline;
    }

    timeline.iter().sum()
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
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn test_part1_example() {
        let output = 21;
        assert_eq!(part1(SHARED_INPUT.trim()), output);
    }

    #[test]
    fn test_part2_example() {
        let output = 40;
        assert_eq!(part2(SHARED_INPUT.trim()), output);
    }
}
