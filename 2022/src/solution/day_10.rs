//! Contains solution for day 10.

use std::str::FromStr;

use thiserror::Error;

use crate::Solver;

/// Error that occurred during parsing of the input
#[derive(Debug, Error)]
pub enum InputError {
    /// Input contains a line that is not a valid instruction
    #[error("Invalid instruction: {0}")]
    InvalidInstruction(String),
}

/// Instruction for the CPU
#[derive(Debug, PartialEq)]
enum Instruction {
    AddX(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[0] {
            "addx" => {
                // Check that there are two parts.
                if parts.len() == 2 {
                    // Parse the number
                    let num: i32 = parts[1]
                        .parse()
                        .map_err(|_| InputError::InvalidInstruction(s.to_string()))?;

                    Ok(Instruction::AddX(num))
                } else {
                    Err(InputError::InvalidInstruction(s.to_string()))
                }
            }
            "noop" => Ok(Instruction::Noop),
            _ => Err(InputError::InvalidInstruction(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Input(Vec<Instruction>);

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<Instruction>, _> = s
            .lines()
            .map(|line| line.trim()) // Trim whitespace
            .filter(|line| *line != "") // Remove empty lines
            .map(|line| line.parse()) // Parse into instruction
            .collect();

        Ok(Input(res?))
    }
}

/// Solution for day 10.
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = String;

    fn get_day(&self) -> u32 {
        10
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        // Current cycle
        let mut cycle = 0;
        // Value in the register
        let mut register = 1;
        // Next cycle during which we are interested in the signal strength
        let mut interesting_cycle = 20;
        // The result of part one.
        let mut result = 0;

        for instruction in input.0.iter() {
            // Get for how much the cycle number needs to increase
            // and for how much the register value needs to increase.
            let (cycle_diff, register_diff) = match instruction {
                Instruction::AddX(val) => (2, *val),
                Instruction::Noop => (1, 0),
            };

            // If during the execution of the instruction the cycle
            // in which we are interested has passed, calculate the strength
            // and update in which cycle we are interested in.
            if cycle + cycle_diff >= interesting_cycle {
                let strength = interesting_cycle * register;
                result += strength;
                interesting_cycle += 40;
            }

            cycle += cycle_diff;
            register += register_diff
        }

        result.to_string()
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        // Row index of the pixel the CRT is drawing.
        let mut crt_index = 0;
        // Value of the register.
        let mut register = 1;
        // Result that is drawn to the string. We start with an empty line, so that the
        // result is printed nicely if run with `cargo run all`.
        let mut result = String::from("\n");

        for instruction in input.0.iter() {
            // Get for how much the cycle number needs to increase
            // and for how much the register value needs to increase.
            let (cycle_diff, register_diff) = match instruction {
                Instruction::AddX(val) => (2, *val),
                Instruction::Noop => (1, 0),
            };

            for _ in 0..cycle_diff {
                // If CRT index is on the sprite, draw a "#", otherwise draw a ".".
                if crt_index >= register - 1 && crt_index <= register + 1 {
                    result += "#";
                } else {
                    result += ".";
                }

                // Increase the crt index and check if we are at the end of the row.
                // If at the end, reset the row index and go into new line.
                crt_index += 1;
                if crt_index == 40 {
                    result += "\n";
                    crt_index = 0;
                }
            }

            // Update the register.
            register += register_diff;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::solution::day_10::{Input, Instruction, Solution};
    use crate::Solver;

    #[test]
    fn parse_input() {
        let input = r#"
            noop
            addx 3
            addx -5
            "#;
        let input = Input::from_str(input).unwrap();
        assert_eq!(
            input,
            Input(vec![
                Instruction::Noop,
                Instruction::AddX(3),
                Instruction::AddX(-5),
            ])
        );
    }

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, "13140");
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(
            solution,
            "\n##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n"
        );
    }

    const INPUT: &str = r#"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
        "#;
}
