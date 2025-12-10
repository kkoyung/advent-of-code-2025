use std::{env, fs, iter::Sum};

use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variables};

// The solution requires external integer programming solver. It uses COIN-OR Branch-and-Cut solver
// (https://github.com/coin-or/Cbc). To install it, you can run the following commands:
//
// Arch Linux:
// $ pacman -S coin-or-cbc
//
// Defian:
// $ sudo apt-get install  coinor-cbc coinor-libcbc-dev

type Light = u8;
type Button = Vec<usize>;
type Joltage = u32;

struct Machine {
    lights: Vec<Light>,
    buttons: Vec<Button>,
    joltages: Vec<Joltage>,
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let section: Vec<&str> = line.split_ascii_whitespace().collect();
            let lights: Vec<Light> = section
                .first()
                .unwrap()
                .trim_matches(['[', ']'])
                .chars()
                .map(|light| match light {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!(),
                })
                .collect();
            let buttons: Vec<Button> = section
                .iter()
                .take(section.len() - 1)
                .skip(1)
                .map(|button_str| {
                    button_str
                        .trim_matches(['(', ')'])
                        .split(",")
                        .map(|num| num.parse().unwrap())
                        .collect::<Button>()
                })
                .collect();
            let joltages: Vec<Joltage> = section
                .last()
                .unwrap()
                .trim_matches(['{', '}'])
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect();

            Machine {
                lights,
                buttons,
                joltages,
            }
        })
        .collect()
}

fn part1(input: &str) -> u64 {
    let machines = parse(input);

    machines
        .iter()
        .map(|machine| {
            variables! {
                vars:
                    0 <= x[machine.buttons.len()] (integer);
                    0 <= multiplier[machine.lights.len()] (integer);
            }

            let mut lp = vars
                .minimise(Expression::sum(x.iter()))
                .using(default_solver);
            let light_indices = 0..machine.lights.len();
            for light_index in light_indices {
                let lin_comb = Expression::sum(
                    machine
                        .buttons
                        .iter()
                        .enumerate()
                        .filter(|(_, button)| button.contains(&light_index))
                        .map(|(column, _)| x[column]),
                );
                lp = lp.with(constraint!(
                    lin_comb == 2 * multiplier[light_index] + machine.lights[light_index]
                ))
            }

            let solution = lp.solve().unwrap();
            solution.eval(Expression::sum(x.iter())).round() as u64
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let machines = parse(input);

    machines
        .iter()
        .map(|machine| {
            variables! {
                vars:
                    0 <= x[machine.buttons.len()] (integer);
            }

            let mut lp = vars
                .minimise(Expression::sum(x.iter()))
                .using(default_solver);
            let joltage_indices = 0..machine.lights.len();
            for joltage_index in joltage_indices {
                let lin_comb = Expression::sum(
                    machine
                        .buttons
                        .iter()
                        .enumerate()
                        .filter(|(_, button)| button.contains(&joltage_index))
                        .map(|(column, _)| x[column]),
                );
                lp = lp.with(constraint!(lin_comb == machine.joltages[joltage_index]))
            }

            let solution = lp.solve().unwrap();
            solution.eval(Expression::sum(x.iter())).round() as u64
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
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

    #[test]
    fn test_part1_example() {
        let output = 7;
        assert_eq!(part1(SHARED_INPUT.trim()), output);
    }

    #[test]
    fn test_part2_example() {
        let output = 33;
        assert_eq!(part2(SHARED_INPUT.trim()), output);
    }
}
