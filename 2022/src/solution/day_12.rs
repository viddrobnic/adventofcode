//! Contains solution for day 12.
use std::collections::VecDeque;
use std::str::FromStr;

use thiserror::Error;

use crate::Solver;

/// Error that occurred during parsing of the input.
#[derive(Debug, Error)]
pub enum InputError {
    /// Input does not contains a starting point.
    #[error("Could not find starting position in the input.")]
    NoStart,

    /// Input does not contains an ending point.
    #[error("Could not find ending position in the input.")]
    NoEnd,
}

/// Point on a map
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    /// Creates a new zero point.
    pub fn new() -> Self {
        Point { x: 0, y: 0 }
    }
}

/// Input for the solution that can be parsed from a string.
#[derive(Debug, PartialEq)]
pub struct Input {
    map: Vec<Vec<u32>>,
    start: Point,
    end: Point,
}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First parse the map into a 2D vector of elevations.
        let map: Vec<Vec<u32>> = s
            .lines()
            .map(|line| line.trim()) // Trim whitespace
            .filter(|line| *line != "") // Skip empty lines
            .map(|line| {
                // Map characters in the line to elevations
                line.chars()
                    .map(|c| match c {
                        'S' => 0,
                        'E' => 25,
                        c => c as u32 - 'a' as u32,
                    })
                    .collect()
            })
            .collect();

        // Secondly find the starting and ending positions.
        let mut start = None;
        let mut end = None;

        for (y, line) in s
            .lines()
            .map(|line| line.trim()) // Trim whitespace
            .filter(|line| *line != "") // Skip empty lines
            .enumerate()
        {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Some(Point { x, y });
                }

                if c == 'E' {
                    end = Some(Point { x, y });
                }
            }
        }

        // Construct the input
        Ok(Input {
            map,
            start: start.ok_or(InputError::NoStart)?,
            end: end.ok_or(InputError::NoEnd)?,
        })
    }
}

/// Solution for day 12.
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = u32;

    fn get_day(&self) -> u32 {
        12
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        shortest_path(
            input.start,                   // We start at the starting point.
            |point, _| point == input.end, // End if we reach the ending point
            // Limit to which points we can move. We can increase elevation by maximum of 1.
            |new_elevation, old_elevation| new_elevation <= old_elevation + 1,
            &input.map,
        )
        .expect("Valid input should have a solution")
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        shortest_path(
            input.end, // Start at the ending
            // Finish when we reach the first point with elevation 0
            |_, elevation| elevation == 0,
            // Limit to which points we can move. We can decrease elevation by maximum of 1.
            // This is exactly reverse than in part one, because we are going in the reverse direction.
            |new_elevation, old_elevation| new_elevation >= old_elevation - 1,
            &input.map,
        )
        .expect("Valid input should have a solution")
    }
}

/// Find the shortest path between two points using BFS.
///
/// Starting point a fixed given point `point`. Ending condition can be dynamically controlled
/// using the `end_on` predicate. It takes the current point and its elevation and returns
/// `true` if the current point is the ending position, and `false` otherwise.
///
/// Points to which we can move are limited with the `move_on` predicate. It takes a pair of
/// new and current elevation and returns `true` if we can move from current elevation to the new
/// one, and `false` otherwise.
fn shortest_path<E, M>(start: Point, end_on: E, move_on: M, map: &Vec<Vec<u32>>) -> Option<u32>
where
    E: Fn(Point, u32) -> bool, // Takes the current point and the elevation of it
    M: Fn(u32, u32) -> bool,   // Takes the new elevation and the current elevation
{
    // Get the width and height of the map.
    let width = map[0].len();
    let height = map.len();

    // Construct the vector of visited positions.
    let mut visited = vec![vec![false; width]; height];
    visited[start.y][start.x] = true;
    // Construct the deque for bfs.
    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((point, distance)) = queue.pop_front() {
        // Get elevation of the current point.
        let elevation = map[point.y][point.x];

        // Check if the current point is the ending.
        if end_on(point, elevation) {
            return Some(distance);
        }

        // Check its neighbours.
        for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let new_x = point.x as i32 + dx;
            let new_y = point.y as i32 + dy;

            // Check if the neighbour is still on the map.
            if new_x < 0 || new_x >= (width as i32) || new_y < 0 || new_y >= (height as i32) {
                continue;
            }

            let new_point = Point {
                x: new_x as usize,
                y: new_y as usize,
            };

            // Check if we can move to the neighbour and if the neighbour has not been visited yet.
            if move_on(map[new_point.y][new_point.x], elevation)
                && !visited[new_point.y][new_point.x]
            {
                // Add the neighbour to the queue.
                queue.push_back((new_point, distance + 1));
                visited[new_point.y][new_point.x] = true;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::solution::day_12::{Input, Point, Solution};
    use crate::Solver;

    const INPUT: &str = r#"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
        "#;

    #[test]
    fn parse_input() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!(
            input,
            Input {
                map: vec![
                    vec![0, 0, 1, 16, 15, 14, 13, 12],
                    vec![0, 1, 2, 17, 24, 23, 23, 11],
                    vec![0, 2, 2, 18, 25, 25, 23, 10],
                    vec![0, 2, 2, 19, 20, 21, 22, 9],
                    vec![0, 1, 3, 4, 5, 6, 7, 8],
                ],
                start: Point { x: 0, y: 0 },
                end: Point { x: 5, y: 2 },
            }
        );
    }

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 31);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 29);
    }
}
