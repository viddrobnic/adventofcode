//! Contains the solution for day 5.

use crate::Solver;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
struct Stack(Vec<char>);

#[derive(Debug, PartialEq)]
struct Instruction {
    // Number of crates to move
    quantity: i32,
    // From which stack they should be moved
    from: i32,
    // To which stack they should be moved
    to: i32,
}

/// Error that occurred during parsing of the input.
#[derive(Debug, Error)]
pub enum InputError {
    /// The input contains an instruction that has invalid number of parts.
    #[error("invalid number of parts for instruction (expected {expected}, actual {actual}")]
    InvalidNumberOfInstructionParts { actual: usize, expected: usize },

    /// The input contains an instruction that has an invalid number
    #[error("invalid number in the instructions")]
    InvalidInstructionNumber {
        #[from]
        source: ParseIntError,
    },
}

/// Input for the solution that can be parsed from a string.
#[derive(Debug, PartialEq)]
pub struct Input {
    stacks: Vec<Stack>,
    instructions: Vec<Instruction>,
}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1. PARSE THE STACKS
        // Get only those lines in the input that are associated with the stacks.
        let stack_lines: Vec<&str> = s
            .lines()
            .filter(|val: &&str| !val.starts_with("move"))
            .filter(|val| *val != "")
            .filter(|val: &&str| !val.contains("1"))
            .collect();

        // Get the number of stacks. We have to iterate through all the stack lines,
        // since the input is trimmed
        // n is number of stacks, w is length of the line.
        // 3 * n + (n-1) = w
        // 4 * n = w + 1
        // n = (w + 1) / 4
        let nr_stacks: usize = stack_lines
            .iter()
            .map(|line: &&str| (line.len() + 1) / 4)
            .max()
            .unwrap();

        // Initialize input struct.
        let mut input = Input {
            stacks: Vec::with_capacity(nr_stacks),
            instructions: vec![],
        };
        for _ in 0..nr_stacks {
            input.stacks.push(Stack(vec![]));
        }

        for line in stack_lines {
            for stack in 0..nr_stacks {
                // Get the starting index of the stack in the line
                let start = stack * 4;
                // Get the ending index of the stack in the line.
                let end = start + 3;

                // Handle indices outside the line, since the lines are trimmed.
                let elt = if start < line.len() && end <= line.len() {
                    // If both indices are inside the line, return the substring
                    &line[start..end]
                } else {
                    // If indices are outside the line, return the substring
                    // representing no crate.
                    "   "
                };

                // If there is no crate, do nothing.
                if elt == "   " {
                    continue;
                }

                // Get the character representing the crate and add it to the correct stack.
                let elt = elt.chars().nth(1).unwrap();
                input.stacks[stack].0.insert(0, elt);
            }
        }

        // 2. PARSE THE INSTRUCTIONS.
        let res: Result<Vec<Instruction>, _> = s
            .lines()
            .filter(|val: &&str| val.starts_with("move")) // Extract only the instruction lines
            .map(|line: &str| {
                // Extract number from the instruction line.
                // This can be done by getting every other "word" and parsing it to int.
                let res: Result<Vec<i32>, _> = line
                    .split_whitespace()
                    .skip(1)
                    .step_by(2)
                    .map(|val: &str| val.parse())
                    .collect();

                // Handle int parsing errors.
                let numbers = match res {
                    Err(err) => return Err(InputError::InvalidInstructionNumber { source: err }),
                    Ok(val) => val,
                };

                // Handle incorrect number of number in an instruction.
                if numbers.len() != 3 {
                    return Err(InputError::InvalidNumberOfInstructionParts {
                        actual: numbers.len(),
                        expected: 3,
                    });
                }

                // Construct the instruction.
                Ok(Instruction {
                    quantity: numbers[0],
                    from: numbers[1],
                    to: numbers[2],
                })
            })
            .collect();

        input.instructions = res?;
        Ok(input)
    }
}

/// Solution for day 5.
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = String;

    fn get_day(&self) -> u32 {
        5
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        let mut stacks = input.stacks.clone();
        for instruction in input.instructions.iter() {
            for _ in 0..instruction.quantity {
                // Move element to the top of the other stack.
                let elt = stacks[instruction.from as usize - 1].0.pop().unwrap();
                stacks[instruction.to as usize - 1].0.push(elt);
            }
        }

        top_of_the_stacks(&stacks)
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        let mut stacks = input.stacks.clone();
        for instruction in input.instructions.iter() {
            for i in 0..instruction.quantity {
                let elt = stacks[instruction.from as usize - 1].0.pop().unwrap();

                // Move element to the bottom of previously moved elements for this stack.
                let stack = &mut stacks[instruction.to as usize - 1].0;
                stack.insert(stack.len() - i as usize, elt);
            }
        }

        top_of_the_stacks(&stacks)
    }
}

// Returns the top of the stacks
fn top_of_the_stacks(stacks: &Vec<Stack>) -> String {
    stacks
        .iter()
        .map(|stack: &Stack| stack.0.last().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::solution::day_05::{Input, Instruction, Solution, Stack};
    use crate::Solver;
    use std::str::FromStr;

    const INPUT: &str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn parse_input() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!(
            input,
            Input {
                stacks: vec![
                    Stack(vec!['Z', 'N']),
                    Stack(vec!['M', 'C', 'D']),
                    Stack(vec!['P']),
                ],
                instructions: vec![
                    Instruction {
                        quantity: 1,
                        from: 2,
                        to: 1,
                    },
                    Instruction {
                        quantity: 3,
                        from: 1,
                        to: 3,
                    },
                    Instruction {
                        quantity: 2,
                        from: 2,
                        to: 1,
                    },
                    Instruction {
                        quantity: 1,
                        from: 1,
                        to: 2,
                    },
                ],
            }
        );
    }

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, "CMZ");
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, "MCD");
    }
}
