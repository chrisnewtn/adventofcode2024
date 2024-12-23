use rust_10::TrailGrid;

fn main() {
    let input = include_str!("../input").trim();

    let grid = TrailGrid::build(input);

    println!("part 1 solution: {}", grid.total_score());

    println!("part 2 solution: {}", grid.total_distinct_score());
}
