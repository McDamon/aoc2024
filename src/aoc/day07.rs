// https://adventofcode.com/2024/day/7

use std::iter::{self};

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
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            equations.push(CalibrationEquation { result, terms });
        }
    }

    Input { equations }
}

fn is_valid_calibration_result(equation: &CalibrationEquation) -> bool {
    let operators = [Operator::Add, Operator::Multiply];

    let operator_seqs: Vec<_> = iter::repeat(operators.iter())
        .take(equation.terms.len())
        .multi_cartesian_product()
        .collect();

    let first_val = *equation.terms.first().unwrap_or(&0);

    for operator_seq in operator_seqs {
        let result = equation.terms.iter().zip(operator_seq.iter()).skip(1).fold(
            first_val,
            |acc, (term, operator)| match operator {
                Operator::Add => {
                    //println!("{} + {} = {}", acc, term, acc + term);
                    acc + term
                }
                Operator::Multiply => {
                    //println!("{} * {} = {}", acc, term, acc * term);
                    acc * term
                }
            },
        );

        if result == equation.result {
            //println!("{} == {}", result, equation.result);
            return true;
        } else {
            //println!("{} != {}", result, equation.result);
        }
    }

    false
}

fn get_total_calibration_result(input_file: &str) -> u64 {
    let input = parse_input(input_file);

    let mut total_calibration_result = 0;

    for equation in input.equations {
        //println!("{:?}", equation);
        if is_valid_calibration_result(&equation) {
            total_calibration_result += equation.result;
        }
    }

    total_calibration_result
}

fn get_total_calibration_result_with_concat(input_file: &str) -> u64 {
    let input = parse_input(input_file);

    let mut total_calibration_result = 0;

    let operators = [Operator::Add, Operator::Multiply];

    for equation in input.equations {
        if is_valid_calibration_result(&equation) {
            total_calibration_result += equation.result;
        } else {
            let operator_seqs: Vec<_> = iter::repeat(operators.iter())
                .take(equation.terms.len())
                .multi_cartesian_product()
                .collect();

            for operator_seq in operator_seqs {
                for i in 0..operator_seq.len() {
                    let (left_operator_seq, right_operator_seq) = operator_seq.split_at(i);
                    let (left_terms, right_terms) = equation.terms.split_at(i);

                    let left_result = left_terms.iter().zip(left_operator_seq.iter()).fold(
                        0,
                        |acc, (term, operator)| match operator {
                            Operator::Add => acc + term,
                            Operator::Multiply => acc * term,
                        },
                    );

                    let right_result = right_terms
                        .iter()
                        .enumerate()
                        .map(|(index, &term)| {
                            if index == 0 {
                                (left_result.to_string() + &term.to_string())
                                    .parse()
                                    .unwrap()
                            } else {
                                term
                            }
                        })
                        .zip(right_operator_seq.iter())
                        .fold(0, |acc, (term, operator)| match operator {
                            Operator::Add => acc + term,
                            Operator::Multiply => acc * term,
                        });

                    if right_result == equation.result {
                        total_calibration_result += right_result;
                    }
                }
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
    fn test_get_total_calibration() {
        assert_eq!(
            3245122495150,
            get_total_calibration_result("input/day07.txt")
        );
    }

    #[test]
    fn test_get_total_calibration_result_test02() {
        assert_eq!(0, get_total_calibration_result("input/day07_test02.txt"));
    }

    #[test]
    fn test_get_total_calibration_result_test03() {
        assert_eq!(0, get_total_calibration_result("input/day07_test03.txt"));
    }

    #[test]
    fn test_get_total_calibration_result_test04() {
        assert_eq!(0, get_total_calibration_result("input/day07_test04.txt"));
    }

    #[test]
    fn test_get_total_calibration_result_test05() {
        assert_eq!(0, get_total_calibration_result("input/day07_test05.txt"));
    }

    #[test]
    fn test_get_total_calibration_result_test06() {
        assert_eq!(0, get_total_calibration_result("input/day07_test06.txt"));
    }

    #[test]
    fn test_get_total_calibration_result_test07() {
        assert_eq!(3267, get_total_calibration_result("input/day07_test07.txt"));
    }

    #[test]
    fn test_get_total_calibration_result_with_concat_test01() {
        assert_eq!(
            11387,
            get_total_calibration_result_with_concat("input/day07_test01.txt")
        );
    }

    #[test]
    fn test_get_total_calibration_result_with_concat_test02() {
        assert_eq!(
            156,
            get_total_calibration_result_with_concat("input/day07_test02.txt")
        );
    }

    #[test]
    fn test_get_total_calibration_result_with_concat_test03() {
        assert_eq!(
            7290,
            get_total_calibration_result_with_concat("input/day07_test03.txt")
        );
    }

    #[test]
    fn test_get_total_calibration_result_with_concat_test04() {
        assert_eq!(
            192,
            get_total_calibration_result_with_concat("input/day07_test04.txt")
        );
    }

    #[test]
    fn test_get_total_calibration_result_with_concat_test05() {
        assert_eq!(
            12,
            get_total_calibration_result_with_concat("input/day07_test05.txt")
        );
    }

    #[test]
    fn test_get_total_calibration_result_with_concat_test06() {
        assert_eq!(
            507905413443,
            get_total_calibration_result_with_concat("input/day07_test06.txt")
        );
    }

    #[test]
    fn test_get_total_calibration_result_with_concat_test07() {
        assert_eq!(
            3267,
            get_total_calibration_result_with_concat("input/day07_test07.txt")
        );
    }

    #[test]
    fn test_get_total_distance_with_concat() {
        assert_eq!(
            0,
            get_total_calibration_result_with_concat("input/day07.txt")
        );
    }
}
