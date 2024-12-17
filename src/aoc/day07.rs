// https://adventofcode.com/2024/day/7

use std::{collections::HashSet, iter::{self}};

use itertools::Itertools;

use super::utils::get_lines;

struct Input {
    equations: Vec<CalibrationEquation>,
}

#[derive(Debug)]
struct CalibrationEquation {
    result: u64,
    terms: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut equations: Vec<CalibrationEquation> = Vec::new();

    for line in lines {
        let line_parts: Vec<&str> = line.split(':').take(2).collect();
        if let [left, right] = &line_parts[..] {
            let result = left.parse::<u64>().unwrap();
            let terms: Vec<u64> = right
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            equations.push(CalibrationEquation { result, terms });
        }
    }

    Input { equations }
}

fn get_total_calibration_result(input_file: &str) -> u64 {
    let input = parse_input(input_file);

    //println!("{:?}", input.equations);

    let mut total_calibration_result = 0;

    let operators = vec![Operator::Add, Operator::Multiply];

    for equation in input.equations {
        let mut cached_results: HashSet<u64> = HashSet::new();

        //println!("{:?}", equation);
        let operator_seqs: Vec<_> = iter::repeat(operators.iter())
            .take(equation.terms.len())
            .multi_cartesian_product()
            .collect();
        
        for operator_seq in operator_seqs {
            //println!("{:?}", operator_seq);
            let result = equation.terms.iter().zip(operator_seq.iter()).fold(0, |acc, (term, operator)| {
                match operator {
                    Operator::Add => acc + term,
                    Operator::Multiply => acc * term,
                }
            });

            if result == equation.result && !cached_results.contains(&result) {
                total_calibration_result += equation.result;
                cached_results.insert(result);
            }
        }
    }

    total_calibration_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_calibration_result_test01() {
        assert_eq!(3749, get_total_calibration_result("input/day07_test01.txt"));
    }

    #[test]
    fn test_get_total_distance() {
        assert_eq!(3245122495150, get_total_calibration_result("input/day07.txt"));
    }
}
