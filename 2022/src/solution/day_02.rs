//! Contains the solution for day 2

use crate::solution::day_02::InputError::InvalidNumberOfParts;
use crate::Solver;
use std::str::FromStr;
use thiserror::Error;

// Represents a shape
#[derive(PartialEq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    // Returns the score associated with a shape
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

// Represents the outcome of a single round.
#[derive(PartialEq, Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    // Represents  the score associated with an outcome.
    fn score(&self) -> i32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

// Represents a single round of a game.
#[derive(PartialEq, Debug)]
struct Round {
    // The shape that the opponent plays.
    opponent: Shape,

    // The shape that I play or the outcome that I want.
    me: (Shape, Outcome),
}

impl Round {
    // Get outcome from the shape that the opponent plays and the shape that I play.
    fn outcome(&self) -> Outcome {
        match (&self.opponent, &self.me.0) {
            (Shape::Rock, Shape::Rock) => Outcome::Draw,
            (Shape::Rock, Shape::Paper) => Outcome::Win,
            (Shape::Rock, Shape::Scissors) => Outcome::Lose,
            (Shape::Paper, Shape::Rock) => Outcome::Lose,
            (Shape::Paper, Shape::Paper) => Outcome::Draw,
            (Shape::Paper, Shape::Scissors) => Outcome::Win,
            (Shape::Scissors, Shape::Rock) => Outcome::Win,
            (Shape::Scissors, Shape::Paper) => Outcome::Lose,
            (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        }
    }

    // Get shape from the outcome that I want and the shape that the opponent plays.
    fn shape(&self) -> Shape {
        match (&self.opponent, &self.me.1) {
            (Shape::Rock, Outcome::Lose) => Shape::Scissors,
            (Shape::Rock, Outcome::Draw) => Shape::Rock,
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Paper, Outcome::Lose) => Shape::Rock,
            (Shape::Paper, Outcome::Draw) => Shape::Paper,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Scissors, Outcome::Lose) => Shape::Paper,
            (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
        }
    }
}

/// Error that occurred during parsing of the input
#[derive(Debug, Error)]
pub enum InputError {
    /// The input contains a line that has invalid number of parts.
    #[error("invalid number of parts on the input (expected {expected}, actual {actual})")]
    InvalidNumberOfParts { actual: usize, expected: usize },

    /// One of tha shapes in the input is invalid.
    #[error("got invalid shape: \"{0}\"")]
    InvalidShape(String),
}

/// Input for the solution that can be parsed from a string.
#[derive(PartialEq, Debug)]
pub struct Input(Vec<Round>);

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Result<Vec<Round>, InputError> = s
            .lines()
            .map(|val: &str| val.trim())
            .filter(|val| *val != "")
            .map(|val: &str| {
                let parts: Vec<&str> = val.split_whitespace().collect();
                if parts.len() != 2 {
                    return Err(InvalidNumberOfParts {
                        actual: parts.len(),
                        expected: 2,
                    });
                }

                let opponent = match parts[0] {
                    "A" => Shape::Rock,
                    "B" => Shape::Paper,
                    "C" => Shape::Scissors,
                    shape => return Err(InputError::InvalidShape(shape.to_owned())),
                };

                let me = match parts[1] {
                    "X" => (Shape::Rock, Outcome::Lose),
                    "Y" => (Shape::Paper, Outcome::Draw),
                    "Z" => (Shape::Scissors, Outcome::Win),
                    shape => return Err(InputError::InvalidShape(shape.to_owned())),
                };

                Ok(Round { opponent, me })
            })
            .collect();

        Ok(Input(result?))
    }
}

/// Solution for day 2.
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = i64;

    fn get_day(&self) -> u32 {
        2
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        input.0.iter().fold(0, |acc, round: &Round| {
            acc + round.me.0.score() + round.outcome().score()
        }) as i64
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        input.0.iter().fold(0, |acc, round: &Round| {
            acc + round.me.1.score() + round.shape().score()
        }) as i64
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::day_02::{Input, Outcome, Round, Shape, Solution};
    use crate::Solver;
    use std::str::FromStr;

    const INPUT: &str = r#"A Y
        B X
        C Z
        "#;

    #[test]
    fn parse_input() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!(
            input,
            Input(vec![
                Round {
                    opponent: Shape::Rock,
                    me: (Shape::Paper, Outcome::Draw),
                },
                Round {
                    opponent: Shape::Paper,
                    me: (Shape::Rock, Outcome::Lose),
                },
                Round {
                    opponent: Shape::Scissors,
                    me: (Shape::Scissors, Outcome::Win),
                },
            ])
        );
    }

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 15);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 12);
    }
}
