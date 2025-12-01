# Advent of Code 2025

[Advent of Code](https://adventofcode.com) is an Advent calendar of
small programming puzzles released every December by [Eric
Wastl](http://was.tl/). This repository contains my solutions to [Advent
of Code 2025](https://adventofcode.com/2025), written in Rust.

The solutions for puzzles are located in `src/bin` directory, along with
unit tests based on the provided example, organized by days. Take the
imaginary puzzle `day00` as an example, the solution is located in
`src/bin/day00.rs`, with unit tests `tests::test_example_part1` and
`tests::test_example_part1`.

To run the code, you can use the [just](https://just.systems/) command
runner with the included configuration file `justfile`.

```bash
# Run the example, for day 00, part 1
$ just test 00 1

# Run the example, for day 00, part 2
$ just test 00 2

# Run the code on your input at "input/day05.txt", for day 00, part 1
$ just run 00 1

# Run the code on your input at "input/day05.txt", for day 00, part 1
$ just run 00 2

```

There is another command to quickly create a file in `src/bin` for a new
day, using `day00` as template.

```bash
# Example: Create src/bin/day05.rs
$ just new 05
```

If you find that the code doesn't work, or you'd like to discuss the
code with me, feel free to open an issue in this GitHub repository or
email me (mailto:kingsley@kkoyung.dev).
