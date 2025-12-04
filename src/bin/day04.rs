use std::{env, fs};

type Map = Vec<Vec<Option<u8>>>;

fn parse(input: &str) -> (Map, usize, usize) {
    let mut map: Map = input
        .lines()
        .map(|row| row.chars().map(|c| (c == '@').then_some(0)).collect())
        .collect();
    let depth = map.len();
    let width = map.first().unwrap().len();

    for x in 0..depth {
        for y in 0..width {
            if map[x][y].is_some() {
                if x > 0 && y > 0 {
                    map[x - 1][y - 1] = map[x - 1][y - 1].map(|count| count + 1);
                }
                if x > 0 {
                    map[x - 1][y] = map[x - 1][y].map(|count| count + 1);
                }
                if x > 0 && y < width - 1 {
                    map[x - 1][y + 1] = map[x - 1][y + 1].map(|count| count + 1);
                }
                if y > 0 {
                    map[x][y - 1] = map[x][y - 1].map(|count| count + 1);
                }
                if y < width - 1 {
                    map[x][y + 1] = map[x][y + 1].map(|count| count + 1);
                }
                if x < depth - 1 && y > 0 {
                    map[x + 1][y - 1] = map[x + 1][y - 1].map(|count| count + 1);
                }
                if x < depth - 1 {
                    map[x + 1][y] = map[x + 1][y].map(|count| count + 1);
                }
                if x < depth - 1 && y < width - 1 {
                    map[x + 1][y + 1] = map[x + 1][y + 1].map(|count| count + 1);
                }
            }
        }
    }

    (map, depth, width)
}

fn remove(map: Map, depth: usize, width: usize) -> (Map, usize) {
    let mut new_map = map.clone();
    let mut removed = 0;

    for x in 0..depth {
        for y in 0..width {
            if map[x][y].is_some_and(|count| count < 4) {
                new_map[x][y] = None;
                removed += 1;
                if x > 0 && y > 0 {
                    new_map[x - 1][y - 1] =
                        new_map[x - 1][y - 1].map(|count| if count > 0 { count - 1 } else { 0 });
                }
                if x > 0 {
                    new_map[x - 1][y] =
                        new_map[x - 1][y].map(|count| if count > 0 { count - 1 } else { 0 });
                }
                if x > 0 && y < width - 1 {
                    new_map[x - 1][y + 1] =
                        new_map[x - 1][y + 1].map(|count| if count > 0 { count - 1 } else { 0 });
                }
                if y > 0 {
                    new_map[x][y - 1] =
                        new_map[x][y - 1].map(|count| if count > 0 { count - 1 } else { 0 });
                }
                if y < width - 1 {
                    new_map[x][y + 1] =
                        new_map[x][y + 1].map(|count| if count > 0 { count - 1 } else { 0 });
                }
                if x < depth - 1 && y > 0 {
                    new_map[x + 1][y - 1] =
                        new_map[x + 1][y - 1].map(|count| if count > 0 { count - 1 } else { 0 });
                }
                if x < depth - 1 {
                    new_map[x + 1][y] =
                        new_map[x + 1][y].map(|count| if count > 0 { count - 1 } else { 0 });
                }
                if x < depth - 1 && y < width - 1 {
                    new_map[x + 1][y + 1] =
                        new_map[x + 1][y + 1].map(|count| if count > 0 { count - 1 } else { 0 });
                }
            }
        }
    }

    (new_map, removed)
}

fn part1(input: &str) -> usize {
    let (map, depth, width) = parse(input);
    let (_, removed) = remove(map, depth, width);
    removed
}

fn part2(input: &str) -> usize {
    let (mut map, depth, width) = parse(input);
    let mut removed;

    let mut total_removed = 0;
    loop {
        (map, removed) = remove(map, depth, width);
        total_removed += removed;
        if removed == 0 {
            break;
        }
    }

    total_removed
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
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    #[test]
    fn test_part1_example() {
        let output = 13;
        assert_eq!(part1(SHARED_INPUT.trim()), output);
    }

    #[test]
    fn test_part2_example() {
        let output = 43;
        assert_eq!(part2(SHARED_INPUT.trim()), output);
    }
}
