use std::{env, fs, ops};

#[derive(Clone, Copy)]
enum Position {
    Space,
    Splitter,
    Beam(u64),
}

type Map = Vec<Vec<Position>>;

fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Position::Space,
                    '^' => Position::Splitter,
                    'S' => Position::Beam(1),
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

impl Position {
    fn is_splitter(&self) -> bool {
        match self {
            Position::Space => false,
            Position::Splitter => true,
            Position::Beam(_) => false,
        }
    }

    fn is_beam(&self) -> bool {
        match self {
            Position::Space => false,
            Position::Splitter => false,
            Position::Beam(_) => true,
        }
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        match (self, rhs) {
            (Position::Space, Position::Space) => Position::Space,
            (Position::Space, Position::Splitter) => Position::Space,
            (Position::Space, Position::Beam(rhs)) => Position::Beam(rhs),
            (Position::Splitter, Position::Space) => Position::Splitter,
            (Position::Splitter, Position::Splitter) => Position::Splitter,
            (Position::Splitter, Position::Beam(_)) => Position::Splitter,
            (Position::Beam(lhs), Position::Space) => Position::Beam(lhs),
            (Position::Beam(lhs), Position::Splitter) => Position::Beam(lhs),
            (Position::Beam(lhs), Position::Beam(rhs)) => Position::Beam(lhs + rhs),
        }
    }
}

fn emit(map: &mut Map) -> u64 {
    let height = map.len();
    let width = map.first().unwrap().len();

    let mut total_split = 0;
    for i in 0..height - 1 {
        for j in 0..width {
            if !map[i][j].is_beam() {
                continue;
            }
            map[i + 1][j] = map[i + 1][j] + map[i][j];
            if map[i + 1][j].is_splitter() {
                total_split += 1;
                if j > 0 {
                    map[i + 1][j - 1] = map[i + 1][j - 1] + map[i][j];
                }
                if j < width - 1 {
                    map[i + 1][j + 1] = map[i + 1][j + 1] + map[i][j];
                }
            }
        }
    }

    total_split
}

fn part1(input: &str) -> u64 {
    let mut map = parse(input);
    emit(&mut map)
}

fn part2(input: &str) -> u64 {
    let mut map = parse(input);
    emit(&mut map);
    map.last()
        .unwrap()
        .iter()
        .map(|position| match position {
            Position::Space => 0,
            Position::Splitter => 0,
            Position::Beam(beam) => *beam,
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
