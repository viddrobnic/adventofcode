//! Contains the solution for day 4.

use crate::Solver;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

// Represents an interval.
#[derive(Debug, PartialEq, Eq)]
struct Assignment {
    start: i32,
    end: i32,
}

impl Assignment {
    // Returns weather `self` contains `other`.
    fn contains(&self, other: &Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    // Returns weather the intervals `self` and `other` overlap.
    fn overlaps(&self, other: &Self) -> bool {
        self.contains_point(other.start)
            || self.contains_point(other.end)
            || other.contains_point(self.start)
            || other.contains_point(self.end)
    }

    // Returns weather the interval `self` contains a point.
    fn contains_point(&self, point: i32) -> bool {
        point >= self.start && point <= self.end
    }
}

/// Error that occurred during parsing of the input.
#[derive(Debug, Error)]
pub enum InputError {
    /// The input contains a line that has invalid number of parts.
    #[error("invalid number of parts on the input (expected {expected}, actual {actual})")]
    InvalidNumberOfParts { actual: usize, expected: usize },

    /// The input contains an invalid sector
    #[error("invalid sector given")]
    InvalidSector {
        #[from]
        source: ParseIntError,
    },
}

/// Input for the solution that can be parsed from a string.
#[derive(Debug, PartialEq)]
pub struct Input(Vec<(Assignment, Assignment)>);

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<(Assignment, Assignment)>, Self::Err> = s
            .lines() // Split to lines
            .map(|val| val.trim()) // Trim whitespace
            .filter(|val| *val != "") // Skip empty lines
            .map(|line: &str| {
                // Get the pair of assignments
                let parts: Result<Vec<Assignment>, _> = line
                    .split(",") // Split by ","
                    .map(|part: &str| {
                        // Map each part into an Assignment.
                        // First split to individual numbers and parse them to ints
                        let res: Result<Vec<i32>, ParseIntError> = part
                            .split("-")
                            .map(|val: &str| val.parse::<i32>())
                            .collect();

                        // Handle errors and invalid number of parts
                        let res = res?;
                        if res.len() != 2 {
                            return Err(InputError::InvalidNumberOfParts {
                                actual: res.len(),
                                expected: 2,
                            });
                        }

                        // Convert number into an assignment
                        Ok(Assignment {
                            start: res[0],
                            end: res[1],
                        })
                    })
                    .collect();

                // Handle errors and invalid number of parts
                let mut parts = parts?;
                if parts.len() != 2 {
                    return Err(InputError::InvalidNumberOfParts {
                        actual: parts.len(),
                        expected: 2,
                    });
                }

                // Convert into pair of assignments without cloning
                let part2 = parts.pop().unwrap();
                let part1 = parts.pop().unwrap();
                Ok((part1, part2))
            })
            .collect();

        // Return the final input
        Ok(Input(res?))
    }
}

/// Solution for day 4.
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = i32;

    fn get_day(&self) -> u32 {
        4
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        input
            .0
            .iter()
            .fold(0, |acc, (ass1, ass2): &(Assignment, Assignment)| {
                if ass1.contains(ass2) || ass2.contains(ass1) {
                    acc + 1
                } else {
                    acc
                }
            })
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        input.0.iter().fold(
            0,
            |acc, (ass1, ass2): &(Assignment, Assignment)| {
                if ass1.overlaps(ass2) {
                    acc + 1
                } else {
                    acc
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::day_04::{Assignment, Input, Solution};
    use crate::Solver;
    use std::str::FromStr;

    const INPUT: &str = r#"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
        "#;

    #[test]
    fn parse_input() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!(
            input,
            Input(vec![
                (
                    Assignment { start: 2, end: 4 },
                    Assignment { start: 6, end: 8 },
                ),
                (
                    Assignment { start: 2, end: 3 },
                    Assignment { start: 4, end: 5 },
                ),
                (
                    Assignment { start: 5, end: 7 },
                    Assignment { start: 7, end: 9 },
                ),
                (
                    Assignment { start: 2, end: 8 },
                    Assignment { start: 3, end: 7 },
                ),
                (
                    Assignment { start: 6, end: 6 },
                    Assignment { start: 4, end: 6 },
                ),
                (
                    Assignment { start: 2, end: 6 },
                    Assignment { start: 4, end: 8 },
                ),
            ])
        )
    }

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 2);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 4);
    }
}
