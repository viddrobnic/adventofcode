//! Contains solution for day 11.
use std::str::FromStr;
use thiserror::Error;

use crate::Solver;

/// Represents a single monkey and its state.
#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    /// Items that the monkey is holding
    items: Vec<u64>,
    /// Operation that is done for each item
    operation: Operation,
    /// Test that is done before throwing an item to the next monkey.
    test: Test,
}

/// Operation done on an item.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    /// Multiplication of two operands
    Multiply(Operand, Operand),
    /// Sum of two operands
    Sum(Operand, Operand),
}

impl Operation {
    /// Evaluate a new value of the operation with the given old value.
    fn evaluate(&self, old: u64) -> u64 {
        match self {
            Operation::Multiply(left, right) => left.value(old) * right.value(old),
            Operation::Sum(left, right) => left.value(old) + right.value(old),
        }
    }
}

/// A single operand in an operation.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Operand {
    /// A constant number
    Constant(u64),
    /// Placeholder for the old value
    Old,
}

impl Operand {
    /// Returns the value of the operand given the old value.
    fn value(&self, old: u64) -> u64 {
        match self {
            Operand::Constant(constant) => *constant,
            Operand::Old => old,
        }
    }
}

/// Test done by the monkey on an item.
#[derive(Debug, PartialEq, Clone, Copy)]
struct Test {
    /// Monkey checks if the item is divisible by this value
    divisible_by: u32,
    /// To which monkey the item is thrown if the test is true.
    on_true: u32,
    /// To which monkey the item is thrown if the test is false.
    on_false: u32,
}

impl Test {
    /// Returns to which monkey an item with the given value is thrown.
    fn test(&self, value: u64) -> u32 {
        if value % (self.divisible_by as u64) == 0 {
            self.on_true
        } else {
            self.on_false
        }
    }
}

/// Error that occurred during the parsing of the input
#[derive(Debug, Error)]
pub enum InputError {
    /// Input contains invalid number of non empty lines.
    ///
    /// Number of non empty lines in the input should be divisible by 6.
    #[error("Input contains invalid number of non empty lines: {0}. Number of non empty lines should be divisible by 6.")]
    InvalidNumberOfLines(usize),

    /// Number of lines for a single monkey is invalid.
    ///
    /// There should be 6 lines of input for every monkey.
    #[error("Number of lines for a monkey is invalid: {0}. Expected 6 lines of input.")]
    InvalidNumberOfMonkeyLines(usize),

    /// Line representing starting items is invalid.
    #[error("Line representing starting items is invalid: {0}")]
    InvalidStartingItems(String),

    /// Line representing operation is invalid.
    #[error("Line representing operation is invalid: {0}")]
    InvalidOperation(String),

    /// Operand in the operation line is invalid.
    #[error("Operation contains an invalid operand: {0}")]
    InvalidOperand(String),

    /// Line representing the monkey test is invalid.
    #[error("Line representing the monkey test is invalid: {0}")]
    InvalidTest(String),
}

/// Input for the solution that can be parsed from a string.
#[derive(Debug, PartialEq)]
pub struct Input(Vec<Monkey>);

impl FromStr for Operand {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Old),
            val => {
                let num = val
                    .parse()
                    .map_err(|_| InputError::InvalidOperand(s.to_string()))?;
                Ok(Operand::Constant(num))
            }
        }
    }
}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s
            .lines()
            .map(|line| line.trim()) // Trim whitespace
            .filter(|line| *line != "") // Filter out empty lines
            .collect();

        // Check that the number of non empty lines is correct.
        if lines.len() % 6 != 0 {
            return Err(InputError::InvalidNumberOfLines(lines.len()));
        }

        // Initialize the monkeys vector.
        let mut monkeys = Vec::with_capacity(lines.len() / 6);

        // Parse the monkeys
        for monkey in 0..lines.len() / 6 {
            let m_index = monkey * 6;
            monkeys.push(Monkey::from_lines(&lines[m_index..m_index + 6])?);
        }

        Ok(Input(monkeys))
    }
}

impl Monkey {
    fn from_lines(lines: &[&str]) -> Result<Self, InputError> {
        // Check that the number of lines is correct
        if lines.len() != 6 {
            return Err(InputError::InvalidNumberOfMonkeyLines(lines.len()));
        }

        // Parse starting items.
        let parts: Vec<&str> = lines[1].split(":").collect();
        if parts.len() != 2 {
            return Err(InputError::InvalidStartingItems(lines[1].to_string()));
        }

        // Get the numbers separated by commas, and convert the to integers.
        let numbers: Result<Vec<u64>, _> =
            parts[1].split(",").map(|num| num.trim().parse()).collect();
        let numbers = match numbers {
            Ok(val) => val,
            Err(_) => return Err(InputError::InvalidStartingItems(lines[1].to_string())),
        };

        // Parse the operation
        // First find the index of "="
        let split_index = lines[2]
            .chars()
            .position(|c| c == '=')
            .ok_or(InputError::InvalidOperation(lines[2].to_string()))?;

        // Get the parts of the operation after the "="
        let parts: Vec<&str> = lines[2][split_index + 1..]
            .trim()
            .split_whitespace()
            .collect();

        // Check if number of parts is correct.
        if parts.len() != 3 {
            return Err(InputError::InvalidOperation(lines[2].to_string()));
        }

        // Parse left and right operand.
        let left = Operand::from_str(parts[0])?;
        let right = Operand::from_str(parts[2])?;

        // Parse the operation itself.
        let operation = match parts[1] {
            "+" => Operation::Sum(left, right),
            "*" => Operation::Multiply(left, right),
            _ => return Err(InputError::InvalidOperation(lines[2].to_string())),
        };

        // Parse the test
        let divisible_by = parse_test_line(lines[3])?;
        let on_true = parse_test_line(lines[4])?;
        let on_false = parse_test_line(lines[5])?;

        // Construct the monkey
        Ok(Monkey {
            items: numbers,
            operation,
            test: Test {
                divisible_by,
                on_true,
                on_false,
            },
        })
    }
}

