use std::env;

use rust_11::StoneLine;

fn main() {
    let dir = env::temp_dir();
    println!("Temporary directory: {}", dir.display());

    let input = include_str!("../input").trim();

    let mut stone_line: StoneLine = input.parse().unwrap();

    stone_line.blink_all_times(25);

    println!("part 1 solution: {}", stone_line.num_stones);

    stone_line.blink_all_times(50);

    println!("part 2 solution: {}", stone_line.num_stones);
}
