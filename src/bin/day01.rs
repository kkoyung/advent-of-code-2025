use std::{env, fs};

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            if let Some(num) = line.strip_prefix("L") {
                -num.parse::<i32>().unwrap()
            } else if let Some(num) = line.strip_prefix("R") {
                num.parse::<i32>().unwrap()
            } else {
                panic!("Unknown rotation");
            }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let input = parse(input);

    let mut dial = 50;
    let mut count = 0;
    for rotation in input {
        dial = (dial + rotation).rem_euclid(100);
        if dial == 0 {
            count += 1;
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let input = parse(input);

    let mut dial: i32 = 50;
    let mut count = 0;
    for rotation in input {
        count += if rotation > 0 {
            // `R` rotation:
            // Count the number of times of crossing multiples of 100.
            (dial + rotation) / 100
        } else {
            // `L` rotation:
            // Label 1 as 99, 2 as 98, ..., 99 as 1, but 0 as 0. New `L` rotation is toward higher
            // numbers. We can then apply the same logic in the case of `R` rotation.
            let reverse_dial = (100 - dial) % 100;
            (reverse_dial - rotation) / 100
        };
        dial = (dial + rotation).rem_euclid(100);
    }

    count as usize
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
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn test_part1_example() {
        let output = 3;
        assert_eq!(part1(SHARED_INPUT.trim()), output);
    }

    #[test]
    fn test_part2_example() {
        let output = 6;
        assert_eq!(part2(SHARED_INPUT.trim()), output);
    }
}
