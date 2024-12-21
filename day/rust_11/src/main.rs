use rust_11::StoneLine;

fn main() {
    let input = include_str!("../input").trim();

    let mut stone_line: StoneLine = input.parse().unwrap();

    stone_line.blink_all_times(25);

    println!("part 1 solution: {}", stone_line.stones.len());

    stone_line.blink_all_times(15);

    println!("part 2 solution: {}", stone_line.stones.len());
}
