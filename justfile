test day part:
  cargo test "tests::test_part{{part}}_example" --bin day{{day}} -- --show-output

run day part:
  cargo run --bin day{{day}} -- part{{part}} input/day{{day}}.txt

new day:
  cp src/bin/day00.rs src/bin/day{{day}}.rs
