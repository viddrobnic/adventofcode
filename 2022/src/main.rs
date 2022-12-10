extern crate core;

use std::time::Duration;

use clap::{Parser, Subcommand};

use advent_of_code_2022::{solution::*, PrintSolver};

#[derive(Parser, Debug)]
#[command(name = "AoC solution")]
#[command(author = "Vid Drobnič <vid@zerodays.dev>")]
#[command(about="Clean code solutions for advent of code 2022", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Select specific days to solve
    Days { days: Vec<u8> },
    /// Solve all days
    All,
}

fn main() {
    let cli = Cli::parse();
    let days = match cli.command {
        Command::All => (1..=25).collect(),
        Command::Days { days } => days,
    };

    let mut total_elapsed = Duration::new(0, 0);
    for day in days {
        match get_solver(day) {
            Ok(Some(solver)) => {
                let elapsed = solver.print_solution();
                total_elapsed += elapsed;
            }
            Ok(None) => println!("Day {:0>2}: Not implemented!", day),
            Err(InvalidDayError(day)) => {
                println!("Day {} is not a valid day!", day);
            }
        }
    }

    println!(
        "\n------------------------\nTotal time elapsed: {:.2}ms",
        total_elapsed.as_micros() as f64 / 1000.0
    );
}

/// Represents an invalid day.
struct InvalidDayError(u8);

// Gets the solver for the specified day.
fn get_solver(day: u8) -> Result<Option<Box<dyn PrintSolver>>, InvalidDayError> {
    match day {
        1 => Ok(Some(Box::new(day_01::Solution))),
        2 => Ok(Some(Box::new(day_02::Solution))),
        3 => Ok(Some(Box::new(day_03::Solution))),
        4 => Ok(Some(Box::new(day_04::Solution))),
        5 => Ok(Some(Box::new(day_05::Solution))),
        6 => Ok(Some(Box::new(day_06::Solution))),
        7 => Ok(Some(Box::new(day_07::Solution))),
        8 => Ok(Some(Box::new(day_08::Solution))),
        9 => Ok(Some(Box::new(day_09::Solution))),
        10 => Ok(Some(Box::new(day_10::Solution))),
        11 => Ok(None),
        12 => Ok(None),
        13 => Ok(None),
        14 => Ok(None),
        15 => Ok(None),
        16 => Ok(None),
        17 => Ok(None),
        18 => Ok(None),
        19 => Ok(None),
        20 => Ok(None),
        21 => Ok(None),
        22 => Ok(None),
        23 => Ok(None),
        24 => Ok(None),
        25 => Ok(None),
        _ => Err(InvalidDayError(day)),
    }
}
