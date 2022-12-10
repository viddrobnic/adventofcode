//! Contains solution for day 9.

use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::Solver;

/// Error that occurred during parsing of the input.
#[derive(Debug, Error)]
pub enum InputError {
    /// Input contains a line that has invalid number of parts.
    #[error("Invalid number of parts in line \"{line}\". Expected {expected}, actual {actual}.")]
    InvalidNumberOfParts {
        line: String,
        actual: usize,
        expected: usize,
    },

    /// Input contains a line with invalid number of steps.
    #[error("Invalid number of steps given: {source}")]
    InvalidNumberOfSteps {
        #[from]
        source: ParseIntError,
    },

    /// Input contains a line with invalid direction.
    #[error("Invalid direction given: {0}")]
    InvalidDirection(String),
}

/// Direction is a direction in which the rope moves.
#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    /// Returns the sign of the move on the x axis for the direction.
    fn dx(&self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            Direction::Up | Direction::Down => 0,
        }
    }

    /// Returns the sign of the move on the y axis for the direction.
    fn dy(&self) -> i32 {
        match self {
            Direction::Left | Direction::Right => 0,
            Direction::Up => 1,
            Direction::Down => -1,
        }
    }
}

impl FromStr for Direction {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err(InputError::InvalidDirection(s.to_string())),
        }
    }
}

/// Move represents a move of the rope.
#[derive(Debug, PartialEq)]
struct Move {
    direction: Direction,
    steps: i32,
}

impl FromStr for Move {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(InputError::InvalidNumberOfParts {
                line: s.to_string(),
                actual: parts.len(),
                expected: 2,
            });
        }

        Ok(Move {
            direction: parts[0].parse()?,
            steps: parts[1].parse()?,
        })
    }
}

/// Point in a 2D space.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Self {
        Point { x: 0, y: 0 }
    }

    /// Returns weather the point is touching the other point.
    fn is_touching(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

/// Input for the solution that can be parsed from a string.
#[derive(Debug, PartialEq)]
pub struct Input(Vec<Move>);

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<Move>, _> = s
            .lines()
            .map(|val| val.trim()) // Trim whitespace
            .filter(|val| *val != "") // Remove empty lines
            .map(|line| line.parse()) // Parse lines
            .collect();
        Ok(Input(res?))
    }
}

/// Solution for day 9.
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = i32;

    fn get_day(&self) -> u32 {
        9
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        simulate_rope(2, &input.0)
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        simulate_rope(10, &input.0)
    }
}

// Simulates rope with a given number of knots.
fn simulate_rope(number_of_knots: usize, moves: &Vec<Move>) -> i32 {
    if number_of_knots == 0 {
        panic!("Rope needs some knots!");
    }

    // Initialize knots on the rope.
    let mut rope = vec![Point::new(); number_of_knots];
    let mut tail_visited: HashSet<Point> = HashSet::new();
    tail_visited.insert(*rope.last().unwrap());

    for mv in moves.iter() {
        for _ in 0..mv.steps {
            // Move the head of the rope.
            rope[0].x += mv.direction.dx();
            rope[0].y += mv.direction.dy();

            for knot in 1..rope.len() {
                // Once the rope is not touching, nothing will change in the future
                // and we can break.
                if rope[knot].is_touching(&rope[knot - 1]) {
                    break;
                }

                // Move the next knot according to the previous one.
                let x_diff = rope[knot - 1].x - rope[knot].x;
                rope[knot].x += x_diff.signum();

                let y_diff = rope[knot - 1].y - rope[knot].y;
                rope[knot].y += y_diff.signum();
            }

            tail_visited.insert(*rope.last().unwrap());
        }
    }

    tail_visited.len() as i32
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::solution::day_09::{Direction, Input, Move, Solution};
    use crate::Solver;

    const INPUT: &str = r#"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
        "#;

    #[test]
    fn parse_input() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!(
            input,
            Input(vec![
                Move {
                    direction: Direction::Right,
                    steps: 4,
                },
                Move {
                    direction: Direction::Up,
                    steps: 4,
                },
                Move {
                    direction: Direction::Left,
                    steps: 3,
                },
                Move {
                    direction: Direction::Down,
                    steps: 1,
                },
                Move {
                    direction: Direction::Right,
                    steps: 4,
                },
                Move {
                    direction: Direction::Down,
                    steps: 1,
                },
                Move {
                    direction: Direction::Left,
                    steps: 5,
                },
                Move {
                    direction: Direction::Right,
                    steps: 2,
                },
            ])
        );
    }

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 13);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 1);
    }

    #[test]
    fn part_two_alternative() {
        let input = Input::from_str(
            r#"
                R 5
                U 8
                L 8
                D 3
                R 17
                D 10
                L 25
                U 20
                "#,
        )
        .unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 36);
    }
}
