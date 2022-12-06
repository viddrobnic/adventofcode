//! Crate containing the solution to the [advent of code 2022](https://adventofcode.com/2022).
//!
//! The goal of this project is **not** to solve problems the fastest and climb the leaderboard,
//! but to practice writing clean code in Rust. Therefore the crate contains a lot more
//! abstraction, comments and tests than needed to solve the advent of code.

extern crate core;

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

/// Output of the solution.
///
/// Includes solutions of the both parts of the problem
/// and duration that each part took to solve.  
pub struct SolverOutput<T> {
    part_one: T,
    part_two: T,
    part_one_duration: Duration,
    part_two_duration: Duration,
    input_parsing_duration: Duration,
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
    /// Durations of solving the problem do not include reading the input from the file.
    fn solve(&self, input_path: &str) -> Result<SolverOutput<Self::Output>, SolverError> {
        let mut file = File::open(input_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let (input, input_parsing_duration) =
            time_fn(|| Self::Input::from_str(&contents).map_err(|_| SolverError::InvalidInput));
        let input = input?;

        let (part_one, part_one_duration) = time_fn(|| self.part_one(&input));
        let (part_two, part_two_duration) = time_fn(|| self.part_two(&input));

        Ok(SolverOutput {
            part_one,
            part_two,
            part_one_duration,
            part_two_duration,
            input_parsing_duration,
        })
    }
}

/// A solver that can print the solution to the stdout.
///
/// Type contains a blanket implementation for the `Solver`.
/// This way we remove associated types from the solver
/// and can have a function with signature
/// `fn get_solver(day: u8) -> Box<dyn PrintSolver>`
pub trait PrintSolver {
    /// Solve the problem and write the solution to the stdout.
    ///
    /// Returns the time needed to solve the problem.
    fn print_solution(&self) -> Duration;
}

/// Blanket implementation of the `PrintSolver` for `Solver`.
///
/// It reads the input from the `inputs` directory.
/// It expects the inputs file to be named as `day_XX.txt`.
impl<T: Solver> PrintSolver for T {
    fn print_solution(&self) -> Duration {
        let day = self.get_day();
        let input_path = format!("inputs/day_{:0>2}.txt", day);

        let mut elapsed = Duration::from_secs(0);

        match self.solve(&input_path) {
            Ok(solution) => {
                elapsed = solution.input_parsing_duration
                    + solution.part_one_duration
                    + solution.part_two_duration;
                println!(
                    "Day {:0>2} [{:.2}ms]:\n\tInput parsing: {:.2}ms\n\tPart one [{:.2}ms]: {}\n\tPart two [{:.2}ms]: {}",
                    day,
                    duration_to_millis(&elapsed),
                    duration_to_millis(&solution.input_parsing_duration),
                    duration_to_millis(&solution.part_one_duration),
                    solution.part_one,
                    duration_to_millis(&solution.part_two_duration),
                    solution.part_two
                );
            }
            Err(err) => {
                println!("Day {:0>2}:\n\tFailed to solve: {}", day, err);
            }
        }

        elapsed
    }
}

// Helper function that times how long the function `func` ran.
fn time_fn<F, T>(func: F) -> (T, Duration)
where
    F: Fn() -> T,
{
    let now = Instant::now();
    let out = func();
    let duration = now.elapsed();

    (out, duration)
}

// Helper function that returns a float representation of duration in millis
// with additional decimal places
fn duration_to_millis(dur: &Duration) -> f64 {
    dur.as_micros() as f64 / 1000.0
}
