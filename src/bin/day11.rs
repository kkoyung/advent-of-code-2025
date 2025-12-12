use std::{collections::HashMap, env, fs};

type Device = String;
type Map = HashMap<Device, Vec<Device>>;

fn parse(input: &str) -> Map {
    let pair_iter = input.lines().flat_map(|line| {
        let (device, output_list) = line.split_once(": ").unwrap();
        output_list
            .split_whitespace()
            .map(|output| (device.to_string(), output.to_string()))
    });

    let mut backward_map = HashMap::<Device, Vec<Device>>::new();
    for (input, output) in pair_iter {
        backward_map
            .entry(output)
            .and_modify(|list| list.push(input.clone()))
            .or_insert(vec![input]);
    }

    backward_map
}

fn count_path_to(
    backward_map: &Map,
    start: &str,
    end: &str,
    memory: &mut HashMap<String, usize>,
) -> usize {
    let count = if end == start {
        1
    } else if let Some(inputs) = backward_map.get(end) {
        let mut sum = 0;
        for input in inputs {
            if let Some(memorized_count) = memory.get(input) {
                sum += memorized_count;
            } else {
                sum += count_path_to(backward_map, start, input, memory);
            }
        }
        sum
    } else {
        0
    };
    memory.insert(end.to_string(), count);

    count
}

fn part1(input: &str) -> usize {
    let backward_map = parse(input);

    let mut memory = HashMap::new();
    count_path_to(&backward_map, "you", "out", &mut memory)
}

fn part2(input: &str) -> usize {
    let backward_map = parse(input);

    let mut product = 1;

    let mut memory = HashMap::new();
    product *= count_path_to(&backward_map, "svr", "fft", &mut memory);

    let mut memory = HashMap::new();
    product *= count_path_to(&backward_map, "fft", "dac", &mut memory);

    let mut memory = HashMap::new();
    product *= count_path_to(&backward_map, "dac", "out", &mut memory);

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

    const SHARED_INPUT_1: &str = r#"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;

    const SHARED_INPUT_2: &str = r#"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

    #[test]
    fn test_part1_example() {
        let output = 5;
        assert_eq!(part1(SHARED_INPUT_1.trim()), output);
    }

    #[test]
    fn test_part2_example() {
        let output = 2;
        assert_eq!(part2(SHARED_INPUT_2.trim()), output);
    }
}
