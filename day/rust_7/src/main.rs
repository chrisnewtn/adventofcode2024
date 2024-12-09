use std::{fs::File, io::{self, BufRead}, str::FromStr};
use rust_7::Equation;

fn main() {
    let file = File::open("./input").unwrap();
    let mut total = 0;

    for line_result in io::BufReader::new(file).lines() {
        if let Ok(line) = line_result {
            let equation = Equation::from_str(&line).unwrap();

            if equation.solvable() {
                total += equation.test_value;
            }
        }
    }

    println!("solutions: {}", total);
}
