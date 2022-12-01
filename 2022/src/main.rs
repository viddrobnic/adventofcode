extern crate core;

use std::time::Duration;

use clap::{Parser, Subcommand};

use advent_of_code_2022::{print_solution, solution::*, Solver};

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
                let elapsed = print_solution(solver);
                total_elapsed += elapsed;
            }
            Ok(None) => println!("Day {:0>2}: Not implemented!", day),
            Err(InvalidDayError(day)) => {
                println!("Day {} is not a valid day!", day);
            }
        }
    }

    println!(
        "\n------------------------\nTotal time elapsed: {}ms",
        total_elapsed.as_millis()
    );
}

/// Represents an invalid day.
struct InvalidDayError(u8);

// Gets the solver for the specified day.
fn get_solver(day: u8) -> Result<Option<impl Solver>, InvalidDayError> {
    match day {
        1 => Ok(Some(day_01::Solution)),
        2 => Ok(None),
        3 => Ok(None),
        4 => Ok(None),
        5 => Ok(None),
        6 => Ok(None),
        7 => Ok(None),
        8 => Ok(None),
        9 => Ok(None),
        10 => Ok(None),
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
