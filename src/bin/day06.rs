use std::{env, fs};

enum Operation {
    Addition,
    Multiplication,
}

fn part1(input: &str) -> u64 {
    // grid[row][column]
    let grid: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let height = grid.len();
    let width = grid.first().unwrap().len();

    // grid[column][row]
    let grid: Vec<Vec<&str>> = (0..width)
        .map(|j| (0..height).map(|i| grid[i][j]).collect())
        .collect();

    let mut grand_total = 0;
    for column in grid {
        grand_total += match *column.last().unwrap() {
            "+" => column
                .iter()
                .take(column.len() - 1)
                .map(|num| num.parse::<u64>().unwrap())
                .sum::<u64>(),
            "*" => column
                .iter()
                .take(column.len() - 1)
                .map(|num| num.parse::<u64>().unwrap())
                .product::<u64>(),
            _ => panic!(),
        };
    }

    grand_total
}

fn part2(input: &str) -> u64 {
    // grid[row][column]
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid.first().unwrap().len();

    // grid[column][row]
    let grid: Vec<Vec<char>> = (0..width)
        .map(|j| (0..height).map(|i| grid[i][j]).collect())
        .collect();

    let mut grand_total = 0;
    let mut total = 0;
    let mut operation = Operation::Addition;
    for column in grid {
        if column.iter().all(|cell| *cell == ' ') {
            grand_total += total;
            continue;
        }

        match column.last().unwrap() {
            '+' => {
                operation = Operation::Addition;
                total = 0;
            }
            '*' => {
                operation = Operation::Multiplication;
                total = 1;
            }
            _ => (),
        }

        let num = column
            .iter()
            .filter(|cell| cell.is_ascii_digit())
            .fold(0u64, |num, digit| {
                num * 10 + digit.to_digit(10).unwrap() as u64
            });
        match operation {
            Operation::Addition => total += num,
            Operation::Multiplication => total *= num,
        }
    }
    grand_total += total;

    grand_total
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
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#;

    #[test]
    fn test_part1_example() {
        let output = 4277556;
        assert_eq!(part1(&SHARED_INPUT[1..SHARED_INPUT.len() - 1]), output);
    }

    #[test]
    fn test_part2_example() {
        let output = 3263827;
        assert_eq!(part2(&SHARED_INPUT[1..SHARED_INPUT.len() - 1]), output);
    }
}