/// Parses a single line of the monkey test and returns the number in it.
fn parse_test_line(line: &str) -> Result<u32, InputError> {
    line.split_whitespace()
        .last()
        .ok_or(InputError::InvalidTest(line.to_string()))?
        .parse()
        .map_err(|_| InputError::InvalidTest(line.to_string()))
}

/// Solution for day 11.
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = u64;

    fn get_day(&self) -> u32 {
        11
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        calculate_monkey_business(&input.0, 3, 20)
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        calculate_monkey_business(&input.0, 1, 10000)
    }
}

/// Calculates the monkey business from the given monkeys.
///
/// It takes the following parameters:
/// - `monkeys`: monkeys for which the monkey business is calculated
/// - `worry_level_division`: number by which the worry level is divided after each inspection
/// - `nr_rounds`: number of rounds that are simulated
fn calculate_monkey_business(
    monkeys: &Vec<Monkey>,
    worry_level_division: u64,
    nr_rounds: usize,
) -> u64 {
    // Clone the monkeys so we can change them in place.
    let mut monkeys = monkeys.clone();
    // Initialize monkey business vector. This holds number of items each monkey has inspected.
    let mut monkey_business = vec![0; monkeys.len()];
    // Calculate the modulo by which the worry level should be calculated.
    let monkey_modulo = calculate_monkey_modulo(&monkeys);

    // Go through all the rounds.
    for _ in 0..nr_rounds {
        // Go through all the monkeys.
        for index in 0..monkeys.len() {
            // Go through all the items that the monkey holds.
            while !monkeys[index].items.is_empty() {
                // Increase number of items the monkey has inspected.
                monkey_business[index] += 1;

                // Calculate the worry level of the next item.
                let item = monkeys[index].items.remove(0);
                let item = monkeys[index].operation.evaluate(item) / worry_level_division;
                let item = item % monkey_modulo;

                // Find which monkey the item is passed to.
                let to_monkey = monkeys[index].test.test(item) as usize;
                // Add the item to that monkey.
                monkeys[to_monkey].items.push(item);
            }
        }
    }

    // Find the two most active numbers and calculate the monkey business.
    // This is done with sorting because it's less code than quick select and the number
    // of monkeys is so low, that it doesn't play a difference in execution time.
    monkey_business.sort_unstable();
    let len = monkey_business.len();
    monkey_business[len - 1] * monkey_business[len - 2]
}

/// Calculate the modulo by which the worry level should be calculated.
///
/// Modulo is calculated by multiplying together all the divisibility tests.
/// This works because the divisibility tests are prime numbers.
fn calculate_monkey_modulo(monkeys: &Vec<Monkey>) -> u64 {
    monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by as u64)
        .product()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::solution::day_11::{Input, Monkey, Operand, Operation, Solution, Test};
    use crate::Solver;

    const INPUT: &str = r#"
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
        
        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0
        
        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3
        
        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
        "#;

    #[test]
    fn parse_input() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!(
            input,
            Input(vec![
                Monkey {
                    items: vec![79, 98],
                    operation: Operation::Multiply(Operand::Old, Operand::Constant(19)),
                    test: Test {
                        divisible_by: 23,
                        on_true: 2,
                        on_false: 3,
                    },
                },
                Monkey {
                    items: vec![54, 65, 75, 74],
                    operation: Operation::Sum(Operand::Old, Operand::Constant(6)),
                    test: Test {
                        divisible_by: 19,
                        on_true: 2,
                        on_false: 0,
                    },
                },
                Monkey {
                    items: vec![79, 60, 97],
                    operation: Operation::Multiply(Operand::Old, Operand::Old),
                    test: Test {
                        divisible_by: 13,
                        on_true: 1,
                        on_false: 3,
                    },
                },
                Monkey {
                    items: vec![74],
                    operation: Operation::Sum(Operand::Old, Operand::Constant(3)),
                    test: Test {
                        divisible_by: 17,
                        on_true: 0,
                        on_false: 1,
                    },
                },
            ])
        );
    }

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 10605);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 2713310158);
    }
}
