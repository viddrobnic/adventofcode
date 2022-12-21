//! Contains solution for day 21.

use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::Solver;

/// Job that a monkey does.
#[derive(Debug, PartialEq, Clone)]
enum Job {
    Number(i64),
    Sum(String, String),
    Difference(String, String),
    Product(String, String),
    Division(String, String),
}

/// Monkey
#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    job: Job,
    contains_human: bool,
}

impl Job {
    /// Gets pair of (left, right) monkey names.
    ///
    /// If monkey represents a number, None is returned.
    fn get_left_right(&self) -> Option<(&str, &str)> {
        match self {
            Job::Number(_) => None,
            Job::Sum(left, right) => Some((left, right)),
            Job::Difference(left, right) => Some((left, right)),
            Job::Product(left, right) => Some((left, right)),
            Job::Division(left, right) => Some((left, right)),
        }
    }
}

/// Error that occurred during parsing of the input.
#[derive(Debug, Error)]
pub enum InputError {
    /// Input contains invalid number.
    #[error("Got invalid number: {source}")]
    InvalidNumber {
        #[from]
        source: ParseIntError,
    },

    /// Input contains invalid operation.
    #[error("Got invalid operation: {0}")]
    InvalidOperation(String),
}

/// Input for the solution that can be parsed from a string.
#[derive(Debug, PartialEq)]
pub struct Input(HashMap<String, Monkey>);

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<HashMap<_, _>, _> = s
            .lines()
            .map(|line| line.trim()) // Trim whitespace
            .filter(|line| *line != "") // Filter out wmpty lines
            .map(|line| {
                // Break line into parts
                let parts: Vec<&str> = line.split_whitespace().collect();

                // Get the name of the monkey.
                let name = parts[0];
                let name = name[..name.len() - 1].to_string();

                let job = if parts.len() == 2 {
                    // If the monkey yells a number, parse the number.
                    let number = parts[1].parse()?;
                    Job::Number(number)
                } else {
                    // Parse the operation that the monkey does.
                    match parts[2] {
                        "+" => Job::Sum(parts[1].to_string(), parts[3].to_string()),
                        "-" => Job::Difference(parts[1].to_string(), parts[3].to_string()),
                        "*" => Job::Product(parts[1].to_string(), parts[3].to_string()),
                        "/" => Job::Division(parts[1].to_string(), parts[3].to_string()),
                        s => return Err(InputError::InvalidOperation(s.to_string())),
                    }
                };

                // Construct the monkey.
                Ok((
                    name,
                    Monkey {
                        job,
                        contains_human: false,
                    },
                ))
            })
            .collect();

        Ok(Input(res?))
    }
}

/// Calculates the number that the specified monkey yells.
fn calculate_numbers(monkeys: &HashMap<String, Monkey>, monkey_name: &str) -> i64 {
    let monkey = monkeys
        .get(monkey_name)
        .expect(&format!("Invalid monkey: {}", monkey_name));

    // Recursively calculate the number based on the monkey's job.
    let number = match &monkey.job {
        Job::Number(n) => *n,
        Job::Sum(left, right) => {
            calculate_numbers(monkeys, left) + calculate_numbers(monkeys, right)
        }
        Job::Difference(left, right) => {
            calculate_numbers(monkeys, left) - calculate_numbers(monkeys, right)
        }
        Job::Product(left, right) => {
            calculate_numbers(monkeys, left) * calculate_numbers(monkeys, right)
        }
        Job::Division(left, right) => {
            calculate_numbers(monkeys, left) / calculate_numbers(monkeys, right)
        }
    };

    number
}

/// Calculate if the given monkey contains the humn monkey.
///
/// Function is used to update update the monkeys so that we know which monkeys contain the humn
/// in the future.
fn calculate_contains_human(monkeys: &mut HashMap<String, Monkey>, monkey_name: &str) -> bool {
    let monkey = monkeys
        .get_mut(monkey_name)
        .expect(&format!("Invalid monkey: {}", monkey_name));

    if monkey_name == "humn" {
        monkey.contains_human = true;
        return true;
    }

    let job = monkey.job.clone();
    let contains_human = match &job {
        Job::Number(_) => false,
        Job::Sum(left, right)
        | Job::Difference(left, right)
        | Job::Product(left, right)
        | Job::Division(left, right) => {
            calculate_contains_human(monkeys, left) || calculate_contains_human(monkeys, right)
        }
    };

    let monkey = monkeys.get_mut(monkey_name).unwrap();
    monkey.contains_human = contains_human;
    contains_human
}

