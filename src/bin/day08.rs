use std::{collections::BTreeMap, env, fs};

type Junction = (u64, u64, u64);
type Circuit = Vec<Junction>;

fn distance_squared(junction_a: Junction, junction_b: Junction) -> u64 {
    junction_a.0.abs_diff(junction_b.0) * junction_a.0.abs_diff(junction_b.0)
        + junction_a.1.abs_diff(junction_b.1) * junction_a.1.abs_diff(junction_b.1)
        + junction_a.2.abs_diff(junction_b.2) * junction_a.2.abs_diff(junction_b.2)
}

fn parse(input: &str) -> (Vec<(Junction, Junction)>, Vec<Circuit>) {
    let junctions: Vec<Junction> = input
        .lines()
        .map(|line| {
            let coordinates = line
                .split(",")
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (coordinates[0], coordinates[1], coordinates[2])
        })
        .collect();

    let mut distance_map = BTreeMap::new();
    for i in 0..junctions.len() {
        for j in i + 1..junctions.len() {
            distance_map.insert(
                distance_squared(junctions[i], junctions[j]),
                (junctions[i], junctions[j]),
            );
        }
    }
    let sorted_pair: Vec<(Junction, Junction)> = distance_map
        .iter()
        .map(|(_, (junction_a, junction_b))| (*junction_a, *junction_b))
        .collect();

    let circuits: Vec<Circuit> = junctions
        .into_iter()
        .map(|junction| vec![junction])
        .collect();

    (sorted_pair, circuits)
}

fn part1(input: &str) -> usize {
    let (sorted_pair, mut circuits) = parse(input);

    #[cfg(test)]
    let shortest_pair = 10;

    #[cfg(not(test))]
    let shortest_pair = 1000;

    for (junction_a, junction_b) in sorted_pair.iter().take(shortest_pair) {
        let involved_circuit: Vec<Circuit> = circuits
            .extract_if(.., |circuit| {
                circuit.contains(junction_a) || circuit.contains(junction_b)
            })
            .collect();

        let new_circuit =
            involved_circuit
                .iter()
                .fold(Circuit::default(), |mut new_circult, circuit| {
                    new_circult.extend(circuit);
                    new_circult
                });
        circuits.push(new_circuit);
    }

    let mut sizes: Vec<usize> = circuits.iter().map(|circuit| circuit.len()).collect();
    sizes.sort();
    sizes.iter().rev().take(3).product()
}

fn part2(input: &str) -> u64 {
    let (sorted_pair, mut circuits) = parse(input);

    let mut product = 0;
    for (junction_a, junction_b) in sorted_pair.iter() {
        let involved_circuit: Vec<Circuit> = circuits
            .extract_if(.., |circuit| {
                circuit.contains(junction_a) || circuit.contains(junction_b)
            })
            .collect();

        let new_circuit =
            involved_circuit
                .iter()
                .fold(Circuit::default(), |mut new_circult, circuit| {
                    new_circult.extend(circuit);
                    new_circult
                });
        circuits.push(new_circuit);

        if circuits.len() == 1 {
            product = junction_a.0 * junction_b.0;
            break;
        }
    }

    product
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
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

    #[test]
    fn test_part1_example() {
        let output = 40;
        assert_eq!(part1(SHARED_INPUT.trim()), output);
    }

    #[test]
    fn test_part2_example() {
        let output = 25272;
        assert_eq!(part2(SHARED_INPUT.trim()), output);
    }
}
