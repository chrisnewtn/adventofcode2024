use std::{num::ParseIntError, str::FromStr};

#[derive(PartialEq, Clone, Debug)]
enum MathOperator {
    Add,
    Multiply
}

impl MathOperator {
    pub fn from_usize(i: &usize) -> Option<Self> {
        match i {
            0 => Some(MathOperator::Add),
            1 => Some(MathOperator::Multiply),
            _ => None,
        }
    }
}

fn operate(left: u64, operator: &MathOperator, right: u64) -> u64 {
    match operator {
        MathOperator::Add => left + right,
        MathOperator::Multiply => left * right,
    }
}

fn add_one(digits: &mut Vec<usize>, radix: usize) -> bool {
    for i in (0..digits.len()).rev() {
        if digits[i] < radix - 1 {
            digits[i] += 1;
            for y in (i+1)..digits.len() {
                digits[y] = 0;
            }
            return true;
        }
    }
    false
}

fn combination_grid(width: usize) -> Vec<Vec<usize>> {
    let mut combinations = vec![vec![0; width]];

    loop {
        let mut row = combinations.last().unwrap().clone();

        if add_one(&mut row, 2) {
            combinations.push(row);
        } else {
            break;
        }
    }

    combinations
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseEquationError {
    NoTestValue,
    NoNumbers,
    InvalidTestValue(ParseIntError),
    InvalidNumber(ParseIntError),
}

#[derive(Debug)]
pub struct Equation {
    pub test_value: u64,
    pub numbers: Vec<u32>,
}

impl Equation {
    pub fn solvable(&self) -> bool {
        let operators: Vec<Vec<MathOperator>> = combination_grid(self.numbers.len() - 1)
            .iter()
            .map(|row| row.iter().map(|i| MathOperator::from_usize(&i).unwrap()).collect())
            .collect();

        for mut operator_row in operators {
            let mut total: Option<u64> = None;

            for i in 1..self.numbers.len() {
                let left = total.or(Some(self.numbers[i - 1].into())).unwrap();
                let operator = operator_row.pop().unwrap();
                let right = self.numbers[i].into();

                let result = operate(left, &operator, right);

                total.replace(result);
            }

            if total == Some(self.test_value) {
                return true;
            }
        }

        false
    }
}

impl FromStr for Equation {
    type Err = ParseEquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(":");
        let mut numbers: Vec<u32> = Vec::new();

        let test_value_string = parts.next().unwrap();

        let test_value = match test_value_string.parse() {
            Ok(number) => number,
            Err(err) => return Err(ParseEquationError::InvalidTestValue(err))
        };

        let number_strings = parts.next();

        if None == number_strings {
            return Err(ParseEquationError::NoNumbers);
        }

        for number_string in number_strings.unwrap().trim().split(" ") {
            match number_string.parse::<u32>() {
                Ok(number) => numbers.push(number),
                Err(err) => return Err(ParseEquationError::InvalidNumber(err))
            };
        }

        Ok(Self {
            test_value,
            numbers
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> String {
        "190: 10 19
         3267: 81 40 27
         83: 17 5
         156: 15 6
         7290: 6 8 6 15
         161011: 16 10 13
         192: 17 8 14
         21037: 9 7 18 13
         292: 11 6 16 20".to_string()
    }

    #[test]
    fn parses_an_equation_row() {
        for line in fixture().lines() {
            Equation::from_str(line).unwrap();
        }
    }

    #[test]
    fn is_able_to_solve_when_solvable() {
        let equation = Equation {
            test_value: 190,
            numbers: vec![10, 19]
        };

        assert!(equation.solvable());
    }

    #[test]
    fn cannot_solve_when_unsolvable() {
        let equation = Equation {
            test_value: 192,
            numbers: vec![17, 8, 14]
        };

        assert_eq!(equation.solvable(), false);
    }

    #[test]
    fn finds_three_solutions_in_the_fixture() {
        let mut solutions = 0;

        for line in fixture().lines() {
            if let Ok(equation) = Equation::from_str(line) {
                if equation.solvable() {
                    solutions += 1;
                }
            }
        }

        assert_eq!(solutions, 3);
    }

    #[test]
    fn errors_on_empty_string() {
        let res = Equation::from_str("");
        assert!(!res.is_ok());
    }

    #[test]
    fn errors_on_no_test_value() {
        let res = Equation::from_str(":12, 42");
        assert!(!res.is_ok());
    }

    #[test]
    fn errors_on_no_numbers() {
        let res = Equation::from_str("123");
        assert_eq!(res.unwrap_err(), ParseEquationError::NoNumbers);
    }

    #[test]
    fn errors_on_unparsable_numbers() {
        let res = Equation::from_str("123: twelve, 42");
        assert!(!res.is_ok());
    }
}