/// Return type for getting the monkey with the unknown variable.
enum Unknown<'a> {
    Left {
        known_monkey_value: i64,
        unknown_name: &'a str,
    },
    Right {
        known_monkey_value: i64,
        unknown_name: &'a str,
    },
}

/// Gets the monkey with the unknown variable.
fn get_variable<'a>(
    monkeys: &'a HashMap<String, Monkey>,
    left: &'a str,
    right: &'a str,
) -> Unknown<'a> {
    let left_monkey = monkeys.get(left).unwrap();
    let right_monkey = monkeys.get(right).unwrap();

    if left_monkey.contains_human && right_monkey.contains_human {
        panic!("Multiple unknown variables!");
    }

    if !left_monkey.contains_human && !right_monkey.contains_human {
        panic!("No unknown variables!");
    }

    if left_monkey.contains_human {
        Unknown::Left {
            known_monkey_value: calculate_numbers(monkeys, right),
            unknown_name: left,
        }
    } else {
        Unknown::Right {
            known_monkey_value: calculate_numbers(monkeys, left),
            unknown_name: right,
        }
    }
}

/// Solve the equation for part two recursively.
fn solve_equation(monkeys: &HashMap<String, Monkey>, left: i64, monkey_name: &str) -> i64 {
    let monkey = monkeys
        .get(monkey_name)
        .expect(&format!("Invalid monkey: {}", monkey_name));

    if monkey_name == "humn" {
        return left;
    }

    match &monkey.job {
        Job::Number(_) => unreachable!(),
        Job::Sum(left_m, right_m) => match get_variable(monkeys, left_m, right_m) {
            Unknown::Left {
                unknown_name,
                known_monkey_value,
            }
            | Unknown::Right {
                unknown_name,
                known_monkey_value,
            } => solve_equation(monkeys, left - known_monkey_value, unknown_name),
        },
        Job::Difference(left_m, right_m) => match get_variable(monkeys, left_m, right_m) {
            Unknown::Left {
                unknown_name,
                known_monkey_value,
            } => solve_equation(monkeys, left + known_monkey_value, unknown_name),
            Unknown::Right {
                unknown_name,
                known_monkey_value,
            } => solve_equation(monkeys, known_monkey_value - left, unknown_name),
        },
        Job::Product(left_m, right_m) => match get_variable(monkeys, left_m, right_m) {
            Unknown::Left {
                unknown_name,
                known_monkey_value,
            }
            | Unknown::Right {
                unknown_name,
                known_monkey_value,
            } => solve_equation(monkeys, left / known_monkey_value, unknown_name),
        },
        Job::Division(left_m, right_m) => match get_variable(monkeys, left_m, right_m) {
            Unknown::Left {
                unknown_name,
                known_monkey_value,
            } => solve_equation(monkeys, left * known_monkey_value, unknown_name),
            Unknown::Right {
                unknown_name,
                known_monkey_value,
            } => solve_equation(monkeys, known_monkey_value / left, unknown_name),
        },
    }
}

/// Implements solution for day 21.
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = i64;

    fn get_day(&self) -> u32 {
        21
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        let mut monkeys = input.0.clone();
        calculate_numbers(&mut monkeys, "root")
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        let mut monkeys = input.0.clone();
        let _ = calculate_contains_human(&mut monkeys, "root");

        let root_monkey = monkeys.get("root").unwrap();
        let (left, right) = root_monkey.job.get_left_right().unwrap();

        match get_variable(&monkeys, left, right) {
            Unknown::Left {
                unknown_name,
                known_monkey_value,
            } => solve_equation(&monkeys, known_monkey_value, unknown_name),
            Unknown::Right {
                unknown_name,
                known_monkey_value,
            } => solve_equation(&monkeys, known_monkey_value, unknown_name),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::str::FromStr;

    use crate::solution::day_21::{Input, Job, Monkey, Solution};
    use crate::Solver;

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 152);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 301);
    }

    #[test]
    fn parse_input() {
        let input = Input::from_str(
            "root: pppw + sjmn
                dbpl: 5",
        )
        .unwrap();

        let mut map = HashMap::new();
        map.insert(
            "root".to_string(),
            Monkey {
                job: Job::Sum("pppw".to_string(), "sjmn".to_string()),
                contains_human: false,
            },
        );
        map.insert(
            "dbpl".to_string(),
            Monkey {
                job: Job::Number(5),
                contains_human: false,
            },
        );
        assert_eq!(input, Input(map));
    }

    const INPUT: &str = r#"
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32
        "#;
}
