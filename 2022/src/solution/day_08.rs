//! Contains solution for day 8.

use crate::Solver;
use std::str::FromStr;
use thiserror::Error;

/// Error that occurred during parsing of the input.
#[derive(Debug, Error)]
pub enum InputError {
    /// One of the tree height in the map is not a valid digit.
    #[error("Given tree height is not a valid digit: {0}")]
    InvalidDigit(char),
}

/// Input for the solution that can be parsed from a string
#[derive(Debug, PartialEq)]
pub struct Input(Vec<Vec<u32>>);

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<Vec<u32>>, _> = s
            .lines()
            .map(|val| val.trim()) // Trim whitespace
            .filter(|val| *val != "") // Filter out empty lines
            .map(|line| {
                // Convert line to vector of numbers
                line.chars()
                    .map(|c| c.to_digit(10).ok_or(InputError::InvalidDigit(c)))
                    .collect()
            })
            .collect();

        Ok(Input(res?))
    }
}

/// Solution for day 8.
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = u64;

    fn get_day(&self) -> u32 {
        8
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        let height = input.0.len();
        let width = input.0[0].len();

        // Flags that correspond to weather the tree at that indices is visible.
        let mut visible = vec![vec![false; width]; height];

        // Go through all the rows in both directions to determine visibility.
        for row in 0..height {
            find_visible_row(row, &input.0, 0..width, &mut visible);
            find_visible_row(row, &input.0, (0..width).rev(), &mut visible);
        }

        // Go through all the columns in both directions to determine visibility.
        for column in 0..width {
            find_visible_column(column, &input.0, 0..height, &mut visible);
            find_visible_column(column, &input.0, (0..height).rev(), &mut visible);
        }

        // Count number of visible trees.
        let nr_visible: usize = visible
            .iter()
            .map(|row| row.iter().filter(|val| **val).count())
            .sum();
        nr_visible as Self::Output
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        let height = input.0.len();
        let width = input.0[0].len();

        let moves = vec![(-1, 0), (0, 1), (0, -1), (1, 0)];

        let mut max_vd = None;
        // We don't have to check the edges, since the viewing distance is 0.
        for row in 1..height - 1 {
            for column in 1..width - 1 {
                // Viewing distance for the current tree.
                let mut vd = 1;
                for (dx, dy) in moves.iter() {
                    // Get viewing distance for the given direction and tree.
                    vd *= viewing_distance(
                        row as i32,
                        column as i32,
                        *dx,
                        *dy,
                        &input.0,
                        width as i32,
                        height as i32,
                    );
                }

                // Check if the new viewing distance is large than the old one.
                match max_vd {
                    None => max_vd = Some(vd),
                    Some(val) if vd > val => max_vd = Some(vd),
                    _ => (),
                }
            }
        }

        max_vd.expect("If the input is valid, solution should exist") as u64
    }
}

// Finds all visible trees in a row going through the columns in the order given.
fn find_visible_row<'a>(
    row_index: usize,
    map: &Vec<Vec<u32>>,
    columns: impl Iterator<Item = usize>,
    visible: &mut Vec<Vec<bool>>,
) {
    let mut max = None;

    for column in columns {
        let element = map[row_index][column];
        set_visible(row_index, column, element, &mut max, visible);
    }
}

// Finds all visible trees in a column going throught the rows in the order given.
fn find_visible_column(
    column_index: usize,
    map: &Vec<Vec<u32>>,
    rows: impl Iterator<Item = usize>,
    visible: &mut Vec<Vec<bool>>,
) {
    let mut max = None;

    for row in rows {
        let element = map[row][column_index];
        set_visible(row, column_index, element, &mut max, visible);
    }
}

// Compares the tree at the given location with an existing maximum.
// If it is bigger, the tree at the given location is visible.
// Function modifies max and visible when a new visible tree is found.
fn set_visible(
    row: usize,
    column: usize,
    element: u32,
    max: &mut Option<u32>,
    visible: &mut Vec<Vec<bool>>,
) {
    match max {
        None => {
            *max = Some(element);
            visible[row][column] = true;
        }
        Some(current_max) if element > *current_max => {
            *max = Some(element);
            visible[row][column] = true;
        }
        _ => (),
    }
}

// Calculates the viewing distance of a tree a the given location, by moving in the given direction.
fn viewing_distance(
    mut row: i32,
    mut column: i32,
    dx: i32,
    dy: i32,
    map: &Vec<Vec<u32>>,
    width: i32,
    height: i32,
) -> u32 {
    let element = map[row as usize][column as usize];
    let mut distance = 0;

    // Move one step at first.
    row += dy;
    column += dx;
    while row >= 0 && row < height && column >= 0 && column < width {
        distance += 1;

        // If we find a bigger tree, we stop with calculation of the viewing distance.
        if map[row as usize][column as usize] >= element {
            break;
        }

        row += dy;
        column += dx;
    }

    distance
}

#[cfg(test)]
mod tests {
    use crate::solution::day_08::{Input, Solution};
    use crate::Solver;
    use std::str::FromStr;

    const INPUT: &str = r#"
        30373
        25512
        65332
        33549
        35390
        "#;

    #[test]
    fn parse_input() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!(
            input,
            Input(vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ])
        );
    }

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 21);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 8);
    }
}
