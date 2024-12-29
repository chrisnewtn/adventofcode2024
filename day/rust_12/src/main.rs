use rust_12::price_grid;

fn main() {
    let input = include_str!("../input").trim();

    let price = price_grid(input);

    println!("part 1 solution: {}", price);
}
