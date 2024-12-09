use std::{fs::File, io::{self, BufRead}, str::FromStr};
use rust_7::{Equation, MathOperator::{self, Add, Multiply, Concat}};

fn solve_file(operators: Vec<MathOperator>) -> u64 {
    let file = File::open("./input").unwrap();
    let mut total = 0;

    for line_result in io::BufReader::new(file).lines() {
        if let Ok(line) = line_result {
            let equation = Equation::from_str(&line).unwrap();

            if equation.solvable(&operators) {
                total += equation.test_value;
            }
        }
    }

    total
}

fn main() {
    println!("part 1 solution: {}", solve_file(vec![Add, Multiply]));
    println!("part 2 solution: {}", solve_file(vec![Add, Multiply, Concat]));
}
