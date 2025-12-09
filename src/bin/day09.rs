use std::{env, fs};

type Tile = (u64, u64);
type Line = (Tile, Tile);

fn parse(input: &str) -> Vec<Tile> {
    input
        .lines()
        .map(|line| {
            let mut iterator = line.split(",").map(|num| num.parse::<u64>().unwrap());
            (iterator.next().unwrap(), iterator.next().unwrap())
        })
        .collect()
}

fn area(tile_a: Tile, tile_b: Tile) -> u64 {
    (tile_a.0.abs_diff(tile_b.0) + 1) * (tile_a.1.abs_diff(tile_b.1) + 1)
}

fn part1(input: &str) -> u64 {
    let tiles = parse(input);

    let mut largest_area = 0;
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let area = area(tiles[i], tiles[j]);
            if area > largest_area {
                largest_area = area;
            }
        }
    }

    largest_area
}

fn line_cut_box(line: Line, tile_a: Tile, tile_b: Tile) -> bool {
    let head = line.0;
    let tail = line.1;

    if head.0 == tail.0 {
        // Vertical line
        let horizontal_condition =
            tile_a.0.min(tile_b.0) < head.0 && head.0 < tile_a.0.max(tile_b.0);
        let vertical_conditionb = !((head.1.max(tail.1) <= tile_a.1.min(tile_b.1))
            || (head.1.min(tail.1) >= tile_a.1.max(tile_b.1)));
        horizontal_condition && vertical_conditionb
    } else if head.1 == tail.1 {
        // Horizontal line
        let horizontal_condition = !((head.0.max(tail.0) <= tile_a.0.min(tile_b.0))
            || (head.0.min(tail.0) >= tile_a.0.max(tile_b.0)));
        let vertical_conditionb =
            tile_a.1.min(tile_b.1) < head.1 && head.1 < tile_a.1.max(tile_b.1);
        horizontal_condition && vertical_conditionb
    } else {
        panic!()
    }
}

fn lines_cut_box(lines: &[Line], tile_a: Tile, tile_b: Tile) -> bool {
    lines.iter().any(|line| line_cut_box(*line, tile_a, tile_b))
}

fn part2(input: &str) -> u64 {
    let tiles = parse(input);

    let lines: Vec<Line> = [tiles.clone(), vec![*tiles.first().unwrap()]]
        .into_iter()
        .flatten()
        .collect::<Vec<Tile>>()
        .windows(2)
        .map(|window| (window[0], window[1]))
        .collect();

    let mut largest_area = 0;
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let tile_a = tiles[i];
            let tile_b = tiles[j];

            if lines_cut_box(&lines, tile_a, tile_b) {
                continue;
            }

            let area = area(tile_a, tile_b);
            if area > largest_area {
                largest_area = area;
            }
        }
    }

    largest_area
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
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    #[test]
    fn test_part1_example() {
        let output = 50;
        assert_eq!(part1(SHARED_INPUT.trim()), output);
    }

    #[test]
    fn test_part2_example() {
        let output = 24;
        assert_eq!(part2(SHARED_INPUT.trim()), output);
    }
}
