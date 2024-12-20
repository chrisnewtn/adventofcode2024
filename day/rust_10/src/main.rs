use std::str::FromStr;
use rust_10::{Grid, Trail};

fn main() {
    let input = include_str!("../input").trim();

    let grid: Grid<Trail> = Grid::from_str(input).unwrap();

    println!("part 1 solution: {}", grid.total_score());

    println!("part 2 solution: {}", grid.total_distinct_score());
}
