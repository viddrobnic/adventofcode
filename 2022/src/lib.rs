//! Crate containing the solution to the [advent of code 2022](https://adventofcode.com/2022).
//!
//! The goal of this project is **not** to solve problems the fastest and climb the leaderboard,
//! but to practice writing clean code in Rust. Therefore the crate contains a lot more
//! abstraction, comments and tests than needed to solve the advent of code.

use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::time::{Duration, Instant};

use thiserror::Error;

pub mod solution;

/// Error that occurred during solving of the day's problem.
#[derive(Debug, Error)]
pub enum SolverError {
    /// Could not read the input from the input file.
    #[error("could not read the input")]
    ReadingInputFailed {
        #[from]
        source: std::io::Error,
    },

    /// Input in the input file is invalid and could not be parsed.
    #[error("could not parse the input")]
    InvalidInput,
}

/// Defines a type that can solve a problem for a single day.
///
/// The solver type needs to implement solving of part one and part two. After that `solve`
/// can be called to solve the problem.
///
/// Each solver defines its own input and output. This way parsing the input string can be lifted
/// out of the code that actually solves the problem.
///
/// Example of a solution for a day:
/// ```ignore
/// use std::str::FromStr;
///
/// struct Input(Vec<i32>);
///
/// impl FromStr for Input {type Err = ();
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         // Parsing input from string and validating it ...
///         todo!()
///     }
/// }
///
/// struct Solution;
///
/// impl Solver for Solution {
///     type Input = Input;
///     type Output = i32;
///
///     fn part_one(input: &Self::Input) -> Self::Output {
///         // Solution to part one
///         todo!()
///     }
///     
///     fn part_two(input: &Self::Input) -> Self::Output {
///         // Solution to part two
///         todo!()
///     }
/// }
/// ```
pub trait Solver {
    /// Input that is given to the functions that solve the problem.
    type Input: FromStr;

    /// Output that is returned from the function that solves the problem.
    type Output: Display;

    /// Returns the day which this solver can solve.
    fn get_day(&self) -> u32;

    /// Solution for part one of the problem.
    fn part_one(&self, input: &Self::Input) -> Self::Output;

    /// Solution for part two of the problem.
    fn part_two(&self, input: &Self::Input) -> Self::Output;

    /// Solves both parts of the problem and returns the solutions.
    ///
    /// The function first reads and parses the input from the file located at `input_path`.
    /// Returns a pair of part one and part two solutions.
    fn solve(&self, input_path: &str) -> Result<(Self::Output, Self::Output), SolverError> {
        let mut file = File::open(input_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let input = Self::Input::from_str(&contents).map_err(|_| SolverError::InvalidInput)?;

        let part_one = self.part_one(&input);
        let part_two = self.part_two(&input);

        Ok((part_one, part_two))
    }
}

/// Prints the solution that the solver solves to the stdout.
///
/// The function reads the input from the `inputs` directory.
/// It expects the inputs file to be named as `day_XX.txt`.
/// Function also measures the time it took to solve the problem and returns the duration.
pub fn print_solution(solver: impl Solver) -> Duration {
    let day = solver.get_day();
    let input_path = format!("inputs/day_{:0>2}.txt", day);

    let now = Instant::now();
    let solution = solver.solve(&input_path);
    let elapsed_time = now.elapsed();

    match solution {
        Ok((part_one, part_two)) => {
            println!(
                "Day {:0>2} [{}ms]:\n\tPart one: {}\n\tPart two: {}",
                day,
                elapsed_time.as_millis(),
                part_one,
                part_two
            );
        }
        Err(err) => {
            println!("Day {:0>2}:\n\tFailed to solve: {}", day, err);
        }
    }

    elapsed_time
}
