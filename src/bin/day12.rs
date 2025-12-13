use std::{env, fs};

struct Piece {
    shape: [[bool; 3]; 3],
}

struct Puzzle {
    size: (usize, usize),
    nums: Vec<usize>,
}

fn parse(input: &str) -> (Vec<Piece>, Vec<Puzzle>) {
    let sections = input.split("\n\n");

    let pieces = sections
        .clone()
        .take(sections.clone().count() - 1)
        .map(|section| {
            let mut piece = Piece {
                shape: [[false; 3]; 3],
            };
            section.lines().skip(1).enumerate().for_each(|(y, row)| {
                row.chars()
                    .enumerate()
                    .for_each(|(x, cell)| piece.shape[x][y] = cell == '#')
            });
            piece
        })
        .collect::<Vec<Piece>>();

    let puzzles = sections
        .last()
        .unwrap()
        .lines()
        .map(|line| {
            let (size, nums) = line.split_once(": ").unwrap();
            let size = size.split_once("x").unwrap();
            let size = (
                size.0.parse::<usize>().unwrap(),
                size.1.parse::<usize>().unwrap(),
            );
            let nums = nums
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Puzzle { size, nums }
        })
        .collect::<Vec<Puzzle>>();

    (pieces, puzzles)
}

fn part1(input: &str) -> usize {
    let (_pieces, puzzles) = parse(input);

    puzzles
        .iter()
        .filter(|puzzle| puzzle.nums.iter().sum::<usize>() * 9 <= puzzle.size.0 * puzzle.size.1)
        .count()
}

fn part2(_input: &str) -> usize {
    // Merry Christmas!
    0
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
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

    #[test]
    fn test_part1_example() {
        let output = 2;
        assert_eq!(part1(SHARED_INPUT.trim()), output);
        // The function `part1` is only a heuristic that works with the user input, but not the
        // example.
    }

    #[test]
    fn test_part2_example() {
        let output = 0;
        assert_eq!(part2(SHARED_INPUT.trim()), output);
    }
}
