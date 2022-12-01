//! Contains the solution for day 1.

use std::num::ParseIntError;
use std::str::FromStr;

use crate::Solver;

/// Input for the solution that can be parsed from a string.
#[derive(PartialEq, Debug)]
pub struct Elves(Vec<i64>);

impl FromStr for Elves {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split string into lines and trim the whitespaces.
        let mut lines = s.lines().map(|val: &str| val.trim());

        // Group lines by elves with the help of fold.
        // Accumulator is pair of:
        // - vector of sums of calories for each elf
        // - sum of calories for the current elf
        let res: Result<(Vec<i64>, i64), Self::Err> =
            lines.try_fold((Vec::with_capacity(0), 0), |(mut result, elf), elt| {
                if elt == "" {
                    // If a line is empty, append current elf to elves
                    // and set the current elf sum to 0.
                    result.push(elf);
                    Ok((result, 0))
                } else {
                    // If the line is not empty, convert the string to int
                    // and add it to the current sum elf.
                    let val: i64 = elt.parse()?;
                    Ok((result, elf + val))
                }
            });

        // Convert result into elves struct.
        let (elves, _) = res?;
        Ok(Elves(elves))
    }
}

/// Solution for day 1.
pub struct Solution;

impl Solver for Solution {
    type Input = Elves;
    type Output = i64;

    fn get_day(&self) -> u32 {
        1
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        // find the maximum of the elves.
        *input
            .0
            .iter()
            .max()
            .expect("Invalid input for day 1 part 1!")
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        // Find the biggest three elves.
        let mut input_copy = input.0.clone();
        input_copy.sort_unstable();

        let len = input_copy.len();
        let mut result = 0;
        for index in ((len - 3)..len).rev() {
            result += input_copy[index];
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::day_01::{Elves, Solution};
    use crate::Solver;
    use std::str::FromStr;

    const INPUT: &str = r#"1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000
        "#;

    #[test]
    fn parse_input() {
        let elves = Elves::from_str(INPUT).unwrap();
        assert_eq!(elves, Elves(vec![6000, 4000, 11000, 24000, 10000]));
    }

    #[test]
    fn part_one() {
        let solver = Solution;
        let elves = Elves::from_str(INPUT).unwrap();

        let solution = solver.part_one(&elves);
        assert_eq!(solution, 24000);
    }

    #[test]
    fn part_two() {
        let solver = Solution;
        let elves = Elves::from_str(INPUT).unwrap();

        let solution = solver.part_two(&elves);
        assert_eq!(solution, 45000);
    }
}
