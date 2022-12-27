//! Contains solution for day 25.

use crate::Solver;
use std::str::FromStr;
use thiserror::Error;

/// Error that occurred during the parsing of the input.
#[derive(Debug, PartialEq, Error)]
pub enum InputError {
    #[error("Invalid digit: {0}")]
    InvalidDigit(char),
}

/// Converts a number from SNAFU to normal integer.
fn from_snafu(n: &str) -> Result<i64, InputError> {
    let base: i64 = 5;

    let mut res = 0;
    // Iterate through digits and add their values the to result.
    for (pow, digit) in n.chars().rev().enumerate() {
        // Get digit value.
        let digit = match digit {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => return Err(InputError::InvalidDigit(digit)),
        };

        // Add to result digit * 5^pow
        res += base.pow(pow as u32) * digit;
    }

    Ok(res)
}

/// Converts a number from integer to SNAFU.
fn to_snafu(mut n: u64) -> String {
    let mut res = String::new();

    while n != 0 {
        // Get the last digit of the number and update the number.
        match n % 5 {
            0 => res.insert(0, '0'),
            1 => {
                res.insert(0, '1');
                n -= 1;
            }
            2 => {
                res.insert(0, '2');
                n -= 2;
            }
            3 => {
                res.insert(0, '=');
                n += 2;
            }
            4 => {
                res.insert(0, '-');
                n += 1
            }
            _ => unreachable!(),
        }

        // Divide the number.
        n /= 5;
    }

    res
}

/// Input for the solution that can be parsed from a string.
pub struct Input(Vec<i64>);

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<_>, _> = s
            .lines()
            .map(|line| line.trim())
            .filter(|line| *line != "")
            .map(|line| from_snafu(line))
            .collect();

        Ok(Input(res?))
    }
}

/// Solution for day 25
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = String;

    fn get_day(&self) -> u32 {
        25
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        let sum: i64 = input.0.iter().sum();
        to_snafu(sum as u64)
    }

    fn part_two(&self, _: &Self::Input) -> Self::Output {
        "Nothing to do here :)".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::day_25::{Input, Solution};
    use crate::Solver;
    use std::str::FromStr;

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, "2=-1=0");
    }

    #[test]
    fn from_snafu() {
        assert_eq!(super::from_snafu("1"), Ok(1));
        assert_eq!(super::from_snafu("2"), Ok(2));
        assert_eq!(super::from_snafu("1="), Ok(3));
        assert_eq!(super::from_snafu("1-"), Ok(4));
        assert_eq!(super::from_snafu("10"), Ok(5));
        assert_eq!(super::from_snafu("11"), Ok(6));
        assert_eq!(super::from_snafu("12"), Ok(7));
        assert_eq!(super::from_snafu("2="), Ok(8));
        assert_eq!(super::from_snafu("2-"), Ok(9));
        assert_eq!(super::from_snafu("20"), Ok(10));
        assert_eq!(super::from_snafu("1=0"), Ok(15));
        assert_eq!(super::from_snafu("1-0"), Ok(20));
        assert_eq!(super::from_snafu("1=11-2"), Ok(2022));
        assert_eq!(super::from_snafu("1-0---0"), Ok(12345));
        assert_eq!(super::from_snafu("1121-1110-1=0"), Ok(314159265));
    }

    #[test]
    fn to_snafu() {
        assert_eq!(super::to_snafu(1), "1");
        assert_eq!(super::to_snafu(2), "2");
        assert_eq!(super::to_snafu(3), "1=");
        assert_eq!(super::to_snafu(4), "1-");
        assert_eq!(super::to_snafu(5), "10");
        assert_eq!(super::to_snafu(6), "11");
        assert_eq!(super::to_snafu(7), "12");
        assert_eq!(super::to_snafu(8), "2=");
        assert_eq!(super::to_snafu(9), "2-");
        assert_eq!(super::to_snafu(10), "20");
        assert_eq!(super::to_snafu(15), "1=0");
        assert_eq!(super::to_snafu(20), "1-0");
        assert_eq!(super::to_snafu(2022), "1=11-2");
        assert_eq!(super::to_snafu(12345), "1-0---0");
        assert_eq!(super::to_snafu(314159265), "1121-1110-1=0");
    }

    const INPUT: &str = r#"
        1=-0-2
        12111
        2=0=
        21
        2=01
        111
        20012
        112
        1=-1=
        1-12
        12
        1=
        122
        "#;
}
