//! Contains solution for day 18.

use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::Solver;

/// Error tha occurred during parsing of the input.
#[derive(Debug, Error)]
pub enum InputError {
    /// One of the numbers in the input is invalid.
    #[error("Got invalid number: {source}")]
    InvalidNumber {
        #[from]
        source: ParseIntError,
    },

    /// One of the lines contains invalid number of parts.
    #[error("A line contains invalid number of parts. Expected: {expected}, actual: {actual}.")]
    InvalidNumberOfParts { actual: usize, expected: usize },
}

/// Represents a point in a 3D space.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Point {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the line by comma and parse the parts
        let parts: Result<Vec<_>, _> = s.split(",").map(|part| part.parse()).collect();
        let parts = parts?;

        // Check that the number of parts is correct.
        if parts.len() != 3 {
            return Err(InputError::InvalidNumberOfParts {
                actual: parts.len(),
                expected: 3,
            });
        }

        // Construct the points.
        Ok(Point {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        })
    }
}

/// Input for the solution that can be parsed from a string.
#[derive(Debug, PartialEq)]
pub struct Input(Vec<Point>);

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<Point>, _> = s
            .lines()
            .map(|line| line.trim()) // Trim the whitespace
            .filter(|line| *line != "") // Filter out empty lines
            .map(|line| line.parse()) // Parse into points
            .collect();

        Ok(Input(res?))
    }
}

/// Valid moves on the map
const MOVES: [(i32, i32, i32); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

/// Limit on the map size.
const MAP_SIZE: usize = 20;

/// Helper function for solving the problem.
fn solve(points: &Vec<Point>, valid: &Vec<Vec<Vec<bool>>>) -> u32 {
    // Generate a map of where the points are, limited by `map_size`.
    let mut map = vec![vec![vec![false; MAP_SIZE]; MAP_SIZE]; MAP_SIZE];
    for point in points.iter() {
        map[point.z as usize][point.y as usize][point.x as usize] = true;
    }

    // Initialize the result.
    let mut res = 0;

    // For each cube check if the neighbour is empty.
    for point in points.iter() {
        for (dx, dy, dz) in MOVES {
            // Calculate the coordinates of the neighbour.
            let x = point.x + dx;
            let y = point.y + dy;
            let z = point.z + dz;

            // Check if the neighbour is on the map. If it isn't, update the result variable
            // and continue on to the next one.
            let map_size = MAP_SIZE as i32;
            if x < 0 || x >= map_size || y < 0 || y >= map_size || z < 0 || z >= map_size {
                res += 1;
                continue;
            }

            // Check if the neighbour is empty and that it is a valid empty coordinate.
            // If both is true, update the result.
            if !map[z as usize][y as usize][x as usize] && valid[z as usize][y as usize][x as usize]
            {
                res += 1;
            }
        }
    }

    res
}

/// Solution for day 18.
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = u32;

    fn get_day(&self) -> u32 {
        18
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        // Solve the case where all the empty spaces are valid.
        let map_size = 20;
        let valid = vec![vec![vec![true; map_size]; map_size]; map_size];
        solve(&input.0, &valid)
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        // Solve the case where only exterior empty spaces are valid. To determine
        // exterior empty spaces, start at an arbitrary exterior empty space and do BFS
        // to all the ones we can reach. If the limit is large enough and the droplet
        // is not blocking the whole border outside the limit, we will find all exterior spaces.
        // If your input is different from mine, and this constants don't work, try moving the whole
        // droplet away from the border and making the MAP_SIZE bigger.

        // Initialize the array of visited locations.
        let mut visited = vec![vec![vec![false; MAP_SIZE]; MAP_SIZE]; MAP_SIZE];

        // Populate the map of where the droplet is.
        let mut map = vec![vec![vec![false; MAP_SIZE]; MAP_SIZE]; MAP_SIZE];
        for point in input.0.iter() {
            map[point.z as usize][point.y as usize][point.x as usize] = true;
        }

        // BFS on exterior spaces. Starting at (0, 0, 0) seems to work.
        let start = Point { x: 0, y: 0, z: 0 };
        let mut queue = VecDeque::new();
        queue.push_back(start);
        visited[start.z as usize][start.y as usize][start.x as usize] = true;
        while !queue.is_empty() {
            // Get next point in the queue
            let point = queue.pop_front().expect("Queue isn't empty");

            // Iterate through the neighbours.
            for (dx, dy, dz) in MOVES {
                // Calculate the coordinates of the neighbour.
                let x = point.x + dx;
                let y = point.y + dy;
                let z = point.z + dz;

                // If the neighbour is outside the map bounds, do nothing.
                let map_size = MAP_SIZE as i32;
                if x < 0 || x >= map_size || y < 0 || y >= map_size || z < 0 || z >= map_size {
                    continue;
                }

                // If we have already visited it, do nothing.
                if visited[z as usize][y as usize][x as usize] {
                    continue;
                }

                // If it is populated wih obsidian, do nothing.
                if map[z as usize][y as usize][x as usize] {
                    continue;
                }

                // Set the neighbour as visited, and add it to the queue.
                visited[z as usize][y as usize][x as usize] = true;
                queue.push_back(Point { x, y, z })
            }
        }

        // Get the final result.
        solve(&input.0, &visited)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::solution::day_18::{Input, Point, Solution};
    use crate::Solver;

    #[test]
    fn parse_input() {
        let input = Input::from_str(
            "2,2,2
                1,2,2
                3,2,2",
        )
        .unwrap();
        assert_eq!(
            input,
            Input(vec![
                Point { x: 2, y: 2, z: 2 },
                Point { x: 1, y: 2, z: 2 },
                Point { x: 3, y: 2, z: 2 }
            ])
        );
    }

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 64);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 58);
    }

    const INPUT: &str = r#"
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
        "#;
}
